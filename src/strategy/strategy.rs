use crate::Arguments;
use crate::cards::{Card, Rank};
use crate::constants::constants::{DECKS_SINGLE_DECK, NUMBER_OF_CARDS_IN_DECK, STRATEGY_MIMIC, TRUE_COUNT_BET, TRUE_COUNT_MULTIPLIER};
use crate::strategy::chart::Chart;
use crate::traits::traits::JsonFetcher;
use crate::utilities::utilities::get_charts_url;
use crate::xlog_panic;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Strategy {
    pub playbook: String,
    pub counts: Vec<isize>,
    pub insurance: String,
    pub soft_double: Chart,
    pub hard_double: Chart,
    pub pair_split: Chart,
    pub soft_stand: Chart,
    pub hard_stand: Chart,
    pub number_of_cards: usize,
    pub json_response: Value,
}

impl Strategy {
    pub fn new() -> Self {
        Strategy {
            playbook: format!("{}-{}", DECKS_SINGLE_DECK, STRATEGY_MIMIC),
            counts: vec![0; 12],
            insurance: "N".to_string(),
            soft_double: Chart::new("Soft Double"),
            hard_double: Chart::new("Hard Double"),
            pair_split: Chart::new("Pair Split"),
            soft_stand: Chart::new("Soft Stand"),
            hard_stand: Chart::new("Hard Stand"),
            number_of_cards: 52,
            json_response: Value::Null,
        }
    }

    pub fn init(&mut self, fetcher: &dyn JsonFetcher, arguments: &Arguments) {
        self.number_of_cards = arguments.number_of_decks * NUMBER_OF_CARDS_IN_DECK;

        if arguments.strategy.to_lowercase() != "mimic" {
            let url = format!(
                "http://{}/{}/{}",
                get_charts_url().expect("Missing strategy chart URL"),
                &arguments.decks,
                &arguments.strategy
            );
            match fetcher.fetch_json(&url) {
                Ok(json_value) => self.json_response = json_value,
                Err(e) => xlog_panic!("Error fetching JSON: {}", e),
            }
            self.fetch_table(self.json_response.clone());

            self.soft_double.print();
            self.hard_double.print();
            self.pair_split.print();
            self.soft_stand.print();
            self.hard_stand.print();
            self.print_count();
        }
    }

    fn fetch_table(&mut self, data: Value) {
        self.playbook = data["playbook"].as_str().unwrap_or_default().to_string();
        self.insurance = data["insurance"].as_str().unwrap_or_default().to_string();
        self.counts = data["counts"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_i64().unwrap_or(0) as isize).collect();

        self.counts.splice(0..0, vec![0, 0]); // Prepend two zeroes

        strategy_load_table(&data["soft-double"], &mut self.soft_double);
        strategy_load_table(&data["hard-double"], &mut self.hard_double);
        strategy_load_table(&data["pair-split"], &mut self.pair_split);
        strategy_load_table(&data["soft-stand"], &mut self.soft_stand);
        strategy_load_table(&data["hard-stand"], &mut self.hard_stand);
    }

    pub fn get_running_count(&self, seen_cards: &[usize]) -> isize {
        (0..=11).map(|i| self.counts[i] * seen_cards[i] as isize).sum()
    }

    pub fn get_true_count(&self, seen_cards: &[usize], running_count: isize) -> isize {
        let unseen: usize = self.number_of_cards - seen_cards.iter().sum::<usize>();
        if unseen > 0 {
            (running_count as f64 / (unseen as f64 / TRUE_COUNT_MULTIPLIER as f64)).floor() as isize
        } else {
            0
        }
    }

    pub fn get_bet(&self, seen_cards: &[usize]) -> usize {
        let true_count = self.get_true_count(seen_cards, self.get_running_count(seen_cards)).max(0);
        (true_count as usize) * TRUE_COUNT_BET
    }

    pub fn get_insurance(&self, seen_cards: &[usize]) -> bool {
        self.process_value(&self.insurance, self.get_true_count(seen_cards, self.get_running_count(seen_cards)), false)
    }

    pub fn get_double(&self, seen_cards: &[usize], total: usize, soft: bool, up: &Card) -> bool {
        let chart = if soft { &self.soft_double } else { &self.hard_double };
        let key = total.to_string();
        self.process_value(
            &chart.get_value_by_key(&key, up.rank.value()),
            self.get_true_count(seen_cards, self.get_running_count(seen_cards)),
            false,
        )
    }

    pub fn get_split(&self, seen_cards: &[usize], pair: &Card, up: &Card) -> bool {
        self.process_value(
            &self.pair_split.get_value_by_key(&pair.rank.key(), up.rank.value()),
            self.get_true_count(seen_cards, self.get_running_count(seen_cards)),
            false,
        )
    }

    pub fn get_stand(&self, seen_cards: &[usize], total: usize, soft: bool, up: &Card) -> bool {
        let chart = if soft { &self.soft_stand } else { &self.hard_stand };
        let key = total.to_string();
        self.process_value(
            &chart.get_value_by_key(&key, up.rank.value()),
            self.get_true_count(seen_cards, self.get_running_count(seen_cards)),
            true,
        )
    }

