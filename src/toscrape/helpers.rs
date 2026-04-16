use scraper::{ElementRef, Selector};

pub(crate) fn select_first_element(element: ElementRef, selection: String) -> Option<ElementRef> {
    element.select(&Selector::parse(&selection).ok()?).next()
}

pub(crate) trait StockParseExt {
    fn parse_stock(&self) -> bool;
}

impl StockParseExt for str {
    fn parse_stock(&self) -> bool {
        match self {
            "In stock" => true,
            "Out of stock" => false,
            _ => false,
        }
    }
}
