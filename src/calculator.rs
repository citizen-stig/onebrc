// Amsterdam;2.1
// Amsterdam;4.6
// Amsterdam;12.8
// Amsterdam;12.7

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Measurement {
    min: f64,
    max: f64,
    sum: f64,
    count: f64,
}

impl Measurement {
    fn new(value: f64) -> Self {
        Self {
            min: value,
            max: value,
            sum: value,
            count: 1.0,
        }
    }

    fn record_value(&mut self, value: f64) {
        self.count += 1.0;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
        self.sum += value;
    }

    fn merge(&mut self, other: Measurement) {
        self.count += other.count;
        self.sum += other.sum;
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max
        }
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // <min>/<mean>/<max>
        let mean = self.sum / self.count;
        write!(f, "{}/{:.1}/{}", self.min, mean, self.max)
    }
}


pub fn calculate_values<R: BufRead>(reader: R) {
    let start = Instant::now();
    let num_workers = 4;

    let mut senders = Vec::with_capacity(num_workers);
    let mut handles = Vec::with_capacity(num_workers);

    for _ in 0..num_workers {
        // let (tx, rx) = mpsc::sync_channel::<String>(1000000);
        let (tx, rx) = mpsc::channel::<String>();

        let handle = thread::spawn(move || {
            let worker_start = Instant::now();
            let mut data: HashMap<String, Measurement> = HashMap::new();
            let mut total_line_processing = Duration::new(0, 0);
            while let Ok(line) = rx.recv() {
                let n = Instant::now();
                let mut parts = line.split(';');

                if let (Some(city), Some(value)) = (parts.next(), parts.next()) {
                    let value = value.parse::<f64>().unwrap();
                    data.entry(city.to_string()).and_modify(|e| e.record_value(value)).or_insert(Measurement::new(value));
                } else {
                    println!("Data is not in the expected format.");
                }
                total_line_processing += n.elapsed();
            }
            println!("WORKER HAS COMPLETED IN {:?}, WHICH SPENT PROCESSING LINE: {:?}", worker_start.elapsed(), total_line_processing);
            data
        });
        senders.push(tx);
        handles.push(handle);
    }
    println!("Threads are started in {:?}", start.elapsed());

    let mut channel_total_duration = Duration::new(0, 0);

    for (idx, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Failed to read line");
        let worker_index = idx % num_workers;
        let n= Instant::now();
        senders[worker_index].send(line).unwrap();
        channel_total_duration += n.elapsed();
    }
    println!("ALL LINES HAS BEEN READ in {:?}, channel work {:?}", start.elapsed(), channel_total_duration);

    drop(senders);
    let merging = Instant::now();
    let mut final_results: HashMap<String, Measurement> = HashMap::with_capacity(10000);
    for handle in handles {
        let partial_results = handle.join().unwrap();
        for (key, value) in partial_results {
            match final_results.entry(key) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(value);
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    e.get_mut().merge(value);
                }
            }
        }
    }
    println!("Merging is done in {:?}, total {:?}", merging.elapsed(), start.elapsed());


    println!("FINAL RESULTS: {}", final_results.len());
    // for (city, measurement) in final_results.iter() {
    //     println!("{}={}", city, measurement);
    // }
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

