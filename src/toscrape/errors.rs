use reqwest::Error;
use thiserror;

#[derive(thiserror::Error, Debug)]
/// Errors that are raised by this library.
pub enum ScraperError {
    /// The URL was invalid.
    #[error("couldn't handle URL operation `{url}` (with {second:?}): {source}")]
    InvalidURL {
        url: String,
        second: Option<String>,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// The given page was invalid.
    #[error("encounted error while trying to fetch a page")]
    InvalidPage(#[from] Error),
    #[error("given page not found: `{url}`")]
    /// The given page doesn't exist (404).
    PageNotFound { url: String },
    #[error("a critical error happened during scraping: {reason}")]
    /// Used when something unexpected happens during scraping process.
    InvalidScraping { reason: String },
    #[error("couldn't convert {input} to Rating")]
    /// Couldn't convert input to a [Rating](crate::toscrape::enums::Rating).
    InvalidRating { input: String },
    /// Couldn't convert input to a [ProductType](crate::toscrape::enums::ProductType).
    #[error("couldn't convert {input} to ProductType")]
    InvalidProductType { input: String },
    /// Couldn't convert input to a [Stock](crate::toscrape::enums::Stock).
    #[error("couldn't convert {input} to Stock")]
    InvalidStock { input: String },
}
