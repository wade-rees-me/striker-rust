use crate::cards::hand::Hand;

#[derive(Debug, Clone)]
pub struct Dealer {
    pub hand: Hand,
    hit_soft_17: bool,
}

impl Dealer {
    pub fn new(hit_soft_17: bool) -> Self {
        let mut dealer = Dealer { hand: Hand::new(), hit_soft_17 };
        dealer.hand.reset();
        dealer
    }

    pub fn should_stand(&self) -> bool {
        if self.hit_soft_17 && self.hand.is_soft_17() {
            return false;
        }
        self.hand.total >= 17
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Rank, Suit};

    #[test]
    fn test_dealer_should_hit_on_soft_17() {
        let mut dealer = Dealer::new(true);

        assert_eq!(dealer.hand.total, 0);

        dealer.hand.draw_card(Some(Card::new(Rank::Ace, Suit::Hearts)));
        assert_eq!(dealer.hand.total, 11);

        dealer.hand.draw_card(Some(Card::new(Rank::Six, Suit::Spades)));
        assert_eq!(dealer.hand.total, 17); // Soft
        assert!(!dealer.should_stand());

        dealer.hand.draw_card(Some(Card::new(Rank::Ten, Suit::Spades)));
        assert_eq!(dealer.hand.total, 17); // Hard
        assert!(dealer.should_stand());
    }

    #[test]
    fn test_dealer_should_stand_on_soft_17() {
        let mut dealer = Dealer::new(false);

        dealer.hand.draw_card(Some(Card::new(Rank::Ace, Suit::Hearts)));
        assert_eq!(dealer.hand.total, 11);

        dealer.hand.draw_card(Some(Card::new(Rank::Six, Suit::Spades)));
        assert_eq!(dealer.hand.total, 17); // Soft
        assert!(dealer.should_stand());
    }
}
