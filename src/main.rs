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
mod xlog;

use arguments::arguments::Arguments;
use arguments::parameters::Parameters;
use constants::constants::STRIKER_WHO_AM_I;
use report::report::Report;
use simulator::simulator::Simulator;
use std::thread;
use std::thread::ThreadId;
use strategy::rules::Rules;
use strategy::strategy::Strategy;
use utilities::utilities::Utility;
use xlog::xlog::*;

//
fn main() {
    init_xlog(SYSLOG_ADDRESS);
    let mut handles = Vec::new();
    let arguments = Arguments::new();
    let parameters = Parameters::new(&arguments);
    let mut rules = Rules::new();
    let mut strategy = Strategy::new();
    let mut final_report = Report::new();
    let utility = Utility::default();

    rules.init(&utility, &arguments.decks);
    strategy.init(&utility, &arguments);
    let start = xlog_start!(
        "Simulation started [{}], strategy={}, decks={}, hands={}",
        parameters.name,
        arguments.strategy,
        arguments.decks,
        arguments.number_of_hands
    );
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
        let handle = thread::spawn(move || {
            let thread_id: ThreadId = thread::current().id();
            let start: usize = xlog_start!("Thread started [{:?}] for [{}]", thread_id, parameters.name);
            let simulator = Simulator::new(&parameters, &rules, &strategy);
            let result = simulator.run_once();
            (result, thread_id, start)
        });
        handles.push(handle);
    }

    // Merge simulation results
    for handle in handles {
        let (simulator, thread_id, start) = handle.join().unwrap();
        final_report.merge(&simulator.get_report());
        xlog_stop!(start as usize, "Thread completed [{:?}] for [{}]", thread_id, parameters.name);
    }
    final_report.finish();

    println!("  -- {:<10} {}", "results", "-".repeat(66));
    final_report.print();
    println!("  {}", "-".repeat(80));
    println!("  -- {:<10} {}", "insert", "-".repeat(66));
    final_report.insert(&utility);
    println!("  {}", "-".repeat(80));
    xlog_stop!(
        start,
        "Simulation completed: [{}], hands={}, rounds={}",
        parameters.name,
        final_report.total_hands,
        final_report.total_rounds
    );
}
