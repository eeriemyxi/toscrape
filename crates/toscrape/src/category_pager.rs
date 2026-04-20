use std::collections::VecDeque;

use crate::{enums::Stock, selectors};

use super::{
    CURRENCY_SYMBOL, Rating,
    book_info::BookCard,
    errors::ScraperError,
    fetching::{fetch_page, get_client},
};
use scraper::{ElementRef, Html};
use url::Url;

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
enum MessageType {
    NextPage { page: u32, cards: Vec<BookCard> },
    NoMorePages { last_page: u32 },
}

/// Paginator for book categories.
pub struct BookCategoryPager {
    /// The link to the category.
    url: Url,
    /// Buffer for the BookCards yielded so far.
    buffer: VecDeque<BookCard>,
    /// Stores a dynamic array of BookCard mapped to its page index
    pages: HashMap<u32, Vec<BookCard>>,
    /// The client that will be used by the threads to fetch the pages
    client: Arc<Client>,
    /// The MPSC Sender
    tx: mpsc::Sender<Result<MessageType, ScraperError>>,
    /// The MPSC Receiver
    rx: mpsc::Receiver<Result<MessageType, ScraperError>>,
    /// The last page that will eventually be fetched (by detecting 404)
    last_page: Option<u32>,
    /// How many threads to use for fetching
    thread_ahead: u16,
    /// How many handles are active at any given moment (should be eq to thread_ahead most of the time)
    active_handles: u16,
    /// Obvious by name
    next_page_to_yield: u32,
    /// Obvious by name
    next_page_to_return: u32,
}

impl BookCategoryPager {
    pub fn new(url: &str) -> Result<Self, ScraperError> {
        let (tx, rx) = mpsc::channel();
        Ok(Self {
            url: Url::parse(url).map_err(|e| ScraperError::InvalidURL {
                url: url.to_string(),
                second: None,
                source: Box::new(e),
            })?,
            buffer: VecDeque::new(),
            thread_ahead: 0,
            pages: HashMap::new(),
            client: get_client(),
            tx,
            rx,
            last_page: None,
            next_page_to_return: 0,
            next_page_to_yield: 0,
            active_handles: 0,
        })
    }

    /// Set the active page. Could be used to paginate from a certain number.
    pub fn page(mut self, page: u32) -> Self {
        self.next_page_to_yield = page;
        self.next_page_to_return = page;
        self
    }

    /// Set how many threads to use during pagination. They'll be used to fetch pages in parallel.
    pub fn thread_ahead(mut self, count: u16) -> Self {
        self.thread_ahead = count;
        self
    }

    fn parse_page(html: String, page_url: &Url) -> Result<Vec<BookCard>, ScraperError> {
        Html::parse_document(&html)
            .select(selectors::card())
            .map(|el| Self::parse_card(el, page_url))
            .collect()
    }

