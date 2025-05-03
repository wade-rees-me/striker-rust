use crate::cards::hand::Hand;

#[derive(Debug, Clone)]
pub struct Wager {
    pub hand: Hand,
    minimum_bet: usize,
    maximum_bet: usize,
    pub amount_bet: usize,
    pub amount_won: isize,
    pub insurance_bet: usize,
    pub insurance_won: isize,
}

impl Wager {
    pub fn new(minimum_bet: usize, maximum_bet: usize) -> Self {
        Wager {
            hand: Hand::new(),
            minimum_bet,
            maximum_bet,
            amount_bet: 0,
            amount_won: 0,
            insurance_bet: 0,
            insurance_won: 0,
        }
    }

    pub fn place_bet(&mut self, bet: usize) {
        self.hand.reset();
        self.amount_bet = ((self.maximum_bet.min(self.minimum_bet.max(bet)) + 1) / 2) * 2;
        self.amount_won = 0;
        self.insurance_bet = 0;
        self.insurance_won = 0;
    }

    pub fn place_insurance_bet(&mut self) {
        self.insurance_bet = self.amount_bet / 2;
    }

    pub fn double_bet(&mut self) {
        self.amount_bet *= 2;
    }

    pub fn won_blackjack(&mut self, pays: usize, bet: usize) {
        self.amount_won = ((self.amount_bet * pays) / bet) as isize;
    }

    pub fn won(&mut self) {
        self.amount_won = self.amount_bet as isize;
    }

    pub fn lost(&mut self) {
        self.amount_won = -(self.amount_bet as isize);
    }

    pub fn push(&mut self) {
        // No action needed
    }

    pub fn won_insurance(&mut self) {
        self.insurance_won = self.insurance_bet as isize * 2;
    }

    pub fn lost_insurance(&mut self) {
        self.insurance_won = -(self.insurance_bet as isize);
    }

    pub fn split_hand(&mut self, split: &mut Wager) {
        if !self.hand.is_pair() {
            panic!("Cannot split a non-pair hand");
        }

        split.amount_bet = self.amount_bet;
        let split_card = self.hand.split_pair().expect("Hand must be a pair to split");
        split.hand.draw_card(Some(split_card)); // Draw the split card into the new wager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Rank, Suit};
    use crate::constants::constants::{MAXIMUM_BET, MINIMUM_BET};

    const DOUBLE_MIN_BET: usize = MINIMUM_BET * 2;

    fn new_test_wager() -> Wager {
        Wager::new(MINIMUM_BET, MAXIMUM_BET)
    }

    #[test]
    fn test_initialization() {
        let wager = new_test_wager();
        assert_eq!(wager.minimum_bet, MINIMUM_BET);
        assert_eq!(wager.maximum_bet, MAXIMUM_BET);
        assert_eq!(wager.amount_bet, 0);
        assert_eq!(wager.amount_won, 0);
        assert_eq!(wager.insurance_bet, 0);
        assert_eq!(wager.insurance_won, 0);
    }

    #[test]
    fn test_place_bet_within_bounds() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);
        assert_eq!(wager.amount_bet, DOUBLE_MIN_BET);
    }

    #[test]
    fn test_place_bet_below_minimum() {
        let mut wager = new_test_wager();
        wager.place_bet(MINIMUM_BET - 2);
        assert_eq!(wager.amount_bet, MINIMUM_BET);
    }

    #[test]
    fn test_place_bet_above_maximum() {
        let mut wager = new_test_wager();
        wager.place_bet(MAXIMUM_BET + 2);
        assert_eq!(wager.amount_bet, MAXIMUM_BET);
    }

    #[test]
    fn test_insurance_and_double_bet() {
        let mut wager = new_test_wager();
        wager.place_bet(MINIMUM_BET);
        wager.place_insurance_bet();
        assert_eq!(wager.insurance_bet, MINIMUM_BET / 2);

        wager.double_bet();
        assert_eq!(wager.amount_bet, DOUBLE_MIN_BET);
    }

    #[test]
    fn test_won_blackjack() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);
        wager.won_blackjack(3, 2); // 3:2 payout
        assert_eq!(wager.amount_won, (DOUBLE_MIN_BET * 3 / 2) as isize);
    }

    #[test]
    fn test_won_and_lost() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);

        wager.won();
        assert_eq!(wager.amount_won, DOUBLE_MIN_BET as isize);

        wager.place_bet(DOUBLE_MIN_BET);
        wager.lost();
        assert_eq!(wager.amount_won, -(DOUBLE_MIN_BET as isize));
    }

    #[test]
    fn test_push_resets_winnings() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);
        wager.push();
        assert_eq!(wager.amount_won, 0);
    }

    #[test]
    fn test_insurance_outcomes() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);
        wager.place_insurance_bet();

        wager.won_insurance();
        assert_eq!(wager.insurance_won, DOUBLE_MIN_BET as isize);

        wager.place_insurance_bet();
        wager.lost_insurance();
        assert_eq!(wager.insurance_won, -(MINIMUM_BET as isize));
    }

    #[test]
    fn test_split_hand_valid_pair() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);

        let ace1 = Card::new(Rank::Ace, Suit::Hearts);
        let ace2 = Card::new(Rank::Ace, Suit::Spades);
        wager.hand.draw_card(Some(ace1.clone()));
        wager.hand.draw_card(Some(ace2));

        let mut split_wager = new_test_wager();
        wager.split_hand(&mut split_wager);

        assert_eq!(wager.hand.total, 11);
        assert_eq!(split_wager.hand.total, 11);
        assert_eq!(wager.amount_bet, DOUBLE_MIN_BET);
        assert_eq!(split_wager.amount_bet, DOUBLE_MIN_BET);
    }

    #[test]
    #[should_panic(expected = "Cannot split a non-pair hand")]
    fn test_split_hand_invalid_pair_panics() {
        let mut wager = new_test_wager();
        wager.place_bet(DOUBLE_MIN_BET);
        wager.hand.draw_card(Some(Card::new(Rank::Ace, Suit::Hearts)));
        wager.hand.draw_card(Some(Card::new(Rank::King, Suit::Spades)));

        let mut split_wager = new_test_wager();
        wager.split_hand(&mut split_wager);
    }
}
