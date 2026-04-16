use core::f64;
use std::{collections::HashMap};

use scraper::{Html, Selector};
use url::Url;

use super::{
    CURRENCY_SYMBOL, Rating,
    fetching::fetch_page,
    enums::ProductType,
    helpers::{StockParseExt, select_first_element},
    stock_regex,
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
    /// Whether the product is in stock or not. So far only items in stock have been observed, thus the parsing is only partially tested.
    pub in_stock: bool,
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
    /// The amount of stocks that are available.
    pub stock_count: u64,
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
    /// Whether the product is in stock or not. So far only items in stock have been observed, thus the parsing is only partially tested.
    pub in_stock: bool,
}

impl BookCard {
    /// Fetch more details from the dedicated product page. It'll be [None] if any error is encountered.
    pub fn full(self) -> Option<BookDetails> {
        fetch_book(&self.page_link)
    }
}

/// Fetch details for a book via its dedicated page URL.
pub fn fetch_book(book_url: &str) -> Option<BookDetails> {
    let (curl, body) = fetch_page(book_url).ok()?;
    if curl.response_code().ok()? != 200 {
        return None;
    }

    let url = Url::parse(book_url).ok()?;
    let html = Html::parse_document(&body);
    let root = html.root_element();

    let thumbnail_el =
        select_first_element(root, "div#product_gallery .thumbnail img".to_string())?;
    let thumbnail_link = url.join(thumbnail_el.attr("src")?.trim()).ok()?.to_string();

    let product_main_el = select_first_element(root, "div.product_main".to_string())?;

    let title = String::from_iter(select_first_element(product_main_el, "h1".to_string())?.text());

    let page_link = book_url.to_string();

    let rating: Rating = select_first_element(product_main_el, "p.star-rating".to_string())?
        .attr("class")?
        .split_ascii_whitespace()
        .last()?
        .parse()
        .ok()?;

    let stock_raw = String::from_iter(
        select_first_element(product_main_el, "p.availability".to_string())?.text(),
    )
    .trim()
    .to_string();

    let stock_capt = stock_regex().captures(&stock_raw)?;

    let in_stock = stock_capt["aval"].parse_stock();

    let stock_count = stock_capt["count"].parse::<u64>().ok()?;

    let description = String::from_iter(
        select_first_element(root, "div#product_description + p".to_string())?.text(),
    );

    let mut table: HashMap<String, String> = HashMap::new();
    for el in root.select(&Selector::parse(".sub-header + table tr").ok()?) {
        let head = String::from_iter(select_first_element(el, "th".to_string())?.text());
        let def = String::from_iter(select_first_element(el, "td".to_string())?.text());
        table.insert(head, def);
    }

    let upc = table.get("UPC")?.clone();

    let product_type = (*table.get("Product Type")?).parse::<ProductType>().ok()?;

    let price: f64 = (*table.get("Price (excl. tax)")?)
        .to_string()
        .trim_start_matches(CURRENCY_SYMBOL)
        .parse()
        .ok()?;

    let tax: f64 = (*table.get("Tax")?)
        .to_string()
        .trim_start_matches(CURRENCY_SYMBOL)
        .parse()
        .ok()?;

    let reviews_count: u64 = (*table.get("Number of reviews")?)
        .to_string()
        .parse()
        .ok()?;

    Some(BookDetails {
        description,
        upc,
        price,
        tax,
        stock_count,
        reviews_count,
        product_type,
        in_stock,
        page_link,
        rating,
        title,
        thumbnail_link,
    })
}
