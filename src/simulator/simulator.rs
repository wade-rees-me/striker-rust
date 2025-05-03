use crate::arguments::parameters::Parameters;
use crate::report::report::Report;
use crate::simulator::table::Table;
use crate::strategy::rules::Rules;
use crate::strategy::strategy::Strategy;

pub struct Simulator {
    parameters: Parameters,
    table: Table,
    report: Report,
}

impl Simulator {
    pub fn new(parameters: &Parameters, rules: &Rules, strategy: &Strategy) -> Self {
        let table = Table::new(parameters.clone(), rules.clone(), strategy.clone());
        Self {
            parameters: parameters.clone(),
            table,
            report: Report::new(),
        }
    }

    pub fn run_once(mut self) -> Self {
        let mimic = self.parameters.strategy == "mimic";
        self.table.session(mimic);

        let table_report = self.table.get_report();
        self.report.merge(&table_report);

        self
    }

    pub fn get_report(&self) -> &Report {
        &self.report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::arguments::Arguments;

    #[test]
    fn test_simulator_initialization() {
        let arguments = Arguments::new();
        let parameters = Parameters::new(&arguments);
        let rules = Rules::default();
        let strategy = Strategy::new();
        let _table = Table::new(parameters.clone(), rules.clone(), strategy.clone());
        let simulator = Simulator::new(&parameters.clone(), &rules.clone(), &strategy.clone());
        simulator.get_report();
    }
}
