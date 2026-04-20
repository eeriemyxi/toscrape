use regex::Regex;
use std::sync::LazyLock;

macro_rules! create_regex {
    ($name:ident, $regex:expr) => {
        pub(crate) fn $name() -> &'static LazyLock<Regex> {
            static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
            &REGEX
        }
    };
}

create_regex!(
    stock_regex,
    r"(?<aval>In stock|Out of stock)(?: \((?<count>\d+) available\))?"
);
