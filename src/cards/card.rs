use crate::cards::{Rank, Suit};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn same_rank(&self, other: &Card) -> bool {
        self.rank == other.rank
    }

    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank.to_string(), self.suit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Rank, Suit};

    #[test]
    fn test_card_is_ace() {
        let ace_card = Card::new(Rank::Ace, Suit::Clubs);
        let non_ace_card = Card::new(Rank::King, Suit::Spades);

        assert!(ace_card.is_ace());
        assert!(!non_ace_card.is_ace());
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        let card_str = format!("{}", card);
        assert_eq!(card_str, "Ace of Hearts");

        let card = Card::new(Rank::Ten, Suit::Clubs);
        let card_str = format!("{}", card);
        assert_eq!(card_str, "Ten of Clubs");
    }

    #[test]
    fn test_card_creation() {
        let card1 = Card::new(Rank::Ace, Suit::Spades);
        let card2 = Card::new(Rank::Ace, Suit::Hearts);

        assert_eq!(card1.rank, Rank::Ace);
        assert_eq!(card1.suit, Suit::Spades);

        assert_eq!(card2.rank, Rank::Ace);
        assert_eq!(card2.suit, Suit::Hearts);

        assert!(card1.same_rank(&card2));
    }
}
