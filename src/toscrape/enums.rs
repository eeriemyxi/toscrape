use std::str::FromStr;

use super::{errors::ScraperError, regexes::stock_regex};

#[derive(Debug)]
/// The rating for a product.
pub enum Rating {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl FromStr for Rating {
    type Err = ScraperError;

    fn from_str(input: &str) -> Result<Rating, Self::Err> {
        match input {
            "One" => Ok(Rating::One),
            "Two" => Ok(Rating::Two),
            "Three" => Ok(Rating::Three),
            "Four" => Ok(Rating::Four),
            "Five" => Ok(Rating::Five),
            _ => Err(ScraperError::InvalidRating {
                input: input.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
/// Enum for product types supported by source.
pub enum ProductType {
    Book,
}

impl FromStr for ProductType {
    type Err = ScraperError;

    fn from_str(input: &str) -> Result<ProductType, Self::Err> {
        match input {
            "Books" => Ok(ProductType::Book),
            _ => Err(ScraperError::InvalidProductType {
                input: input.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
/// Enum for product availability
pub enum Stock {
    /// The product is in stock.
    InStock {
        /// The amount of stocks available.
        count: Option<u64>,
    },
    /// The product is out of stock.
    OutOfStock,
}

impl FromStr for Stock {
    type Err = ScraperError;

    fn from_str(input: &str) -> Result<Stock, Self::Err> {
        let regex = stock_regex();
        let capts = regex
            .captures(input.trim())
            .ok_or_else(|| ScraperError::InvalidStock {
                input: input.to_string(),
            })?;

        let mut stock = match &capts["aval"] {
            "Out of stock" => Stock::OutOfStock,
            "In stock" => Stock::InStock { count: None },
            _ => Stock::OutOfStock,
        };

        if let Some(count) = capts.name("count") {
            let parsed_count =
                count
                    .as_str()
                    .parse::<u64>()
                    .map_err(|_| ScraperError::InvalidScraping {
                        reason: format!("coudn't convert {:?} to u64", count),
                    })?;
            stock = Stock::InStock {
                count: Some(parsed_count),
            }
        }

        Ok(stock)
    }
}
