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
