use std::str::FromStr;

use super::errors::ScraperError;

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
