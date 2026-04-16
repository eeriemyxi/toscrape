use regex::Regex;
use std::sync::LazyLock;

pub(crate) fn stock_regex() -> &'static LazyLock<Regex> {
    static REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?<aval>In stock|Out of stock)(?: \((?<count>\d+) available\))?").unwrap()
    });
    &REGEX
}
