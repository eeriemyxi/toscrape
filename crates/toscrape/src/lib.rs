pub(crate) mod book_category;
pub(crate) mod book_info;
pub(crate) mod category_pager;
pub(crate) mod enums;
pub(crate) mod errors;
pub(crate) mod fetching;
pub(crate) mod regexes;
pub(crate) mod selectors;

pub use book_category::{BookCategory, fetch_categories};
pub use book_info::{BookCard, BookDetails, fetch_book};
pub use category_pager::{BookCategoryPager, paginate_category};
pub use enums::{ProductType, Rating, Stock};
pub use errors::ScraperError;

/// The origin URL. Hardly useful but available.
pub const ORIGIN_URL: &str = "https://books.toscrape.com/";
/// The currency symbol that the scraper trims. Hardly useful but available.
pub const CURRENCY_SYMBOL: &str = "£";

#[cfg(test)]
mod tests {
    
}
