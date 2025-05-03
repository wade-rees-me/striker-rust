#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn to_string(&self) -> &str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        }
    }
}

// Implementing Iterator for Suit
impl Suit {
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_to_string() {
        assert_eq!(Suit::Hearts.to_string(), "Hearts");
        assert_eq!(Suit::Diamonds.to_string(), "Diamonds");
        assert_eq!(Suit::Clubs.to_string(), "Clubs");
        assert_eq!(Suit::Spades.to_string(), "Spades");
    }
}
