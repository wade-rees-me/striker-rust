use crate::traits::traits::JsonFetcher;
use crate::utilities::utilities::get_rules_url;
use crate::xlog_panic;
use serde_json::Value;
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Rules {
    pub playbook: String,
    pub hit_soft_17: bool,
    pub surrender: bool,
    pub double_any_two_cards: bool,
    pub double_after_split: bool,
    pub resplit_aces: bool,
    pub hit_split_aces: bool,
    pub blackjack_bets: usize,
    pub blackjack_pays: usize,
    pub penetration: f64,
    json_response: Value,
}

impl Rules {
    pub fn new() -> Self {
        Rules { ..Default::default() }
    }

    // This method fetches the JSON data and parses it separately
    pub fn init(&mut self, fetcher: &dyn JsonFetcher, decks: &str) {
        let url = format!("http://{}/{}", get_rules_url().expect("Missing rules URL"), decks);
        match fetcher.fetch_json(&url) {
            Ok(json_value) => self.json_response = json_value,
            Err(e) => xlog_panic!("Error fetching JSON: {}", e),
        }
        self.fetch_table();
    }

    // Extract values from JSON and set member variables
    fn fetch_table(&mut self) {
        if let Some(playbook) = self.json_response.get("playbook").and_then(|v| v.as_str()) {
            self.playbook = playbook.to_string();
        }

        self.hit_soft_17 = self.json_response.get("hitSoft17").and_then(|v| v.as_bool()).unwrap();
        self.surrender = self.json_response.get("surrender").and_then(|v| v.as_bool()).unwrap();
        self.double_any_two_cards = self.json_response.get("doubleAnyTwoCards").and_then(|v| v.as_bool()).unwrap();
        self.double_after_split = self.json_response.get("doubleAfterSplit").and_then(|v| v.as_bool()).unwrap();
        self.resplit_aces = self.json_response.get("resplitAces").and_then(|v| v.as_bool()).unwrap();
        self.hit_split_aces = self.json_response.get("hitSplitAces").and_then(|v| v.as_bool()).unwrap();
        self.blackjack_bets = self.json_response.get("blackjackBets").and_then(|v| v.as_i64()).unwrap() as usize;
        self.blackjack_pays = self.json_response.get("blackjackPays").and_then(|v| v.as_i64()).unwrap() as usize;
        self.penetration = self.json_response.get("penetration").and_then(|v| v.as_f64()).unwrap();
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:>4}{}", "", "Table Rules")?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Table", self.playbook)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Hit soft 17", self.hit_soft_17)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Surrender", self.surrender)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Double any two cards", self.double_any_two_cards)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Double after split", self.double_after_split)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Re-split aces", self.resplit_aces)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Hit split aces", self.hit_split_aces)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Blackjack bets", self.blackjack_bets)?;
        writeln!(f, "{:>6}{:<24}: {}", "", "Blackjack pays", self.blackjack_pays)?;
        write!(f, "{:>6}{:<24}: {:.3} %", "", "Penetration", self.penetration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::utilities::Utility;
    use serde_json::json;
    use std::env;

    #[test]
    #[should_panic]
    fn test_rules_table_fail() {
        let utility = Utility::default();
        let mut rules = Rules::new();
        unsafe {
            env::set_var("STRIKER_URL_RULES", "https://example.com/charts");
        }
        rules.init(&utility, "single-deck");
        unsafe {
            env::remove_var("STRIKER_URL_RULES");
        }
    }

    #[test]
    fn test_rules_table() {
        let mut rules = Rules::new();
        rules.json_response = json!({
            "playbook": "single-deck",
            "hitSoft17": false,
            "surrender": true,
            "doubleAnyTwoCards": false,
            "doubleAfterSplit": true,
            "resplitAces": true,
            "hitSplitAces": true,
            "blackjackBets": 2,
            "blackjackPays": 3,
            "penetration": 0.75
        });
        rules.fetch_table();

        assert_eq!(rules.playbook, "single-deck");
        assert_eq!(rules.hit_soft_17, false);
        assert_eq!(rules.surrender, true);
        assert_eq!(rules.double_any_two_cards, false);
        assert_eq!(rules.double_after_split, true);
        assert_eq!(rules.resplit_aces, true);
        assert_eq!(rules.hit_split_aces, true);
        assert_eq!(rules.blackjack_bets, 2);
        assert_eq!(rules.blackjack_pays, 3);
        assert!((rules.penetration - 0.75).abs() < 1e-6);

        println!("{}", rules);
    }
}
