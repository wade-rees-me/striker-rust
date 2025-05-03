use crate::constants::constants::*;
use std::env;
use std::process;

pub struct Arguments {
    pub strategy: String,
    pub decks: String,
    pub number_of_decks: usize,
    pub number_of_hands: usize,
    pub number_of_threads: usize,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            strategy: STRATEGY_MIMIC.to_string(),
            decks: DECKS_SINGLE_DECK.to_string(),
            number_of_decks: 1,
            number_of_hands: NUMBER_OF_HANDS_DEFAULT,
            number_of_threads: NUMBER_OF_CORES_DEFAULT,
        }
    }
}

impl Arguments {
    pub fn new() -> Self {
        let argv: Vec<String> = env::args().collect();
        Self::parse_args(&argv)
    }

    #[allow(dead_code)]
    pub fn make_args(args: &[&str]) -> Self {
        let owned: Vec<String> = std::iter::once("program".into())
            .chain(args.iter().filter(|s| !s.is_empty()).map(|s| s.to_string()))
            .collect();
        Self::parse_args(&owned)
    }

    pub fn parse_args(argv: &[String]) -> Self {
        let mut args: Arguments = Default::default();
        let mut i = 1;

        while i < argv.len() {
            match argv[i].as_str() {
                "-h" | "--number-of-hands" => {
                    args.number_of_hands = Arguments::parse_usize_arg(argv, &mut i, NUMBER_OF_HANDS_MINIMUM, NUMBER_OF_HANDS_MAXIMUM, "number of hands");
                }
                "-t" | "--number-of-threads" => {
                    args.number_of_threads = Arguments::parse_usize_arg(argv, &mut i, NUMBER_OF_CORES_MINIMUM, NUMBER_OF_CORES_MAXIMUM, "number of threads");
                }
                "-M" | "--mimic" => {
                    args.strategy = STRATEGY_MIMIC.to_string();
                }
                "-B" | "--basic" => {
                    args.strategy = STRATEGY_BASIC.to_string();
                }
                "-L" | "--linear" => {
                    args.strategy = STRATEGY_LINEAR.to_string();
                }
                "-P" | "--polynomial" => {
                    args.strategy = STRATEGY_POLYNOMIAL.to_string();
                }
                "-N" | "--neural" => {
                    args.strategy = STRATEGY_NEURAL.to_string();
                }
                "-H" | "--high-low" => {
                    args.strategy = STRATEGY_HIGH_LOW.to_string();
                }
                "-W" | "--wong" => {
                    args.strategy = STRATEGY_WONG.to_string();
                }
                "-1" | "--single-deck" => {
                    args.decks = DECKS_SINGLE_DECK.to_string();
                    args.number_of_decks = 1;
                }
                "-2" | "--double-deck" => {
                    args.decks = DECKS_DOUBLE_DECK.to_string();
                    args.number_of_decks = 2;
                }
                "-6" | "--six-shoe" => {
                    args.decks = DECKS_SIX_SHOE.to_string();
                    args.number_of_decks = 6;
                }
                "--help" => {
                    Arguments::print_help_message();
                    process::exit(0);
                }
                "--version" => {
                    println!("{}: version: {}", STRIKER_WHO_AM_I, STRIKER_VERSION);
                    process::exit(0);
                }
                _ => {
                    Arguments::print_help_message();
                    panic!("Error: Invalid argument: {}", argv[i]);
                }
            }
            i += 1;
        }

        args
    }

    fn parse_usize_arg(argv: &[String], i: &mut usize, min: usize, max: usize, field_name: &str) -> usize {
        *i += 1;
        if *i >= argv.len() {
            panic!("Missing {}", field_name);
        }
        let value = argv[*i].replace(",", "");
        let parsed = value.parse().unwrap_or_else(|_| {
            panic!("Invalid {}", field_name);
        });
        if parsed < min || parsed > max {
            panic!("{} must be between {} and {}", field_name, min, max);
        }
        parsed
    }

