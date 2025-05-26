use crate::arguments::parameters::Parameters;
use crate::constants::constants::{BILLION, NUMBER_OF_HANDS_DATABASE, STRIKER_VERSION};
use crate::traits::traits::JsonSender;
use crate::utilities::utilities::get_simulations_url;
use crate::utilities::utilities::is_my_computer;
use num_format::{Locale, ToFormattedString};
use std::time::SystemTime;

#[derive(Default, Debug, Clone)]
pub struct Report {
    pub name: String,
    pub version: String,
    pub simulator: String,
    pub playbook: String,
    pub strategy: String,
    pub decks: String,
    pub epoch: String,
    pub total_rounds: usize,
    pub total_hands: usize,
    pub total_bet: usize,
    pub total_won: isize,
    pub total_blackjacks: usize,
    pub total_doubles: usize,
    pub total_splits: usize,
    pub total_splits_ace: usize,
    pub total_wins: usize,
    pub total_loses: usize,
    pub total_pushes: usize,
    pub out_of_cards: usize,
    pub total_shuffles: usize,
    pub total_threads: usize,
    pub start: usize,
    pub end: usize,
    pub duration: usize,
    pub advantage: f64,
    pub per_billion: f64,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, parameters: &Parameters) {
        self.name = parameters.name.clone();
        self.version = STRIKER_VERSION.to_string();
        self.simulator = parameters.processor.clone();
        self.playbook = parameters.playbook.clone();
        self.strategy = parameters.strategy.clone();
        self.decks = parameters.decks.clone();
        self.epoch = parameters.epoch.clone();
        self.total_threads = parameters.number_of_threads;
        self.start = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;
    }

    pub fn merge(&mut self, other: &Report) {
        self.total_rounds += other.total_rounds;
        self.total_hands += other.total_hands;
        self.total_bet += other.total_bet;
        self.total_won += other.total_won;
        self.total_blackjacks += other.total_blackjacks;
        self.total_doubles += other.total_doubles;
        self.total_splits += other.total_splits;
        self.total_splits_ace += other.total_splits_ace;
        self.total_wins += other.total_wins;
        self.total_loses += other.total_loses;
        self.total_pushes += other.total_pushes;
        self.total_shuffles += other.total_shuffles;
        self.out_of_cards += other.out_of_cards;
    }

    pub fn finish(&mut self) {
        self.end = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;
        self.duration = self.end - self.start;
        self.advantage = (self.total_won as f64 / self.total_bet as f64) * 100.0;
        self.per_billion = self.duration as f64 * BILLION as f64 / self.total_hands as f64;
    }

    pub fn insert(&mut self, sender: &dyn JsonSender) {
        if !is_my_computer() {
            println!("    This code is restricted to running only on my computer.");
            return;
        }

        if self.total_hands < NUMBER_OF_HANDS_DATABASE {
            println!(
                "    Error: Not enough hands played ({}). Minimum required is {}",
                self.total_hands.to_formatted_string(&Locale::en),
                NUMBER_OF_HANDS_DATABASE.to_formatted_string(&Locale::en)
            );
            return;
        }

        let json = self.to_json_object();
        let url = format!(
            "http://{}/{}/{}/{}",
            get_simulations_url().expect("Missing simulation URL"),
            self.simulator,
            self.playbook,
            self.name
        );
        match sender.send_json(&url, json) {
            Ok(value) => {
                if let Some(status) = value.get("status") {
                    if status == "success" {
                        println!("Insert successful");
                    } else {
                        println!("Request failed with status: {}", status);
                    }
                }
            }
            Err(e) => {
                println!("HTTP error: {}", e);
            }
        }
    }

    pub fn to_json_object(&self) -> serde_json::Value {
        serde_json::json!({
            "guid": self.name,
            "version": STRIKER_VERSION,
            "simulator": self.simulator,
            "threads": self.total_threads,
            "playbook": self.playbook,
            "decks": self.decks,
            "strategy": self.strategy,
            "rounds": self.total_rounds,
            "hands": self.total_hands,
            "out_of_cards": self.out_of_cards,
            "total_shuffles": self.total_shuffles,
            "total_bet": self.total_bet,
            "total_won": self.total_won,
            "total_blackjacks": self.total_blackjacks,
            "total_doubles": self.total_doubles,
            "total_splits": self.total_splits,
            "total_splits_ace": self.total_splits_ace,
            "total_wins": self.total_wins,
            "total_loses": self.total_loses,
            "total_pushes": self.total_pushes,
            "advantage": self.advantage,
            "epoch": self.epoch,
            "start": self.start,
            "end": self.end,
            "duration": self.duration,
            "per_billion": self.per_billion,
        })
    }

    pub fn print(&self) {
        println!("    {:<26}: {:>17}", "Number of hands", self.total_hands.to_formatted_string(&Locale::en));
        println!("    {:<26}: {:>17}", "Number of rounds", self.total_rounds.to_formatted_string(&Locale::en));
        println!("    {:<26}: {:>17}", "Number of shuffles", self.total_shuffles.to_formatted_string(&Locale::en));
        println!("    {:<26}: {:>17}", "Out of cards", self.out_of_cards.to_formatted_string(&Locale::en));
        println!(
            "    {:<26}: {:>17} {:+08.3} average bet per hand",
            "Total bet",
            self.total_bet.to_formatted_string(&Locale::en),
            self.total_bet as f64 / self.total_hands as f64
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} average win per hand",
            "Total won",
            self.total_won.to_formatted_string(&Locale::en),
            self.total_won as f64 / self.total_hands as f64
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of blackjacks",
            self.total_blackjacks.to_formatted_string(&Locale::en),
            self.total_blackjacks as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of doubles",
            self.total_doubles.to_formatted_string(&Locale::en),
            self.total_doubles as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of splits",
            self.total_splits.to_formatted_string(&Locale::en),
            self.total_splits as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of splits - Aces",
            self.total_splits_ace.to_formatted_string(&Locale::en),
            self.total_splits_ace as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of wins",
            self.total_wins.to_formatted_string(&Locale::en),
            self.total_wins as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of pushes",
            self.total_pushes.to_formatted_string(&Locale::en),
            self.total_pushes as f64 / self.total_hands as f64 * 100.0
        );
        println!(
            "    {:<26}: {:>17} {:+08.3} % of total hands",
            "Number of loses",
            self.total_loses.to_formatted_string(&Locale::en),
            self.total_loses as f64 / self.total_hands as f64 * 100.0
        );
        println!("    {:<26}: {:>17} seconds", "Total time", self.duration.to_formatted_string(&Locale::en));
        println!("    {:<26}: {:>17} threads", "Number of threads", self.total_threads.to_formatted_string(&Locale::en));
        println!(
            "    {:<26}: {:>17} seconds per {} hands",
            "Average time",
            ((self.duration as f64 * BILLION as f64 / (self.total_hands as f64)) as i64).to_formatted_string(&Locale::en),
            BILLION.to_formatted_string(&Locale::en)
        );
        println!("    {:<26}: {:>17} {:+08.3} %", "Player advantage", "", self.advantage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::arguments::Arguments;
    use crate::arguments::parameters::Parameters;
    use serde_json::Value;
    use serde_json::json;
    use std::env;
    use std::error::Error;

    #[derive(Default, Debug, Clone)]
    struct MockJsonSender;
    impl JsonSender for MockJsonSender {
        fn send_json(&self, _url: &str, _json_value: Value) -> Result<Value, Box<dyn Error>> {
            let mock_response = json!({
                "status": "success",
                "message": "This is a mock response"
            });
            Ok(mock_response)
        }
    }
    #[derive(Default, Debug, Clone)]
    struct MockJsonSenderError;
    impl JsonSender for MockJsonSenderError {
        fn send_json(&self, _url: &str, _json_value: Value) -> Result<Value, Box<dyn Error>> {
            let mock_response = json!({
                "status": "failed",
                "message": "This is a mock response"
            });
            Ok(mock_response)
        }
    }

    #[test]
    fn test_parameters_print_does_not_panic() {
        let report = Report::default();
        report.print();
    }

    #[test]
    fn test_parameters_to_json() {
        let report = Report::default();
        let _json = report.to_json_object();
    }

    #[test]
    fn test_report_insert() {
        let mock = MockJsonSender::default();
        let mut report = Report::default();
        unsafe {
            env::set_var("STRIKER_URL_SIMULATIONS", "https://example.com/simulations");
        }
        report.insert(&mock);
        report.total_hands = BILLION;
        report.insert(&mock);
        unsafe {
            env::remove_var("STRIKER_URL_SIMULATIONS");
        }
    }

    #[test]
    fn test_report_insert_error() {
        let mock = MockJsonSenderError::default();
        let mut report = Report::default();
        unsafe {
            env::set_var("STRIKER_URL_SIMULATIONS", "https://example.com/simulations");
        }
        report.insert(&mock);
        report.total_hands = BILLION;
        report.insert(&mock);
        unsafe {
            env::remove_var("STRIKER_URL_SIMULATIONS");
        }
    }

    #[test]
    fn test_report_init_merge_finish_to_json() {
        let args = Arguments::new();
        let parameters = Parameters::new(&args);
        let mut report1 = Report::default();
        let mut report2 = Report::default();
        let test_count = 1000;

        report1.init(&parameters);

        report2.total_rounds = test_count;
        report2.total_hands = test_count;
        report2.total_bet = test_count;
        report2.total_won = test_count as isize;
        report2.total_blackjacks = test_count;
        report2.total_doubles = test_count;
        report2.total_splits = test_count;
        report2.total_splits_ace = test_count;
        report2.total_wins = test_count;
        report2.total_loses = test_count;
        report2.total_pushes = test_count;

        report1.merge(&report2);
        report1.start -= 20;
        report1.finish();

        assert_eq!(report1.total_rounds, test_count);
        assert_eq!(report1.total_hands, test_count);
        assert_eq!(report1.total_bet, test_count);
        assert_eq!(report1.total_won, test_count as isize);
        assert_eq!(report1.total_blackjacks, test_count);
        assert_eq!(report1.total_doubles, test_count);
        assert_eq!(report1.total_splits, test_count);
        assert_eq!(report1.total_splits_ace, test_count);
        assert_eq!(report1.total_wins, test_count);
        assert_eq!(report1.total_loses, test_count);
        assert_eq!(report1.total_pushes, test_count);

        assert!(report1.duration > 0);
        assert!(report1.advantage > 0.0);
        assert!(report1.per_billion > 0.0);

        let json = report1.to_json_object();
        assert_eq!(json["total_bet"], test_count);
        assert_eq!(json["total_won"], test_count);
        assert_eq!(json["total_blackjacks"], test_count);
        assert_eq!(json["total_doubles"], test_count);
        assert_eq!(json["total_splits"], test_count);
        assert_eq!(json["total_splits_ace"], test_count);
        assert_eq!(json["total_wins"], test_count);
        assert_eq!(json["total_loses"], test_count);
        assert_eq!(json["total_pushes"], test_count);
        assert!(json["per_billion"].as_f64().unwrap() > 0.0);
    }
}
