use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn key(&self) -> &str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "X",
            Rank::Jack => "X",
            Rank::Queen => "X",
            Rank::King => "X",
            Rank::Ace => "A",
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        };
        write!(f, "{}", s)
    }
}

// Implementing Iterator for Rank
impl Rank {
    pub fn iter() -> impl Iterator<Item = Rank> {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
        .iter()
        .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::Rank;

    #[test]
    fn test_rank_iter_and_methods() {
        let expected_ranks = vec![
            (Rank::Two, "2", 2, "2"),
            (Rank::Three, "3", 3, "3"),
            (Rank::Four, "4", 4, "4"),
            (Rank::Five, "5", 5, "5"),
            (Rank::Six, "6", 6, "6"),
            (Rank::Seven, "7", 7, "7"),
            (Rank::Eight, "8", 8, "8"),
            (Rank::Nine, "9", 9, "9"),
            (Rank::Ten, "X", 10, "Ten"),
            (Rank::Jack, "X", 10, "Jack"),
            (Rank::Queen, "X", 10, "Queen"),
            (Rank::King, "X", 10, "King"),
            (Rank::Ace, "A", 11, "Ace"),
        ];

        let actual_ranks: Vec<Rank> = Rank::iter().collect();

        // Check that Rank::iter() yields the expected sequence
        let expected_enum_order: Vec<Rank> = expected_ranks.iter().map(|(r, _, _, _)| *r).collect();
        assert_eq!(actual_ranks, expected_enum_order);

        // Check methods for each rank
        for (rank, expected_key, expected_value, expected_str) in expected_ranks {
            assert_eq!(rank.key(), expected_key, "Rank::{} key failed", expected_str);
            assert_eq!(rank.value(), expected_value, "Rank::{} value failed", expected_str);
            assert_eq!(format!("{}", rank), expected_str);
        }
    }
}
