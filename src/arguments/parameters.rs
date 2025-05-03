use crate::Arguments;
use crate::constants::constants::*;
use chrono::{Datelike, Local};
use num_format::{Locale, ToFormattedString};
use std::fmt;

#[derive(Clone)]
pub struct Parameters {
    pub name: String,
    pub processor: String,
    pub playbook: String,
    pub strategy: String,
    pub decks: String,
    pub epoch: String,
    pub number_of_decks: usize,
    pub number_of_hands: usize,
    pub share_of_hands: usize,
    pub number_of_threads: usize,
    pub verbose: bool,
}

impl Parameters {
    pub fn new(arguments: &Arguments) -> Self {
        let now = Local::now();
        let strategy = arguments.strategy.clone();
        let decks = arguments.decks.clone();
        let threads = arguments.number_of_threads.max(1);
        Self {
            name: format!("{STRIKER_WHO_AM_I}_{:04}_{:02}_{:02}_{:012}", now.year(), now.month(), now.day(), now.timestamp()),
            processor: STRIKER_WHO_AM_I.to_string(),
            playbook: format!("{decks}-{strategy}"),
            strategy: strategy,
            decks: decks,
            epoch: now.format(TIME_LAYOUT).to_string(),
            number_of_decks: arguments.number_of_decks,
            number_of_hands: arguments.number_of_hands,
            share_of_hands: (arguments.number_of_hands / threads) + 1,
            number_of_threads: arguments.number_of_threads,
            verbose: arguments.number_of_threads == 1,
        }
    }
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:>4}{:<26}: {}", "", "Processor", self.processor)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Threads", self.number_of_threads)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Name", self.name)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Version", STRIKER_VERSION)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Playbook", self.playbook)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Decks", self.decks)?;
        writeln!(f, "{:>4}{:<26}: {}", "", "Strategy", self.strategy)?;
        writeln!(f, "{:>4}{:<26}: {:>17}", "", "Number of hands", self.number_of_hands.to_formatted_string(&Locale::en))?;
        writeln!(f, "{:>4}{:<26}: {:>17}", "", "Thread share of hands", self.share_of_hands.to_formatted_string(&Locale::en))?;
        write!(f, "{:>4}{:<26}: {}", "", "Epoch", self.epoch)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let args = Arguments::default();
        let parameters = Parameters::new(&args);
        print!("{}", parameters);
    }
}
