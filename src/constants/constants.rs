// General constants
pub const STRIKER_WHO_AM_I: &str = "striker-rust";
pub const STRIKER_VERSION: &str = "v3.1.0"; // Epoch.Major.Minor
pub const TIME_LAYOUT: &str = "%Y-%m-%d %H:%M:%S %z";
pub const STATUS_ROUNDS: usize = 1000000;
pub const MY_HOSTNAME: &str = "Striker";

// Simulation constants
pub const MILLION: usize = 1000000;
pub const BILLION: usize = MILLION * 1000;
pub const NUMBER_OF_HANDS_MAXIMUM: usize = 10 * BILLION;
pub const NUMBER_OF_HANDS_MINIMUM: usize = 100;
pub const NUMBER_OF_HANDS_DEFAULT: usize = 500 * MILLION;
pub const NUMBER_OF_HANDS_DATABASE: usize = 10 * MILLION;
pub const NUMBER_OF_CARDS_IN_DECK: usize = 52;
pub const NUMBER_OF_CORES_MINIMUM: usize = 1;
pub const NUMBER_OF_CORES_PHYSICAL: usize = 24;
pub const NUMBER_OF_CORES_LOGICAL: usize = 32;
pub const NUMBER_OF_CORES_DEFAULT: usize = NUMBER_OF_CORES_PHYSICAL;
pub const NUMBER_OF_CORES_MAXIMUM: usize = NUMBER_OF_CORES_LOGICAL;

//
pub const STRATEGY_MIMIC: &str = "mimic";
pub const STRATEGY_BASIC: &str = "basic";
pub const STRATEGY_NEURAL: &str = "neural";
pub const STRATEGY_LINEAR: &str = "linear";
pub const STRATEGY_POLYNOMIAL: &str = "polynomial";
pub const STRATEGY_HIGH_LOW: &str = "high-low";
pub const STRATEGY_WONG: &str = "wong";
pub const DECKS_SINGLE_DECK: &str = "single-deck";
pub const DECKS_DOUBLE_DECK: &str = "double-deck";
pub const DECKS_SIX_SHOE: &str = "six-shoe";

// Betting constants
pub const MINIMUM_BET: usize = 2;
pub const MAXIMUM_BET: usize = 20;
pub const TRUE_COUNT_BET: usize = 2;
pub const TRUE_COUNT_MULTIPLIER: usize = 26;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_values() {
        assert_eq!(STRIKER_WHO_AM_I, "striker-rust");
        assert_eq!(STRIKER_VERSION, "v3.00.00");
        assert_eq!(TIME_LAYOUT, "%Y-%m-%d %H:%M:%S %z");
        assert_eq!(STATUS_ROUNDS, 1000000);

        assert_eq!(MILLION, 1000000);
        assert_eq!(BILLION, 1000000000);
        assert_eq!(NUMBER_OF_HANDS_MAXIMUM, 10 * BILLION);
        assert_eq!(NUMBER_OF_HANDS_MINIMUM, 100);
        assert_eq!(NUMBER_OF_HANDS_DEFAULT, 500 * MILLION);
        assert_eq!(NUMBER_OF_HANDS_DATABASE, 10 * MILLION);
        assert_eq!(NUMBER_OF_CARDS_IN_DECK, 52);
        assert_eq!(NUMBER_OF_CORES_PHYSICAL, 24);
        assert_eq!(NUMBER_OF_CORES_LOGICAL, 32);
        assert_eq!(NUMBER_OF_CORES_DEFAULT, 24);

        assert_eq!(MINIMUM_BET, 2);
        assert_eq!(MAXIMUM_BET, 20);
        assert_eq!(TRUE_COUNT_BET, 2);
        assert_eq!(TRUE_COUNT_MULTIPLIER, 26);
    }
}
