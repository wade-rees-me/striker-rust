use serde_json::Value;
use std::error::Error;

pub trait JsonFetcher {
    fn fetch_json(&self, url: &str) -> Result<Value, Box<dyn Error>>;
}

pub trait JsonSender {
    fn send_json(&self, url: &str, json_value: serde_json::Value) -> Result<Value, Box<dyn Error>>;
}
