use std::fs;
use std::collections::HashMap;

// Define a struct for color counts
struct ColorCounts {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    // Initialize a variable to store the sum of products
    let mut sum_of_products = 0;

    // Parse the input
    for (game_index, line) in contents.lines().enumerate() {
        // Initialize the maximum counts with zero
        let mut max_counts = ColorCounts { red: 0, green: 0, blue: 0 };

        // Splitting the line to ignore the "Game X:" prefix
        if let Some(data) = line.splitn(2, ':').nth(1) {
            let segments = data.split(';').collect::<Vec<&str>>();

            for segment in segments.iter() {
                let mut counts = HashMap::new();
                let entries = segment.split(',').collect::<Vec<&str>>();
                for entry in entries {
                    let parts = entry.trim().split_whitespace().collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        if let Ok(count) = parts[0].parse::<i32>() {
                            let color = parts[1];
                            *counts.entry(color).or_insert(0) += count;
                        }
                    }
                }

                let turn_counts = ColorCounts {
                    red: *counts.get("red").unwrap_or(&0),
                    green: *counts.get("green").unwrap_or(&0),
                    blue: *counts.get("blue").unwrap_or(&0),
                };

                // Update the maximum counts
                max_counts.red = max_counts.red.max(turn_counts.red);
                max_counts.green = max_counts.green.max(turn_counts.green);
                max_counts.blue = max_counts.blue.max(turn_counts.blue);
            }
        }

        // Calculate the product for the current game
        let product = max_counts.red * max_counts.green * max_counts.blue;
        sum_of_products += product;

        // Print the maximum counts for each color for the game
        println!("Game {}: Maximum Red: {}, Green: {}, Blue: {}",
                 game_index + 1, max_counts.red, max_counts.green, max_counts.blue);
    }

    // Print the total sum of products across all games
    println!("Total sum of products across all games: {}", sum_of_products);
}

