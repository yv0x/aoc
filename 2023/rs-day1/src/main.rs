use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt"; // Replace with your file path
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;
    let mut coord = 0;

    for line in reader.lines() {
        let line = line?;
        let mut current_word = String::new();
        let mut first_digit_found = false;
        let mut first_digit = 0;
        let mut last_digit = 0;

        for ch in line.chars() {
            if ch.is_alphabetic() {
                current_word.push(ch);
                if first_digit_found {
                    if let Some(digit) = match_digit_word(&current_word, false) {
                        last_digit = digit;
                        //current_word.clear();
                    }
                } else {
                    if let Some(digit) = match_digit_word(&current_word, true) {
                        first_digit = digit;
                        first_digit_found = true;
                        current_word.clear();
                    }
                }
            } else if ch.is_digit(10) {
                let digit = ch.to_digit(10).unwrap();
                if !first_digit_found {
                    first_digit = digit;
                    first_digit_found = true;
                } else {
                    last_digit = digit;
                }
                current_word.clear();
            } else {
                current_word.clear();
            }
        }

        // Handle the case where the last digit is in the end part of the string
        if !first_digit_found {
            first_digit = last_digit;
        } else if !current_word.is_empty() {
            if let Some(digit) = match_digit_word(&current_word, false) {
                last_digit = digit;
            }
        }
        // If only one digit is found, it is both the first and last digit
        if last_digit == 0 {
            last_digit = first_digit;
        }

        coord = first_digit * 10 + last_digit;
        total_sum += coord; 
        println!("First number: {}, Last number: {}, Sum: {}, Totalsum: {}, Line: {}", first_digit, last_digit, coord, total_sum, line);
    }

    println!("Total sum of all first and last digits: {}", total_sum);
    Ok(())
}

fn match_digit_word(word: &str, is_first_digit: bool) -> Option<u32> {
    let digit_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for &(name, value) in &digit_words {
        if is_first_digit && word.contains(name) {
            return Some(value);
        } else if !is_first_digit && word.ends_with(name) {
            return Some(value);
        }
    }

    None
}

