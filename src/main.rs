mod calculator;

use std::env;
use std::fs::File;
use std::io;

// just count lines 39.47s user

// Java Baseline
// real	2m49.563s
// user	2m45.377s
// sys	0m3.299s


// Single Naive
// cargo run --release -- ../../community/1brc/measurements.txt  95.46s user 2.10s system 98% cpu 1:38.82 total
// Multi Threaded ( 2 threads) =)
// cargo run --release -- ../../community/1brc/measurements.txt  300.15s user 45.06s system 257% cpu 2:14.03 total



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a filename");
        std::process::exit(1);
    }

    let filename = &args[1]; // First argument is the filename

    let file = File::open(filename).expect("Cannot open file");

    let reader = io::BufReader::new(file);

    // Iterate over each line in the file.
    // for line_result in reader.lines() {
    //     let line = line_result?; // Handle any errors that might arise while reading the line.
    //     println!("{}", line); // or process the line
    // }

    calculator::calculate_values(reader);

    // let line_count = reader.lines().count();
    //
    // println!("{} has {} lines", filename, line_count);
}


// fn main() {
//     // Open the file
//     let file = File::open("large_file.txt").expect("Cannot open file");
//     let reader = BufReader::new(file);
//
//     // Determine the number of chunks
//     let chunk_size = determine_chunk_size(); // Implement this based on file size and system
//     let mut handles = vec![];
//
//     for chunk_start in (0..file_size).step_by(chunk_size) {
//         // Spawn a thread to handle each chunk
//         let handle = thread::spawn(move || {
//             // Read and process the chunk
//             // Note: Actual reading would involve seeking to the right position first
//             let chunk_data = read_chunk(chunk_start, chunk_size); // Implement this
//             process_chunk(chunk_data) // Implement this, e.g., count lines
//         });
//
//         handles.push(handle);
//     }
//
//     // Collect results
//     let mut total_lines = 0;
//     for handle in handles {
//         total_lines += handle.join().unwrap(); // Collect line count from each thread
//     }
//
//     println!("Total lines: {}", total_lines);
// }