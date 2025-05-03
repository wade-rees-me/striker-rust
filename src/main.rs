mod arguments;
mod cards;
mod constants;
mod player;
mod report;
mod resources;
mod simulator;
mod strategy;
mod traits;
mod utilities;

use arguments::arguments::Arguments;
use arguments::parameters::Parameters;
use constants::constants::STRIKER_WHO_AM_I;
use report::report::Report;
use simulator::simulator::Simulator;
use std::thread;
use strategy::rules::Rules;
use strategy::strategy::Strategy;
use utilities::utilities::Utility;

//
fn main() {
    let mut handles = Vec::new();
    let arguments = Arguments::new();
    let parameters = Parameters::new(&arguments);
    let mut rules = Rules::new();
    let mut strategy = Strategy::new();
    let mut final_report = Report::new();
    let utility = Utility::default();

    rules.init(&utility, &arguments.decks);
    strategy.init(&utility, &arguments);
    println!("Start: {}", STRIKER_WHO_AM_I);
    println!("  -- {:<10} {}", "arguments", "-".repeat(66));
    println!("{}", parameters);
    println!("{}", rules);
    println!("  {}", "-".repeat(80));

    // Spawn worker threads
    final_report.init(&parameters);
    for _ in 0..parameters.number_of_threads {
        let parameters = parameters.clone();
        let rules = rules.clone();
        let strategy = strategy.clone();

        handles.push(thread::spawn(move || {
            let simulator = Simulator::new(&parameters, &rules, &strategy);
            simulator.run_once()
        }));
    }

    // Merge simulation results
    for handle in handles {
        if let Ok(simulator) = handle.join() {
            final_report.merge(&simulator.get_report());
        }
    }
    final_report.finish();

    println!("  -- {:<10} {}", "results", "-".repeat(66));
    final_report.print();
    println!("  {}", "-".repeat(80));
    println!("  -- {:<10} {}", "insert", "-".repeat(66));
    final_report.insert(&utility);
    println!("  {}", "-".repeat(80));
}
