use std::str::FromStr;

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
    type Err = ();

    fn from_str(input: &str) -> Result<Rating, Self::Err> {
        match input {
            "One" => Ok(Rating::One),
            "Two" => Ok(Rating::Two),
            "Three" => Ok(Rating::Three),
            "Four" => Ok(Rating::Four),
            "Five" => Ok(Rating::Five),
            _ => Err(()),
        }
    }
}
