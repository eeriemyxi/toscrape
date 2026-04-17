use std::collections::VecDeque;

use crate::toscrape::{enums::Stock, selectors};

use super::{
    CURRENCY_SYMBOL, Rating, book_info::BookCard, errors::ScraperError, fetching::fetch_page,
};
use scraper::Html;
use url::Url;

/// Paginator for the product cards of a category.
pub struct BookCategoryPager {
    /// The link to the category.
    url: Url,
    /// The page to paginate. This property will change when iterated.
    page: u32,
    buffer: VecDeque<BookCard>,
}

impl BookCategoryPager {
    pub fn new(url: impl ToString) -> Result<Self, ScraperError> {
        Ok(Self {
            url: Url::parse(&url.to_string()).map_err(|e| ScraperError::InvalidURL {
                url: url.to_string(),
                second: None,
                source: Box::new(e),
            })?,
            page: 0,
            buffer: VecDeque::new(),
        })
    }

    /// Set the active page. Could be used to paginate from a certain number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    fn fetch_next_page(&self, page_url: &str) -> Result<Vec<BookCard>, ScraperError> {
        let mut books = vec![];

        let (curl, body) = fetch_page(page_url)?;
        if curl.response_code()? == 404 {
            return Err(ScraperError::PageNotFound {
                url: self.url.to_string(),
            });
        }

        for el in Html::parse_document(&body).select(selectors::card()) {
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

            let thumbnail_link = self
                .url
                .join(thumbnail_src)
                .map_err(|e| ScraperError::InvalidURL {
                    url: self.url.to_string(),
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

            let page_link = self
                .url
                .join(card_link)
                .map_err(|e| ScraperError::InvalidURL {
                    url: self.url.to_string(),
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

            books.push(BookCard {
                thumbnail_link,
                title,
                page_link,
                rating,
                price,
                stock,
            })
        }

        Ok(books)
    }
}

impl Iterator for BookCategoryPager {
    type Item = Result<BookCard, ScraperError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(book) = self.buffer.pop_front() {
            return Some(Ok(book));
        }

        let mut url = match Url::parse(self.url.as_ref()) {
            Ok(u) => u,
            Err(e) => {
                return Some(Err(ScraperError::InvalidURL {
                    url: self.url.to_string(),
                    second: None,
                    source: Box::new(e),
                }));
            }
        };

        if self.page > 0 {
            let mut segments = match url.path_segments_mut() {
                Ok(s) => s,
                Err(_) => {
                    return Some(Err(ScraperError::InvalidURL {
                        url: self.url.to_string(),
                        second: None,
                        source: "URL didn't have enough path segments".into(),
                    }));
                }
            };
            segments
                .pop()
                .push(format!("page-{}.html", self.page + 1).as_str());
        }

        let result = self.fetch_next_page(url.as_str());

        match result {
            Ok(r) => {
                self.page += 1;
                self.buffer.extend(r);
                self.buffer.pop_front().map(Ok)
            }
            Err(ScraperError::PageNotFound { url: _ }) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Paginate product cards via category URL. See [BookCategoryPager::page] to optionally set the page.
pub fn paginate_category(category_url: &str) -> Result<BookCategoryPager, ScraperError> {
    BookCategoryPager::new(category_url)
}
