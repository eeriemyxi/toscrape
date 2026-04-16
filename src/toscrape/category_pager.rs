use crate::toscrape::selectors;

use super::{
    CURRENCY_SYMBOL, Rating, book_info::BookCard, fetching::fetch_page, helpers::StockParseExt,
};
use scraper::Html;
use url::Url;

/// Paginator for the product cards of a category.
pub struct BookCategoryPager {
    /// The link to the category.
    url: String,
    /// The page to paginate. This property will change when iterated.
    page: u32,
}

impl BookCategoryPager {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            page: 0,
        }
    }

    /// Set the active page. Could be used to paginate from a certain number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }
}

impl Iterator for BookCategoryPager {
    type Item = Vec<BookCard>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut books = vec![];
        let mut url = Url::parse(&self.url).unwrap();
        if self.page > 0 {
            url.path_segments_mut()
                .ok()?
                .pop()
                .push(format!("page-{}.html", self.page + 1).as_str());
        }

        let (curl, body) = fetch_page(url.clone().as_str()).ok()?;
        if curl.response_code().ok()? != 200 {
            return None;
        }

        for el in Html::parse_document(&body).select(selectors::card()) {
            let thumbnail_el = el.select(selectors::card_thumbnail()).nth(0)?;

            let thumbnail_link = url.join(thumbnail_el.attr("src")?.trim()).ok()?.to_string();

            let title = thumbnail_el.attr("alt").unwrap().trim().to_string();

            let page_link = url
                .join(
                    el.select(selectors::card_link())
                        .nth(0)?
                        .attr("href")
                        .unwrap()
                        .trim(),
                )
                .ok()?
                .to_string();

            let rating: Rating = el
                .select(selectors::product_rating())
                .nth(0)?
                .attr("class")?
                .split_ascii_whitespace()
                .last()?
                .parse()
                .ok()?;

            let price: f64 = String::from_iter(el.select(selectors::card_price()).nth(0)?.text())
                .trim()
                .trim_start_matches(CURRENCY_SYMBOL)
                .parse()
                .ok()?;

            let stock_raw = String::from_iter(el.select(selectors::product_stock()).nth(0)?.text())
                .trim()
                .to_string();

            let in_stock = stock_raw.as_str().parse_stock();

            books.push(BookCard {
                thumbnail_link,
                title,
                page_link,
                rating,
                price,
                in_stock,
            })
        }

        self.page += 1;
        Some(books)
    }
}

/// Paginate product cards via category URL. See [BookCategoryPager::page] to optionally set the page.
pub fn paginate_category(category_url: &str) -> BookCategoryPager {
    BookCategoryPager::new(category_url)
}
