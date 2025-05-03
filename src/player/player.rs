use crate::cards::{card::Card, shoe::Shoe, wager::Wager};
use crate::constants::constants::{MAXIMUM_BET, MINIMUM_BET};
use crate::report::report::Report;
use crate::strategy::rules::Rules;
use crate::strategy::strategy::Strategy;

pub struct Player {
    pub rules: Rules,
    pub strategy: Strategy,
    pub wager: Wager,
    pub splits: Vec<Wager>,
    pub report: Report,
    pub seen_cards: [usize; 13],
}

impl Player {
    pub fn new(rules: &Rules, strategy: &Strategy) -> Self {
        Self {
            rules: rules.clone(),
            strategy: strategy.clone(),
            wager: Wager::new(MINIMUM_BET, MAXIMUM_BET),
            splits: Vec::new(),
            report: Report::new(),
            seen_cards: [0; 13],
        }
    }

    pub fn shuffle(&mut self) {
        self.seen_cards = [0; 13];
    }

    pub fn draw_card(&mut self, maybe_card: Option<Card>) {
        self.wager.hand.draw_card(maybe_card.clone());
        self.show_card(&maybe_card.clone());
    }

    pub fn show_card(&mut self, card: &Option<Card>) {
        if let Some(card) = card {
            let value = card.rank.value() as usize;
            self.seen_cards[value] += 1;
        }
    }

    pub fn get_report(&self) -> &Report {
        &self.report
    }

    pub fn busted_or_blackjack(&self) -> bool {
        if self.splits.is_empty() {
            return self.wager.hand.is_busted() || self.wager.hand.is_blackjack();
        }
        !self.splits.iter().any(|split| !split.hand.is_busted())
    }

    pub fn place_bet(&mut self, mimic: bool) {
        self.splits.clear();
        self.wager.hand.reset();
        if mimic {
            self.wager.place_bet(MINIMUM_BET);
        } else {
            let bet = self.strategy.get_bet(&self.seen_cards);
            self.wager.place_bet(bet);
        }
    }

    pub fn insurance(&mut self) {
        if self.strategy.get_insurance(&self.seen_cards) {
            self.wager.place_insurance_bet();
        }
    }

    pub fn mimic_stand(&self) -> bool {
        !self.wager.hand.is_soft_17() && self.wager.hand.total >= 17
    }

    pub fn play(&mut self, up: &Card, shoe: &mut Shoe, mimic: bool) {
        if self.wager.hand.is_blackjack() {
            self.report.total_blackjacks += 1;
            return;
        }

        if mimic {
            while !self.mimic_stand() {
                self.draw_card(shoe.draw_card());
            }
            return;
        }

        if self.strategy.get_double(&self.seen_cards, self.wager.hand.total, self.wager.hand.is_soft(), up) {
            self.wager.double_bet();
            self.draw_card(shoe.draw_card());
            self.report.total_doubles += 1;
            return;
        }

        if self.wager.hand.is_pair() && self.strategy.get_split(&self.seen_cards, self.wager.hand.get_card_pair().unwrap(), up) {
            self.handle_split(shoe, up);
            return;
        }

        self.hit_until_stand_or_bust(shoe, up);
    }

    fn hit_until_stand_or_bust(&mut self, shoe: &mut Shoe, up: &Card) {
        while !self.wager.hand.is_busted() && !self.strategy.get_stand(&self.seen_cards, self.wager.hand.total, self.wager.hand.is_soft(), up) {
            self.draw_card(shoe.draw_card());
        }
    }

    fn handle_split(&mut self, shoe: &mut Shoe, up: &Card) {
        let mut wager = std::mem::replace(&mut self.wager, Wager::new(MINIMUM_BET, MAXIMUM_BET));
        let mut split = Wager::new(MINIMUM_BET, MAXIMUM_BET);

        self.report.total_splits += 1;
        if wager.hand.is_pair_of_aces() {
            self.report.total_splits_ace += 1;

            wager.split_hand(&mut split);
            let card = shoe.draw_card();
            self.show_card(&card);
            wager.hand.draw_card(card);

            let card = shoe.draw_card();
            self.show_card(&card);
            split.hand.draw_card(card);

            self.splits.push(split);
            self.wager = wager;
            return;
        }

        wager.split_hand(&mut split);
        self.report.total_splits += 1;

        let card = shoe.draw_card();
        self.show_card(&card);
        wager.hand.draw_card(card);
        self.play_split(&mut wager, shoe, up);

        let card = shoe.draw_card();
        self.show_card(&card);
        split.hand.draw_card(card);
        self.play_split(&mut split, shoe, up);

        self.splits.push(split);
        self.wager = wager;
    }

    pub fn play_split(&mut self, wager: &mut Wager, shoe: &mut Shoe, up: &Card) {
        if wager.hand.is_pair() && self.strategy.get_split(&self.seen_cards, wager.hand.get_card_pair().unwrap(), up) {
            let mut split = Wager::new(MINIMUM_BET, MAXIMUM_BET);
            wager.split_hand(&mut split);
            self.report.total_splits += 1;

            let card = shoe.draw_card();
            self.show_card(&card);
            wager.hand.draw_card(card);
            self.play_split(wager, shoe, up);

            let card = shoe.draw_card();
            self.show_card(&card);
            split.hand.draw_card(card);
            self.play_split(&mut split, shoe, up);

            self.splits.push(split);
            return;
        }

        let mut do_stand = self.strategy.get_stand(&self.seen_cards, wager.hand.total, wager.hand.is_soft(), up);

        while !wager.hand.is_busted() && !do_stand {
            let card = shoe.draw_card();
            self.show_card(&card);
            wager.hand.draw_card(card);
            if !wager.hand.is_busted() {
                do_stand = self.strategy.get_stand(&self.seen_cards, wager.hand.total, wager.hand.is_soft(), up);
            }
        }
    }

