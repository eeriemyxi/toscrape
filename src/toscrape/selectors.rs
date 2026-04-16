use scraper::Selector;
use std::sync::LazyLock;

// Generic selectors

pub(crate) fn table_head() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("th").unwrap());
    &SELECTOR
}

pub(crate) fn table_def() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td").unwrap());
    &SELECTOR
}

// Category selectors
pub(crate) fn card() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("div > ol.row > li > article.product_pod").unwrap());
    &SELECTOR
}

pub(crate) fn card_thumbnail() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("img.thumbnail").unwrap());
    &SELECTOR
}

pub(crate) fn card_link() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h3 > a").unwrap());
    &SELECTOR
}

pub(crate) fn card_price() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("p.price_color").unwrap());
    &SELECTOR
}

// Dedicated book page selectors

pub(crate) fn nav_list() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse(".nav-list > li > ul > li > a").unwrap());
    &SELECTOR
}

pub(crate) fn product_thumbnail() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("div#product_gallery .thumbnail img").unwrap());
    &SELECTOR
}

pub(crate) fn product_main() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("div.product_main").unwrap());
    &SELECTOR
}

pub(crate) fn product_title() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("h1").unwrap());
    &SELECTOR
}

pub(crate) fn product_rating() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("p.star-rating").unwrap());
    &SELECTOR
}

pub(crate) fn product_stock() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("p.availability").unwrap());
    &SELECTOR
}

pub(crate) fn product_description() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("div#product_description + p").unwrap());
    &SELECTOR
}

pub(crate) fn product_info_table() -> &'static LazyLock<Selector> {
    static SELECTOR: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse(".sub-header + table tr").unwrap());
    &SELECTOR
}
