use scraper::Selector;
use std::sync::LazyLock;

macro_rules! create_selector {
    ($name:ident, $select:expr) => {
        pub(crate) fn $name() -> &'static LazyLock<Selector> {
            static SELECTOR: LazyLock<Selector> =
                LazyLock::new(|| Selector::parse($select).unwrap());
            &SELECTOR
        }
    };
}

// Generic selectors
create_selector!(table_head, "th");
create_selector!(table_def, "td");

// Category selectors
create_selector!(card, "div > ol.row > li > article.product_pod");
create_selector!(card_thumbnail, "img.thumbnail");
create_selector!(card_link, "h3 > a");
create_selector!(card_price, "p.price_color");

// Dedicated book page selectors
create_selector!(nav_list, ".nav-list > li > ul > li > a");
create_selector!(product_thumbnail, "div#product_gallery .thumbnail img");
create_selector!(product_main, "div.product_main");
create_selector!(product_title, "h1");
create_selector!(product_rating, "p.star-rating");
create_selector!(product_stock, "p.availability");
create_selector!(product_description, "div#product_description + p");
create_selector!(product_info_table, ".sub-header + table tr");
