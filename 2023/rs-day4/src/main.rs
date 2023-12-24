use std::fs;
use indexmap::IndexMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error reading file");
    let mut parts_vec = Vec::new(); 
    let mut total_vec = IndexMap::new(); 
    let mut match_vec = Vec::new();
    let mut total_value = 0;
    let mut largest_value = 0;

    for (_, line) in contents.lines().enumerate() {
        if let Some(data) = line.splitn(2, ':').nth(1) {
            //println!("{}", data);
            let num_split = data.split('|').collect::<Vec<&str>>();
            //println!("{:?}", num_split);
            for entry in num_split {
                let parts = entry.trim().split_whitespace().collect::<Vec<&str>>();
                //println!("Part :{:?}", parts);
                parts_vec.push(parts);
            }

            //println!("parts_vec: {:?}", parts_vec);
            let left = &parts_vec[0];
            let right = &parts_vec[1];
            // Count matches
            let mut matches = 0;
            let mut points = 0;
            for &num in left {
                if right.contains(&num) {
                    matches += 1;
                    if matches == 1 {
                        points = matches;
                    } else {
                        points = points * 2;
                    }
                }
            }
            //println!("In {:?} matches: {}, points: {}", parts_vec, matches, points);
            //println!("Matches in vector: {}", matches);
            total_vec.entry(parts_vec.clone()).or_insert((matches, 0));
            //total_vec.insert(index, (parts_vec.clone(), match_vec));
            match_vec.push((matches, 0, 0));
            parts_vec.clear();
            total_value += points;

            if matches > largest_value {
                largest_value = matches;
            }
            //println!("total points: {}", total_value);
            //println!("most matches: {}", largest_value);

        }

    }
    for (key, value) in &total_vec {
        println!("{:?}, {:?}", key, value);
    } 
    let mut index = 1;
    for line in &match_vec {
        println!("Card {}: {:?}",index, line);
        index += 1;
    }
    let cards = match_vec.clone();
    let mut card_counts = vec![1; cards.len()];
    println!("  ");
    for i in 0..cards.len() {
        let (matches, _, _) = cards[i];
        let win_count = card_counts[i]; // Number of this card determines how many of next cards are won

        // Win cards based on the number of matches
        for j in 1..=matches as usize {
            if i + j < cards.len() {
                card_counts[i + j] += win_count;
            }
        }
    }

    // Print the final counts
    let mut total = 0;
    for (index, count) in card_counts.iter().enumerate() {
        println!("Card {}: {}", index + 1, count);
        total += count;
    }

    println!("Total number of cards: {}", total);
}