    fn print_help_message() {
        println!("Usage: strikerRust++ [options]\n");
        println!("Options:");
        println!("  --help                                       Show this help message");
        println!("  --version                                    Display the program version");
        println!("  -h, --number-of-hands <number of hands>      The number of hands to play in this simulation");
        println!("  -t, --number-of-threads <number of threads>  The number of threads to use in this simulation");
        println!("  -M, --mimic                                  Use the mimic dealer player strategy");
        println!("  -B, --basic                                  Use the basic player strategy");
        println!("  -N, --neural                                 Use the neural player strategy");
        println!("  -L, --linear                                 Use the linear regression player strategy");
        println!("  -P, --polynomial                             Use the polynomial regression player strategy");
        println!("  -H, --high-low                               Use the high low count player strategy");
        println!("  -W, --wong                                   Use the Wong count player strategy");
        println!("  -1, --single-deck                            Use a single deck of cards and rules");
        println!("  -2, --double-deck                            Use a double deck of cards and rules");
        println!("  -6, --six-shoe                               Use a six deck shoe of cards and rules");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::str;
    use test_case::test_case;

    #[test]
    fn test_new() {
        Arguments::new();
    }

    #[test]
    fn test_help_flags() {
        Arguments::make_args(&["--help"]);
    }

    #[test]
    fn test_version_flags() {
        Arguments::make_args(&["--version"]);
    }

    #[rstest]
    #[case("--mimic", "-M", STRATEGY_MIMIC)]
    #[case("--basic", "-B", STRATEGY_BASIC)]
    #[case("--linear", "-L", STRATEGY_LINEAR)]
    #[case("--polynomial", "-P", STRATEGY_POLYNOMIAL)]
    #[case("--neural", "-N", STRATEGY_NEURAL)]
    #[case("--high-low", "-H", STRATEGY_HIGH_LOW)]
    #[case("--wong", "-W", STRATEGY_WONG)]
    fn test_strategy_flags(#[case] flag: &str, #[case] alias: &str, #[case] expected: &str) {
        let arguments = Arguments::make_args(&[flag]);
        assert_eq!(arguments.strategy, expected);

        let arguments = Arguments::make_args(&[alias]);
        assert_eq!(arguments.strategy, expected);
    }

    #[rstest]
    #[case("--single-deck", "-1", DECKS_SINGLE_DECK, 1)]
    #[case("--double-deck", "-2", DECKS_DOUBLE_DECK, 2)]
    #[case("--six-shoe", "-6", DECKS_SIX_SHOE, 6)]
    fn test_deck_flags(#[case] flag: &str, #[case] alias: &str, #[case] expected: &str, #[case] count: usize) {
        let arguments = Arguments::make_args(&[flag]);
        assert_eq!(arguments.decks, expected);
        assert_eq!(arguments.number_of_decks, count);

        let arguments = Arguments::make_args(&[alias]);
        assert_eq!(arguments.decks, expected);
        assert_eq!(arguments.number_of_decks, count);
    }

    #[rstest]
    #[case("--number-of-hands", MILLION)]
    #[case("-h", MILLION)]
    fn test_number_of_hands(#[case] flag: &str, #[case] expected: usize) {
        let arguments = Arguments::make_args(&[flag, &expected.to_string()]);
        assert_eq!(arguments.number_of_hands, expected);
    }

    #[rstest]
    #[case("--number-of-threads", 2)]
    #[case("-t", 2)]
    fn test_number_of_threads(#[case] flag: &str, #[case] expected: usize) {
        let arguments = Arguments::make_args(&[flag, &expected.to_string()]);
        assert_eq!(arguments.number_of_threads, expected);
    }

    #[test_case("--number-of-threads", ""; "invalid threads missing")]
    #[test_case("--number-of-threads", "empty"; "invalid threads empty")]
    #[test_case("--number-of-threads", &(NUMBER_OF_CORES_MAXIMUM + 1).to_string(); "invalid threads maximum")]
    #[test_case("--number-of-threads", &(NUMBER_OF_CORES_MINIMUM - 1).to_string(); "invalid threads minimum")]
    #[test_case("--number-of-hands", ""; "invalid hands missing")]
    #[test_case("--number-of-hands", "empty"; "invalid hands empty")]
    #[test_case("--number-of-hands", &(NUMBER_OF_HANDS_MAXIMUM + 1).to_string(); "invalid hands maximum")]
    #[test_case("--number-of-hands", &(NUMBER_OF_HANDS_MINIMUM - 1).to_string(); "invalid hands minimum")]
    #[should_panic]
    fn test_arguments_panics(flag: &str, option: &str) {
        Arguments::make_args(&[flag, option]);
    }
}
