use once_cell::sync::Lazy;
use serde_json::Value;
use serde_json::json;

#[allow(dead_code)]
pub static SINGLE_DECK_BASIC_JSON: Lazy<Value> = Lazy::new(|| {
    json!({
    "playbook": "single-deck-basic",
    "counts": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    "insurance": "N",
    "soft-double": {
      "12": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "13": [  "N",    "N",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "14": [  "N",    "N",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "15": [  "N",    "N",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "16": [  "N",    "N",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "17": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "18": [  "N",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "19": [  "N",    "N",    "N",    "N",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "20": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "21": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ]
    },
    "hard-double": {
       "4": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "5": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "6": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "7": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "8": [  "N",    "N",    "N",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
       "9": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "10": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N" ],
      "11": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "12": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "13": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "14": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "15": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "16": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "17": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "18": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "19": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "20": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "21": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ]
    },
    "pair-split": {
       "2": [  "N",    "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N" ],
       "3": [  "N",    "N",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N" ],
       "4": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "5": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "6": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
       "7": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N" ],
       "8": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
       "9": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "Y",    "Y",    "N",    "N" ],
       "X": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "A": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ]
    },
    "soft-stand": {
      "12": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "13": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "14": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "15": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "16": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "17": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "18": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N" ],
      "19": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "20": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "21": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ]
    },
    "hard-stand": {
       "4": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "5": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "6": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "7": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "8": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
       "9": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "10": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "11": [  "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N",    "N" ],
      "12": [  "N",    "N",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "13": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "14": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "15": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "16": [  "Y",    "Y",    "Y",    "Y",    "Y",    "N",    "N",    "N",    "N",    "N" ],
      "17": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "18": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "19": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "20": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ],
      "21": [  "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y",    "Y" ]
    }
      })
});

#[cfg(test)]
mod tests {
    use super::SINGLE_DECK_BASIC_JSON;
    use serde_json::Value;

    #[test]
    fn test_single_deck_basic_json_structure() {
        let val: &Value = &*SINGLE_DECK_BASIC_JSON;

        // Check root keys
        assert!(val.get("playbook").is_some(), "Missing 'playbook' key");
        assert_eq!(val["playbook"], "single-deck-basic");

        assert!(val.get("counts").is_some(), "Missing 'counts' key");
        assert_eq!(val["counts"].as_array().unwrap().len(), 10);

        assert_eq!(val["insurance"], "N");

        // Soft double rule for 13 vs dealer 2 is "N", dealer 4 is "Y"
        assert_eq!(val["soft-double"]["13"][0], "N");
        assert_eq!(val["soft-double"]["13"][2], "Y");

        // Hard double rule for 10 vs dealer 7 is "Y"
        assert_eq!(val["hard-double"]["10"][6], "Y");

        // Pair split rule for 8s vs dealer 9 is "Y"
        assert_eq!(val["pair-split"]["8"][8], "Y");

        // Soft stand for 18 vs dealer 8 is "N"
        assert_eq!(val["soft-stand"]["18"][7], "N");

        // Hard stand for 13 vs dealer 3 is "Y"
        assert_eq!(val["hard-stand"]["13"][2], "Y");
    }
}
