use once_cell::sync::Lazy;
use serde_json::Value;
use serde_json::json;

#[allow(dead_code)]
pub static RULES_JSON: Lazy<Value> = Lazy::new(|| {
    json!({
        "playbook": "single-deck",
        "hitSoft17": true,
        "surrender": false,
        "doubleAnyTwoCards": true,
        "doubleAfterSplit": false,
        "resplitAces": false,
        "hitSplitAces": false,
        "blackjackBets": 2,
        "blackjackPays": 3,
        "penetration": 0.75
    })
});

#[cfg(test)]
mod tests {
    use super::RULES_JSON;

    #[test]
    fn test_rules_json_contents() {
        assert_eq!(RULES_JSON["playbook"], "single-deck");
        assert_eq!(RULES_JSON["hitSoft17"], true);
        assert_eq!(RULES_JSON["surrender"], false);
        assert_eq!(RULES_JSON["doubleAnyTwoCards"], true);
        assert_eq!(RULES_JSON["doubleAfterSplit"], false);
        assert_eq!(RULES_JSON["resplitAces"], false);
        assert_eq!(RULES_JSON["hitSplitAces"], false);
        assert_eq!(RULES_JSON["blackjackBets"], 2);
        assert_eq!(RULES_JSON["blackjackPays"], 3);
        let penetration = RULES_JSON["penetration"].as_f64().unwrap();
        assert!((penetration - 0.75).abs() < 1e-6);
    }
}
