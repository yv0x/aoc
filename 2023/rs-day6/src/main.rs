use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);
    let mut time: i64 = 0;
    let mut distance: i64  = 0;

    for line in buffered.lines() {
        let line = line?;
        if line.starts_with("Time: ") {
            let words = line.split_whitespace().skip(1);
            let concatenated = words.fold(String::new(), |mut acc, word| {
                acc.push_str(word);
                acc
            });
            time = concatenated.parse().unwrap_or(0);
        } else if line.starts_with("Distance: ") {
            let words = line.split_whitespace().skip(1);
            let concatenated = words.fold(String::new(), |mut acc, word| {
                acc.push_str(word);
                acc
            });
            distance = concatenated.parse().unwrap_or(0);
        }

    }

    println!("{}", time);
    println!("{}", distance);

    let mut count = 0;

    for j in 0..time {
        let travel_distance = j * (time - j);
        //println!("{}", travel_distance);
        if travel_distance > distance {
            count += 1;
        } 

    }
    println!("Count: {}", count);

    
    Ok(())
}
