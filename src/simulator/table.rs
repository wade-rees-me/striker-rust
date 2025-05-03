use crate::{
    arguments::parameters::Parameters, cards::card::Card, cards::dealer::Dealer, cards::shoe::Shoe, constants::constants::STATUS_ROUNDS, player::player::Player,
    report::report::Report, strategy::rules::Rules, strategy::strategy::Strategy,
};
use num_format::{Locale, ToFormattedString};
use std::io::{self, Write};

pub struct Table {
    parameters: Parameters,
    shoe: Shoe,
    dealer: Dealer,
    player: Player,
    report: Report,
    up: Option<Card>,
    down: Option<Card>,
}

impl Table {
    pub fn new(parameters: Parameters, rules: Rules, strategy: Strategy) -> Self {
        Self {
            player: Player::new(&rules, &strategy),
            shoe: Shoe::new(parameters.number_of_decks, rules.penetration),
            dealer: Dealer::new(rules.hit_soft_17),
            report: Report::new(),
            up: None,
            down: None,
            parameters,
        }
    }

    pub fn session(&mut self, mimic: bool) {
        while self.report.total_hands < self.parameters.share_of_hands {
            if self.parameters.verbose {
                self.print_status(self.report.total_rounds, self.report.total_hands);
            }

            self.shoe.shuffle();
            self.player.shuffle();
            self.report.total_rounds += 1;

            while !self.shoe.should_shuffle() {
                self.report.total_hands += 1;
                self.dealer.hand.reset();
                self.player.place_bet(mimic);
                self.deal_cards();

                if !mimic && self.up.as_ref().is_some_and(|card| card.is_ace()) {
                    self.player.insurance();
                }

                if !self.dealer.hand.is_blackjack() {
                    self.player.play(self.up.as_ref().unwrap(), &mut self.shoe, mimic);

                    if !self.player.busted_or_blackjack() {
                        while !self.dealer.should_stand() {
                            let card = self.shoe.draw_card();
                            self.dealer.hand.draw_card(card.clone());
                            self.show_card(&card);
                        }
                    }
                }

                let down = self.down.clone(); // Immutable borrow ends here
                self.show_card(&down); // Safe to mutably borrow self
                //self.show_card(&self.down);
                self.player.payoff(self.dealer.hand.is_blackjack(), self.dealer.hand.is_busted(), self.dealer.hand.total);
            }
        }

        if self.parameters.verbose {
            print!("\r");
        }

        let player_report = self.player.get_report();
        self.report.out_of_cards = self.shoe.out_of_cards;
        self.report.total_shuffles = self.shoe.number_of_shuffles;
        self.report.merge(&player_report);
    }

    pub fn deal_cards(&mut self) {
        self.player.draw_card(self.shoe.draw_card());
        self.down = self.shoe.draw_card();
        self.dealer.hand.draw_card(self.down.clone());
        self.player.draw_card(self.shoe.draw_card());
        self.up = self.shoe.draw_card();
        self.dealer.hand.draw_card(self.up.clone());
        let up = self.up.clone(); // Immutable borrow ends here
        self.show_card(&up); // Safe to mutably borrow self
    }

    pub fn show_card(&mut self, card: &Option<Card>) {
        self.player.show_card(card);
    }

    pub fn get_report(&self) -> &Report {
        &self.report
    }

    fn print_status(&self, round: usize, hand: usize) {
        if round % STATUS_ROUNDS == 0 {
            print!(
                "\r    Rounds: [{:>13}] Hands [{:>13}]: Simulating...",
                round.to_formatted_string(&Locale::en),
                hand.to_formatted_string(&Locale::en),
            );
            io::stdout().flush().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::arguments::Arguments;
    use crate::cards::{Rank, Suit};

    fn make_table() -> Table {
        let arguments = Arguments::new();
        let parameters = Parameters::new(&arguments);
        let rules = Rules::new();
        let strategy = Strategy::new();
        Table::new(parameters.clone(), rules.clone(), strategy.clone())
    }

    #[test]
    fn test_table_initialization() {
        let table = make_table();
        assert_eq!(table.report.total_hands, 0);
        assert!(table.up.is_none());
        assert!(table.down.is_none());
        table.print_status(0, 0);
        table.print_status(1, 0);
    }

    #[test]
    fn test_deal_cards_sets_up_and_down() {
        let mut table = make_table();
        table.deal_cards();
        assert!(table.up.is_some());
        assert!(table.down.is_some());
    }

    #[test]
    fn test_show_card_updates_seen_cards() {
        let mut table = make_table();
        let card = Some(Card::new(Rank::Ten, Suit::Clubs));
        table.show_card(&card);
    }
}
