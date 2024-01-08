// Amsterdam;2.1
// Amsterdam;4.6
// Amsterdam;12.8
// Amsterdam;12.7

use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Measurement {
    min: f64,
    max: f64,
    average: f64,
    count: f64,
}

impl Measurement {
    fn new(value: f64) -> Self {
        Self {
            min: value,
            max: value,
            average: value,
            count: 1.0,
        }
    }

    fn record_value(&mut self, value: f64) {
        self.count += 1.0;

        // Then calculate the new average
        let new_average = self.average + (value - self.average) / self.count;

        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
        self.average = new_average;
    }
}

pub fn calculate_values<R: BufRead>(reader: R) {
    // let line_count = reader.lines().count();

    let mut data: HashMap<String, Measurement> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        let mut parts = line.split(';');

        if let (Some(city), Some(value)) = (parts.next(), parts.next()) {
            let value = value.parse::<f64>().unwrap();
            data.entry(city.to_string()).and_modify(|e| e.record_value(value)).or_insert(Measurement::new(value));
        } else {
            println!("Data is not in the expected format.");
        }
    }

    println!("OUTPUT: {:?}", data);

    // println!("File has {} lines", line_count);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_measurement() {
        let values = vec![1.0, 3.0, 4.0, 5.0, 8.0];

        let mut measurement = Measurement::new(values[0]);

        for value in values.iter().skip(1) {
            measurement.record_value(*value);
        }

        let expected_min = values.iter().cloned().reduce(f64::min).unwrap();
        let expected_max = values.iter().cloned().reduce(f64::max).unwrap();
        let expected_average = values.iter().sum::<f64>() / values.len() as f64;

        assert_eq!(expected_min, measurement.min);
        assert_eq!(expected_max, measurement.max);
        assert_eq!(expected_average, measurement.average);
    }
}

