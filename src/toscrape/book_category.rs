use scraper::Html;
use url::Url;

use super::{ORIGIN_URL, category_pager::BookCategoryPager, fetching::fetch_page, selectors};

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
pub fn fetch_categories() -> Option<Vec<BookCategory>> {
    let mut categories: Vec<BookCategory> = vec![];
    let url = Url::parse(ORIGIN_URL).ok()?;
    let (_, body) = fetch_page(url.as_str()).ok()?;
    for el in Html::parse_document(&body).select(selectors::nav_list()) {
        let label = String::from_iter(el.text()).trim().to_string();

        categories.push(BookCategory {
            label,
            url: url.join(el.attr("href")?).ok()?.to_string(),
        })
    }

    Some(categories)
}
