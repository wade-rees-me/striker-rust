use crate::cards::Card;

#[derive(Debug, Clone, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub total: usize,
    soft_ace: usize,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            total: 0,
            soft_ace: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.total = 0;
        self.soft_ace = 0;
    }

    pub fn draw_card(&mut self, maybe_card: Option<Card>) {
        let card = maybe_card.expect("Expected a card but got None");
        self.cards.push(card);
        self.calculate_total();
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.total == 21
    }

    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].same_rank(&self.cards[1])
    }

    pub fn is_pair_of_aces(&self) -> bool {
        self.cards[0].is_ace() && self.is_pair()
    }

    pub fn is_busted(&self) -> bool {
        self.total > 21
    }

    pub fn is_soft(&self) -> bool {
        self.soft_ace > 0
    }

    pub fn is_soft_17(&self) -> bool {
        self.total == 17 && self.is_soft()
    }

    pub fn get_card_pair(&self) -> Option<&Card> {
        self.cards.get(0)
    }

    pub fn split_pair(&mut self) -> Result<Card, &'static str> {
        if !self.is_pair() {
            return Err("Error: Trying to split a non-pair");
        }
        let card = self.cards.pop().expect("Hand was empty when trying to split");
        self.calculate_total();
        Ok(card)
    }

    fn calculate_total(&mut self) {
        self.total = 0;
        self.soft_ace = 0;

        for card in &self.cards {
            let val = card.rank.value();
            self.total += val;
            if val == 11 {
                self.soft_ace += 1;
            }
        }

        while self.total > 21 && self.soft_ace > 0 {
            self.total -= 10;
            self.soft_ace -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Rank, Suit};

    fn make_card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    fn make_hand(cards: Vec<(Rank, Suit)>) -> Hand {
        let mut hand = Hand::new();
        for (rank, suit) in cards {
            hand.draw_card(Some(make_card(rank, suit)));
        }
        hand
    }

    #[test]
    fn test_blackjack_and_soft_hand() {
        let hand = make_hand(vec![(Rank::Ace, Suit::Spades), (Rank::Ten, Suit::Clubs)]);
        assert_eq!(hand.total, 21);
        assert!(hand.is_blackjack());
        assert!(!hand.is_pair());
        assert!(!hand.is_busted());
        assert!(hand.is_soft());
        assert!(!hand.is_soft_17());
    }

    #[test]
    fn test_pair_of_aces_soft_17() {
        let mut hand = make_hand(vec![(Rank::Ace, Suit::Spades), (Rank::Ace, Suit::Spades)]);
        assert_eq!(hand.total, 12);
        assert!(hand.is_pair());
        assert!(hand.is_pair_of_aces());
        assert!(hand.is_soft());
        assert!(!hand.is_soft_17());

        hand.draw_card(Some(make_card(Rank::Four, Suit::Spades)));
        assert_eq!(hand.total, 16);
        hand.draw_card(Some(make_card(Rank::Ace, Suit::Hearts))); // 16 + 11 → 17 (soft)
        assert_eq!(hand.total, 17);
        assert!(hand.is_soft_17());
    }

    #[test]
    fn test_split_pair_success() {
        let mut hand = make_hand(vec![(Rank::Ace, Suit::Hearts), (Rank::Ace, Suit::Clubs)]);
        let result = hand.split_pair();
        assert!(result.is_ok());

        let remaining_card = hand.get_card_pair();
        assert_eq!(remaining_card, Some(&make_card(Rank::Ace, Suit::Hearts)));
    }

    #[test]
    #[should_panic(expected = "Expected a card but got None")]
    fn test_draw_card_none_should_panic() {
        let mut hand = Hand::new();
        hand.draw_card(None);
    }

    #[test]
    #[should_panic(expected = "Error: Trying to split a non-pair")]
    fn test_split_non_pair_should_panic() {
        let mut hand = make_hand(vec![(Rank::Ace, Suit::Spades), (Rank::Ten, Suit::Clubs)]);
        hand.split_pair().unwrap();
    }
}
