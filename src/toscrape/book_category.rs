use scraper::Html;
use url::Url;

use super::{
    ORIGIN_URL, category_pager::BookCategoryPager, errors::ScraperError, fetching::fetch_page,
    selectors,
};

#[derive(Debug)]
/// Information regarding a book category.
pub struct BookCategory {
    /// The label of the category.
    pub label: String,
    /// The link to the list of products for the category.
    pub url: String,
}

impl BookCategory {
    /// Get a paginator for products.
    pub fn pages(self) -> BookCategoryPager {
        BookCategoryPager::new(self.url)
    }
}

/// Fetch the currently available categories from source.
pub fn fetch_categories() -> Result<Vec<BookCategory>, ScraperError> {
    let mut categories: Vec<BookCategory> = vec![];
    let url = Url::parse(ORIGIN_URL).map_err(|e| ScraperError::InvalidURL {
        url: ORIGIN_URL.to_string(),
        second: None,
        source: Box::new(e),
    })?;
    let (_, body) = fetch_page(url.as_str())?;
    for el in Html::parse_document(&body).select(selectors::nav_list()) {
        let label = String::from_iter(el.text()).trim().to_string();

        categories.push(BookCategory {
            label,
            url: url
                .join(
                    el.attr("href")
                        .ok_or_else(|| ScraperError::InvalidScraping {
                            reason: "Couldn'd find the href during creation of BookCategory"
                                .to_string(),
                        })?,
                )
                .map_err(|e| ScraperError::InvalidURL {
                    url: url.to_string(),
                    second: None,
                    source: Box::new(e),
                })?
                .to_string(),
        })
    }

    Ok(categories)
}
