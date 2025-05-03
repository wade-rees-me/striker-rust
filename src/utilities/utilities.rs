use crate::traits::traits::{JsonFetcher, JsonSender};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::string::String;

#[derive(Default, Debug, Clone)]
pub struct Utility {}
impl JsonFetcher for Utility {
    fn fetch_json(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let response = Client::new().get(url).send()?.text()?;
        let cleaned = strip_quotes(&response);
        let json_response: Value = serde_json::from_str(&cleaned)?;
        Ok(json_response)
    }
}
impl JsonSender for Utility {
    fn send_json(&self, url: &str, json_value: Value) -> Result<Value, Box<dyn Error>> {
        let response = reqwest::blocking::Client::new().post(url).json(&json_value).send();
        match response {
            Ok(res) => {
                let json_response: Value = res.json()?;
                Ok(json_response)
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}

// Function declarations for getting environment variables
pub fn get_rules_url() -> Option<String> {
    env::var("STRIKER_URL_RULES").ok()
}

pub fn get_charts_url() -> Option<String> {
    env::var("STRIKER_URL_CHARTS").ok()
}

pub fn get_simulations_url() -> Option<String> {
    env::var("STRIKER_URL_SIMULATIONS").ok()
}

//
pub fn strip_quotes(input: &str) -> String {
    input.trim_matches('"').replace("\\n", "").replace("\\", "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_getters() {
        unsafe {
            env::set_var("STRIKER_URL_RULES", "https://example.com/rules");
            env::set_var("STRIKER_URL_CHARTS", "https://example.com/charts");
            env::set_var("STRIKER_URL_SIMULATIONS", "https://example.com/simulations");
        }
        assert_eq!(get_simulations_url(), Some("https://example.com/simulations".to_string()));
        assert_eq!(get_charts_url(), Some("https://example.com/charts".to_string()));
        assert_eq!(get_rules_url(), Some("https://example.com/rules".to_string()));

        unsafe {
            env::remove_var("STRIKER_URL_SIMULATIONS");
            env::remove_var("STRIKER_URL_CHARTS");
            env::remove_var("STRIKER_URL_RULES");
        }
        assert_eq!(get_simulations_url(), None);
        assert_eq!(get_charts_url(), None);
        assert_eq!(get_rules_url(), None);
    }

    #[test]
    fn test_strip_quotes() {
        assert_eq!(strip_quotes("\"hello\""), "hello");
        assert_eq!(strip_quotes("hello"), "hello");
        assert_eq!(strip_quotes("\"123\""), "123");
    }
}
