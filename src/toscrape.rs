pub(crate) mod book_category;
pub(crate) mod book_info;
pub(crate) mod category_pager;
pub(crate) mod fetching;
pub(crate) mod helpers;
pub(crate) mod rating;

use lazy_regex::{Lazy, Regex, regex};

pub use book_category::{BookCategory, fetch_categories};
pub use book_info::{BookCard, BookDetails, ProductType, fetch_book};
pub use category_pager::{BookCategoryPager, paginate_category};
pub use rating::Rating;

pub(crate) fn stock_regex() -> &'static Lazy<Regex> {
    regex!(r"(?<aval>In stock|Out of stock)(?: \((?<count>\d+) available\))?")
}

/// The origin URL. Hardly useful but available.
pub const ORIGIN_URL: &str = "https://books.toscrape.com/";
/// The currency symbol that the scraper trims. Hardly useful but available.
pub const CURRENCY_SYMBOL: &str = "£";