    pub fn payoff(&mut self, dealer_blackjack: bool, dealer_busted: bool, dealer_total: usize) {
        if self.splits.is_empty() {
            self.payoff_hand(dealer_blackjack, dealer_busted, dealer_total);
            return;
        }

        Self::payoff_split(&mut self.report, &mut self.wager, dealer_busted, dealer_total);
        for split in &mut self.splits {
            Self::payoff_split(&mut self.report, split, dealer_busted, dealer_total);
        }
    }

    fn payoff_split(report: &mut Report, wager: &mut Wager, dealer_busted: bool, dealer_total: usize) {
        if wager.hand.is_busted() {
            wager.lost();
            report.total_loses += 1;
        } else if dealer_busted || wager.hand.total > dealer_total {
            wager.won();
            report.total_wins += 1;
        } else if dealer_total > wager.hand.total {
            wager.lost();
            report.total_loses += 1;
        } else {
            wager.push();
            report.total_pushes += 1;
        }

        report.total_won += wager.amount_won;
        report.total_bet += wager.amount_bet;
    }

    pub fn payoff_hand(&mut self, dealer_blackjack: bool, dealer_busted: bool, dealer_total: usize) {
        if dealer_blackjack {
            self.wager.won_insurance();
        } else {
            self.wager.lost_insurance();
        }

        if dealer_blackjack {
            if self.wager.hand.is_blackjack() {
                self.wager.push();
                self.report.total_pushes += 1;
            } else {
                self.wager.lost();
                self.report.total_loses += 1;
            }
        } else {
            if self.wager.hand.is_blackjack() {
                self.wager.won_blackjack(self.rules.blackjack_pays, self.rules.blackjack_bets);
            } else if self.wager.hand.is_busted() {
                self.wager.lost();
                self.report.total_loses += 1;
            } else if dealer_busted || (self.wager.hand.total > dealer_total) {
                self.wager.won();
                self.report.total_wins += 1;
            } else if dealer_total > self.wager.hand.total {
                self.wager.lost();
                self.report.total_loses += 1;
            } else {
                self.wager.push();
                self.report.total_pushes += 1;
            }
        }

        self.report.total_bet += self.wager.amount_bet + self.wager.insurance_bet;
        self.report.total_won += self.wager.amount_won + self.wager.insurance_won;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::arguments::Arguments;
    use crate::cards::{Card, Rank, Suit};
    use crate::strategy::rules::Rules;
    use crate::strategy::strategy::Strategy;
    use crate::utilities::utilities::Utility;

    #[test]
    fn test_place_bet_mimic() {
        let rules = Rules::default();
        let mut strategy = Strategy::new();
        let utility = Utility::default();
        strategy.init(&utility, &Arguments::new());
        let mut player = Player::new(&rules, &strategy);

        player.place_bet(true);
        assert_eq!(player.wager.amount_bet, MINIMUM_BET);
    }

    #[test]
    fn test_play_mimic() {
        let rules = Rules::default();
        let strategy = Strategy::new();
        let mut player = Player::new(&rules, &strategy);

        player.place_bet(true);
        player.wager.hand.draw_card(Some(Card::new(Rank::Ace, Suit::Hearts)));
        player.wager.hand.draw_card(Some(Card::new(Rank::Six, Suit::Hearts)));
        assert!(!player.mimic_stand());

        player.wager.hand.draw_card(Some(Card::new(Rank::Ten, Suit::Hearts)));
        assert!(player.mimic_stand());
    }

    fn mock_player() -> Player {
        let rules = Rules::default();
        let strategy = Strategy::new();
        let mut player = Player::new(&rules, &strategy);

        player.place_bet(true);
        player.wager.hand.draw_card(Some(Card::new(Rank::Ten, Suit::Hearts)));
        player.wager.hand.draw_card(Some(Card::new(Rank::Ten, Suit::Hearts)));

        player
    }

    #[test]
    fn test_payoff_player_busted() {
        let mut player = mock_player();

        player.wager.hand.draw_card(Some(Card::new(Rank::Ten, Suit::Hearts)));
        player.payoff(false, false, 21);

        assert_eq!(player.wager.amount_won, -(player.wager.amount_bet as isize));
    }

    #[test]
    fn test_payoff_dealer_busted() {
        let mut player = mock_player();
        player.payoff(false, true, 22);

        assert_eq!(player.wager.amount_won, player.wager.amount_bet as isize);
    }

    #[test]
    fn test_payoff_dealer_blackjack() {
        let mut player = mock_player();
        player.payoff(true, false, 21);

        assert_eq!(player.wager.amount_won, -(player.wager.amount_bet as isize));
    }

    #[test]
    fn test_payoff_dealer_21() {
        let mut player = mock_player();
        player.payoff(false, false, 21);
    }

    #[test]
    fn test_payoff_dealer_20() {
        let mut player = mock_player();
        player.payoff(false, false, 20);
    }

    #[test]
    fn test_payoff_dealer_19() {
        let mut player = mock_player();
        player.payoff(false, false, 19);
    }
}
