use core::f64;
use std::collections::HashMap;

use scraper::Html;
use url::Url;

use crate::fetching::get_client;

use super::{
    CURRENCY_SYMBOL, Rating,
    enums::{ProductType, Stock},
    errors::ScraperError,
    fetching::fetch_page,
    selectors::{self},
};

#[derive(Debug)]
/// Details about a book product.
pub struct BookDetails {
    /// The link to the dedicated page.
    pub page_link: String,
    /// The link to the thumbnail image.
    pub thumbnail_link: String,
    /// The title for the product.
    pub title: String,
    /// The rating for the product.
    pub rating: Rating,
    /// Availabality of the product.
    /// > **Note:**
    /// > So far only items in stock have been observed, thus the parsing is only partially tested.
    pub stock: Stock,
    /// The "UPC" information from product details.
    pub upc: String,
    /// The product type. So far only [ProductType::Book] has been observed.
    pub product_type: ProductType,
    /// The description of the product.
    pub description: String,
    /// The price of the product *without* include tax. See [BookDetails::tax] field.
    pub price: f64,
    /// The tax amount of the product.
    pub tax: f64,
    /// The amount of reviews on this product. The review data is unavailable from source.
    pub reviews_count: u64,
}

#[derive(Debug)]
/// The information scraped from a product card.
pub struct BookCard {
    /// The link to the dedicated page.
    pub page_link: String,
    /// The link to the thumbnail image.
    pub thumbnail_link: String,
    /// The title for the product.
    pub title: String,
    /// The rating for the product.
    pub rating: Rating,
    /// The price for the product.
    pub price: f64,
    /// Availability of the product.
    ///
    /// > **Note:**
    /// > So far only items in stock have been observed, thus the parsing is only partially tested.
    ///
    /// > **Note**:
    /// > [Stock::InStock::count] is always [None] because the data is unavailable in the card. You have to use [BookCard::full] or [fetch_book] to get that information from [BookDetails::stock].
    pub stock: Stock,
}

impl BookCard {
    /// Fetch more details from the dedicated product page. It'll be [None] if any error is encountered.
    pub fn full(self) -> Result<BookDetails, ScraperError> {
        fetch_book(&self.page_link)
    }
}

/// Fetch details for a book via its dedicated page URL.
pub fn fetch_book(book_url: &str) -> Result<BookDetails, ScraperError> {
    let response = fetch_page(get_client(), book_url)?;
    if response.status() == 404 {
        return Err(ScraperError::PageNotFound {
            url: book_url.to_string(),
        });
    }

    let url = Url::parse(book_url).map_err(|e| ScraperError::InvalidURL {
        url: book_url.to_string(),
        second: None,
        source: Box::new(e),
    })?;
    let html = Html::parse_document(&response.text()?);
    let root = html.root_element();

    let thumbnail_el = root
        .select(selectors::product_thumbnail())
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
    let thumbnail_link = url
        .join(thumbnail_src)
        .map_err(|e| ScraperError::InvalidURL {
            url: url.to_string(),
            second: Some(thumbnail_src.to_string()),
            source: Box::new(e),
        })?
        .to_string();

    let product_main_el = root
        .select(selectors::product_main())
        .next()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find product_main element".to_string(),
        })?;

    let title = product_main_el
        .select(selectors::product_title())
        .next()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find the product title element".to_string(),
        })?
        .text()
        .collect::<String>();

    let page_link = book_url.to_string();

    let rating: Rating = product_main_el
        .select(selectors::product_rating())
        .next()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find the product rating element".to_string(),
        })?
        .attr("class")
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find the `class` element for the product rating element".to_string(),
        })?
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "no classes exist for the product rating element".to_string(),
        })?
        .parse()?;

    let stock_raw = product_main_el
        .select(selectors::product_stock())
        .next()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find the product stock element".to_string(),
        })?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let stock = stock_raw.parse::<Stock>()?;

    let description = root
        .select(selectors::product_description())
        .next()
        .ok_or_else(|| ScraperError::InvalidScraping {
            reason: "couldn't find the product description element".to_string(),
        })?
        .text()
        .collect::<String>();

    let mut table: HashMap<String, String> = HashMap::new();
    for el in root.select(selectors::product_info_table()) {
        let head = el
            .select(selectors::table_head())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find the table head while trying to find product information"
                    .to_string(),
            })?
            .text()
            .collect::<String>();
        let def = el
            .select(selectors::table_def())
            .next()
            .ok_or_else(|| ScraperError::InvalidScraping {
                reason: "couldn't find the table def while trying to find product information"
                    .to_string(),
            })?
            .text()
            .collect::<String>();
        table.insert(head, def);
    }

    macro_rules! use_table {
        ($table:ident, $key:expr) => {{
            $table
                .get($key)
                .ok_or_else(|| ScraperError::InvalidScraping {
                    reason: format!(
                        "couldn't find {:?} in {:?} during product info parsing",
                        $key, &$table
                    ),
                })
        }};
    }

    let upc = use_table!(table, "UPC")?.clone();

    let product_type = use_table!(table, "Product Type")?.parse::<ProductType>()?;

    let price: f64 = use_table!(table, "Price (excl. tax)")?
        .to_string()
        .trim_start_matches(CURRENCY_SYMBOL)
        .parse()
        .map_err(|_| ScraperError::InvalidScraping {
            reason: format!(
                "Couldn't parse price {:?} as f64",
                use_table!(table, "Price (excl. tax)")
            ),
        })?;

    let tax: f64 = use_table!(table, "Tax")?
        .to_string()
        .trim_start_matches(CURRENCY_SYMBOL)
        .parse()
        .map_err(|_| ScraperError::InvalidScraping {
            reason: format!("Couldn't parse tax {:?} as f64", use_table!(table, "Tax")),
        })?;

    let reviews_count: u64 = use_table!(table, "Number of reviews")?
        .to_string()
        .parse()
        .map_err(|_| ScraperError::InvalidScraping {
            reason: format!(
                "Couldn't parse review count {:?} as u64",
                use_table!(table, "Number of reviews")
            ),
        })?;

    Ok(BookDetails {
        description,
        upc,
        price,
        tax,
        stock,
        reviews_count,
        product_type,
        page_link,
        rating,
        title,
        thumbnail_link,
    })
}
