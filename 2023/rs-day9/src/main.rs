use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut data_nums = Vec::new();

    if args.len() > 1 {
        let filename = &args[1];
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let mut data_lines = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let numbers: Vec<i32> = line.split_whitespace()
                                        .filter_map(|s| s.parse().ok())
                                        .collect();
            data_lines.push(numbers);
        }

        // Process each line
        for line in data_lines {
            data_nums.push(line.clone());
            let mut current_line = line;
            while *current_line.last().unwrap_or(&0) != 0 {
                //println!("{:?}", current_line);
                current_line = compute_differences(&current_line);
                data_nums.push(current_line.clone());
            }
            //println!("{:?}", current_line);  // Print the final line of zeros
            //data_nums.push(current_line.clone());
            //println!("");
        }
    } else {
        println!("No filename provided.");
    }

    // Loop back and modify each line
    let mut last_element = 0;
    let mut accumulate = 0;
    let mut prev_first = 0;
    let mut diff = 0;
    for line in data_nums.iter_mut().rev() {
        if let Some(&first) = line.first() {
            if let Some(&last) = line.last() {
                if last != 0 {
                    // Calculate the difference and prepend it
                    diff = first - prev_first ;
                    line.insert(0, diff);
                    // Update prev_first for the next iteration (previous line)
                    prev_first = diff;
                } else {
                    accumulate += diff;
                    diff = 0;
                    prev_first = 0;
                }
            }
        }
    }
    // Handle the last(first) row separately
    accumulate += diff;

    // Print the modified data
    for line in &data_nums {
        println!("{:?}", line);
    }

    println!("Accumulate: {}", accumulate);
    Ok(())
}

fn compute_differences(numbers: &[i32]) -> Vec<i32> {
    numbers.windows(2).map(|window| window[1] - window[0]).collect()
}