    pub fn print_count(&self) {
        println!("Counts\n--------------------2-----3-----4-----5-----6-----7-----8-----9-----X-----A---");
        print!("     ");
        for count in self.counts.iter().take(12) {
            print!("{:4}, ", count);
        }
        println!("\n------------------------------------------------------------------------------\n");
    }

    fn process_value(&self, value: &str, true_count: isize, default: bool) -> bool {
        match value.to_lowercase().as_str() {
            "yes" | "y" => true,
            "no" | "n" => false,
            v if v.starts_with('r') => true_count <= v[1..].parse::<isize>().unwrap_or_default(),
            v => v.parse::<isize>().map_or(default, |n| true_count >= n),
        }
    }
}

fn strategy_load_table(strategy: &Value, chart: &mut Chart) {
    if let Some(map) = strategy.as_object() {
        for (key, values) in map.iter() {
            if let Some(array) = values.as_array() {
                let mut index = Rank::Two.value();
                for value in array {
                    chart.insert(key, index, value.as_str().unwrap_or("---"));
                    index += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Rank, Suit};
    use crate::resources::single_deck_basic::SINGLE_DECK_BASIC_JSON;
    use std::env;
    use std::error::Error;

    #[derive(Default, Debug, Clone)]
    struct MockJsonFetcher;
    impl JsonFetcher for MockJsonFetcher {
        fn fetch_json(&self, _url: &str) -> Result<Value, Box<dyn Error>> {
            Ok(SINGLE_DECK_BASIC_JSON.clone())
        }
    }

    #[derive(Default, Debug, Clone)]
    struct MockJsonFetcherError;
    impl JsonFetcher for MockJsonFetcherError {
        fn fetch_json(&self, _url: &str) -> Result<Value, Box<dyn Error>> {
            Err("Cannot fetch json".into())
        }
    }

    fn mock_strategy() -> Strategy {
        let mock = MockJsonFetcher::default();
        let mut strategy = Strategy::new();
        let mut arguments = Arguments::default();

        arguments.strategy = "basic".to_string();
        unsafe {
            env::set_var("STRIKER_URL_CHARTS", "https://example.com/charts");
        }
        strategy.init(&mock, &arguments);
        unsafe {
            env::remove_var("STRIKER_URL_CHARTS");
        }
        strategy
    }

    #[test]
    #[should_panic]
    fn test_get_test_fetch_json_error() {
        let mock = MockJsonFetcherError::default();
        let mut strategy = Strategy::new();
        let mut arguments = Arguments::default();

        arguments.strategy = "basic".to_string();
        unsafe {
            env::set_var("STRIKER_URL_CHARTS", "https://example.com/charts");
        }
        strategy.init(&mock, &arguments);
        unsafe {
            env::remove_var("STRIKER_URL_CHARTS");
        }
    }

    #[test]
    fn test_get_true_count_nonzero_unseen() {
        let seen_cards = [0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 8, 2];
        let strategy = Strategy::new();
        let running_count = strategy.get_running_count(&seen_cards);
        let true_count = strategy.get_true_count(&seen_cards, running_count);
        assert_eq!(true_count, 0);
    }

    #[test]
    fn test_process_value_yes_no() {
        let strategy = Strategy::new();
        assert!(strategy.process_value("yes", 3, false));
        assert!(!strategy.process_value("no", 3, true));
    }

    #[test]
    fn test_process_value_threshold() {
        let strategy = Strategy::new();
        assert!(strategy.process_value("2", 3, false));
        assert!(!strategy.process_value("4", 3, false));
        assert!(strategy.process_value("r5", 3, false)); // reverse logic: true if TC <= 5
        assert!(!strategy.process_value("r1", 3, false));
    }

    #[test]
    fn test_get_bet_base_case() {
        let strategy = Strategy::new();
        let seen_cards = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(strategy.get_bet(&seen_cards), 0);
    }

    #[test]
    fn test_get_insurance() {
        let strategy = Strategy::new();
        let seen_cards = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(strategy.get_insurance(&seen_cards), false);
    }

    #[test]
    fn test_get_play_double() {
        let strategy = mock_strategy();
        let up = Card::new(Rank::Ten, Suit::Hearts);
        let seen_cards = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(strategy.get_double(&seen_cards, 20, false, &up), false);
    }

    #[test]
    fn test_get_play_split() {
        let strategy = mock_strategy();
        let up = Card::new(Rank::Ten, Suit::Hearts);
        let pair = Card::new(Rank::Ten, Suit::Hearts);
        let seen_cards = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(strategy.get_split(&seen_cards, &pair, &up), false);
    }

    #[test]
    fn test_get_play_stand() {
        let strategy = mock_strategy();
        let up = Card::new(Rank::Ten, Suit::Hearts);
        let seen_cards = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(strategy.get_stand(&seen_cards, 20, false, &up), true);
    }

    #[test]
    fn test_print_count_does_not_panic() {
        let strategy = Strategy::new();
        strategy.print_count(); // Just check it doesn't panic
    }
}
