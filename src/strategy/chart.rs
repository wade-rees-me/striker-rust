pub const TABLE_SIZE: usize = 21;
pub const NUM_COLUMNS: usize = 12;

#[derive(Default, Debug, Clone)]
pub struct ChartRow {
    key: String,
    value: [String; NUM_COLUMNS],
}

impl ChartRow {
    fn new() -> Self {
        Self {
            key: "--".to_string(),
            value: std::array::from_fn(|_| "---".to_string()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Chart {
    rows: [ChartRow; TABLE_SIZE],
    name: String,
    next_row: usize,
}

impl Chart {
    pub fn new(name: &str) -> Self {
        Self {
            rows: std::array::from_fn(|_| ChartRow::new()),
            name: name.to_string(),
            next_row: 0,
        }
    }

    pub fn insert(&mut self, key: &str, up: usize, value: &str) {
        let index = match self.get_row_index(key) {
            Some(i) => i,
            None => {
                let i = self.next_row;
                self.next_row += 1;
                self.rows[i].key = key.to_uppercase();
                i
            }
        };
        self.rows[index].value[up] = value.to_uppercase();
    }

    pub fn get_value_by_key(&self, key: &str, up: usize) -> &str {
        self.get_row_index(key)
            .map(|i| &self.rows[i].value[up])
            .unwrap_or_else(|| panic!("Cannot find value in {} for {} vs {}", self.name, key, up))
    }

    pub fn print(&self) {
        println!("{}", self.name);
        println!("--------------------2-----3-----4-----5-----6-----7-----8-----9-----X-----A---");
        for i in 0..self.next_row {
            let row = &self.rows[i];
            print!("{:>2} : ", row.key);
            for val in &row.value {
                print!("{:>4}, ", val);
            }
            println!();
        }
        println!("------------------------------------------------------------------------------\n");
    }

    fn get_row_index(&self, key: &str) -> Option<usize> {
        let key_upper = key.to_uppercase();
        self.rows.iter().take(self.next_row).position(|row| row.key == key_upper)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::chart::{Chart, NUM_COLUMNS, TABLE_SIZE};

    #[test]
    fn test_chart_row_new() {
        let row = Chart::new("Test").rows[0].clone();
        assert_eq!(row.key, "--");
        assert_eq!(row.value.len(), NUM_COLUMNS);
        for v in &row.value {
            assert_eq!(v, "---");
        }
    }

    #[test]
    fn test_chart_new() {
        let chart = Chart::new("Basic Strategy");
        assert_eq!(chart.name, "Basic Strategy");
        assert_eq!(chart.next_row, 0);
        assert_eq!(chart.rows.len(), TABLE_SIZE);
    }

    #[test]
    fn test_chart_insert_and_get_value_by_key() {
        let mut chart = Chart::new("Test Chart");
        chart.insert("8", 2, "H");
        chart.insert("8", 3, "S");
        chart.insert("A,7", 4, "D");

        assert_eq!(chart.get_value_by_key("8", 2), "H");
        assert_eq!(chart.get_value_by_key("8", 3), "S");
        assert_eq!(chart.get_value_by_key("A,7", 4), "D");
    }

    #[test]
    fn test_chart_case_insensitive_keys() {
        let mut chart = Chart::new("Test Chart");
        chart.insert("8", 2, "H");
        chart.insert("a,7", 3, "D");

        assert_eq!(chart.get_value_by_key("8", 2), "H");
        assert_eq!(chart.get_value_by_key("A,7", 3), "D");
        assert_eq!(chart.get_value_by_key("a,7", 3), "D");
    }

    #[test]
    #[should_panic(expected = "Cannot find value in Test Chart for 5 vs 4")]
    fn test_chart_get_value_invalid_key_panics() {
        let chart = Chart::new("Test Chart");
        chart.get_value_by_key("5", 4);
    }

    #[test]
    fn test_chart_insert_multiple_rows() {
        let mut chart = Chart::new("Multi Row Chart");
        chart.insert("2", 2, "H");
        chart.insert("3", 2, "S");
        chart.insert("4", 2, "D");

        assert_eq!(chart.get_value_by_key("2", 2), "H");
        assert_eq!(chart.get_value_by_key("3", 2), "S");
        assert_eq!(chart.get_value_by_key("4", 2), "D");
        chart.print();
    }
}