    fn parse_card(el: ElementRef, page_url: &Url) -> Result<BookCard, ScraperError> {
        let thumbnail_el = el
            .select(selectors::card_thumbnail())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find the thumbnail element".to_string(),
            })?;

        let thumbnail_src = thumbnail_el
            .attr("src")
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find the thumbnail's `src` attribute".to_string(),
            })?
            .trim();

        let thumbnail_link = page_url
            .join(thumbnail_src)
            .map_err(|e| ScraperError::InvalidURL {
                url: page_url.to_string(),
                second: Some(thumbnail_src.to_string()),
                source: Box::new(e),
            })?
            .to_string();

        let title = thumbnail_el
            .attr("alt")
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find the thumbnail's `alt` attribute".to_string(),
            })?
            .trim()
            .to_string();

        let card_link = el
            .select(selectors::card_link())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldnt't find the card link element".to_string(),
            })?
            .attr("href")
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "coudn't find `href` attribute in card element".to_string(),
            })?
            .trim();

        let page_link = page_url
            .join(card_link)
            .map_err(|e| ScraperError::InvalidURL {
                url: page_url.to_string(),
                second: Some(card_link.to_string()),
                source: Box::new(e),
            })?
            .to_string();

        let rating_class = el
            .select(selectors::product_rating())
            .next() // use .next() instead of .nth(0)
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find rating element".to_string(),
            })?
            .attr("class")
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "rating element missing class attribute".to_string(),
            })?;

        let rating: Rating = rating_class
            .split_ascii_whitespace()
            .last()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: format!("rating class is empty: '{}'", rating_class),
            })?
            .parse() // This assumes Rating implements FromStr
            .map_err(|_| ScraperError::InvalidScraping {
                reason: format!("failed to parse rating from class '{}'", rating_class),
            })?;

        let price_text = el
            .select(selectors::card_price())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find price element".to_string(),
            })?
            .text()
            .collect::<String>();

        let price: f64 = price_text
            .trim()
            .trim_start_matches(CURRENCY_SYMBOL)
            .parse()
            .map_err(|_| ScraperError::InvalidScraping {
                reason: format!("couldn't parse price '{}' as f64", price_text),
            })?;

        let stock_raw = el
            .select(selectors::product_stock())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find stock element".to_string(),
            })?
            .text()
            .collect::<String>();

        let stock = stock_raw.parse::<Stock>()?;

        Ok(BookCard {
            thumbnail_link,
            title,
            page_link,
            rating,
            price,
            stock,
        })
    }

    fn get_page_url_for(origin: &Url, page: u32) -> Result<Url, ScraperError> {
        let mut url = origin.clone();
        if page > 0 {
            let mut segments = match url.path_segments_mut() {
                Ok(s) => s,
                Err(_) => {
                    return Err(ScraperError::InvalidURL {
                        url: origin.to_string(),
                        second: None,
                        source: "URL didn't have enough path segments".into(),
                    });
                }
            };
            segments.pop().push(&format!("page-{}.html", page + 1));
        }
        Ok(url)
    }

    fn active_handles_inc(&mut self) {
        if self.thread_ahead != 0 {
            self.active_handles += 1;
        }
    }

    fn active_handles_dec(&mut self) {
        if self.thread_ahead != 0 {
            self.active_handles -= 1;
        }
    }

    fn ensure_active_threads(&mut self) -> Result<(), ScraperError> {
        if let Some(last) = self.last_page
            && self.next_page_to_yield >= last
        {
            return Ok(());
        }

        if self.active_handles > self.thread_ahead {
            return Ok(());
        }

        let get_msg = |client, url: Url, next_page| -> Result<MessageType, ScraperError> {
            let resp = fetch_page(client, &url)?;
            if resp.status().as_u16() == 404 {
                return Ok(MessageType::NoMorePages {
                    last_page: next_page,
                });
            }

            Ok(MessageType::NextPage {
                page: next_page,
                cards: Self::parse_page(resp.text()?, &url)?,
            })
        };

        if self.thread_ahead == 0 {
            let url = Self::get_page_url_for(&self.url, self.next_page_to_yield)?;
            let _ = self
                .tx
                .send(get_msg(self.client.clone(), url, self.next_page_to_yield));
            self.next_page_to_yield += 1;
            return Ok(());
        }

        for _ in 0..(self.thread_ahead - self.active_handles) {
            let tx = self.tx.clone();
            let client = Arc::clone(&self.client);
            let next_page = self.next_page_to_yield;
            let origin = self.url.clone();

            thread::spawn(move || {
                let page_url = Self::get_page_url_for(&origin, next_page);
                let Ok(page_url) = page_url else {
                    let _ = tx.send(Err(page_url.unwrap_err()));
                    return;
                };
                let _ = tx.send(get_msg(client, page_url, next_page));
            });

            self.next_page_to_yield += 1;
            self.active_handles_inc();
        }

        Ok(())
    }
}

impl Iterator for BookCategoryPager {
    type Item = Result<BookCard, ScraperError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vec) = self.pages.remove(&self.next_page_to_return) {
            self.next_page_to_return += 1;
            if let Err(e) = self.ensure_active_threads() {
                return Some(Err(e));
            }
            self.buffer.extend(vec);
            return self.buffer.pop_front().map(Ok);
        }

        if let Some(book) = self.buffer.pop_front() {
            return Some(Ok(book));
        }

        if let Some(page) = self.last_page
            && self.next_page_to_return >= page
        {
            return None;
        }

        if let Err(e) = self.ensure_active_threads() {
            return Some(Err(e));
        }

        while let Ok(result) = self.rx.recv() {
            let Ok(msg) = result else {
                return Some(Err(result.unwrap_err()));
            };
            match msg {
                MessageType::NextPage { page, cards } => {
                    self.active_handles_dec();
                    if page == self.next_page_to_return {
                        self.next_page_to_return += 1;
                        self.buffer.extend(cards);
                        return self.buffer.pop_front().map(Ok);
                    }
                    self.pages.insert(page, cards);
                }
                MessageType::NoMorePages { last_page } => {
                    self.active_handles_dec();
                    self.last_page = Some(self.last_page.map_or(last_page, |l| l.min(last_page)));
                    return self.next();
                }
            }
            if let Err(e) = self.ensure_active_threads() {
                return Some(Err(e));
            }
        }

        None
    }
}

/// Paginate product cards via category URL.
/// See [BookCategoryPager::page] to optionally set the page.
/// And [BookCategoryPager::thread_ahead] for faster results.
pub fn paginate_category(category_url: &str) -> Result<BookCategoryPager, ScraperError> {
    BookCategoryPager::new(category_url)
}
