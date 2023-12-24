use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Eq)]
struct Card {
    rank: char,
    is_joker: bool,
}

impl Card {
    fn new(rank: char) -> Self {
        let is_joker = rank == 'J';
        Card { rank, is_joker }
    }

    fn value(&self) -> u8 {
        match self.rank {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,  // Jokers are the weakest in general ranking
            digit => digit.to_digit(10).unwrap() as u8,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    hand_type: u8,
    ranks: Vec<u8>,
    original_ranks: Vec<u8>,   // Stores the original order of card values
}

impl Hand {
    fn new(hand_str: &str, bid: i32) -> Self {
        let cards = hand_str.chars().map(Card::new).collect::<Vec<_>>();

        let original_ranks = hand_str.chars().map(|c| Card::new(c).value()).collect::<Vec<_>>();
        let mut hand = Hand {
            cards,
            bid,
            hand_type: 0,
            ranks: vec![],
            original_ranks,
        };
        hand.determine_hand_type();
        hand
    }

    fn determine_hand_type(&mut self) {
        if self.cards.iter().any(|card| card.is_joker) {
            // Implement logic for hands with joker
        } else {
            // Existing logic for hands without a joker
            let mut counts = std::collections::HashMap::new();
            for card in &self.cards {
                *counts.entry(card.value()).or_insert(0) += 1;
            }

            let mut ordered_ranks = Vec::new();
            for card in &self.cards {
                let count = counts.get(&card.value()).copied().unwrap_or_default();
                if !ordered_ranks.contains(&(card.value(), count)) {
                    ordered_ranks.push((card.value(), count));
                }
            }

            ordered_ranks.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
            self.ranks = ordered_ranks.iter().map(|(rank, _)| *rank).collect();

            self.hand_type = determine_hand_type_from_counts(&counts);
        }
    }

}

fn determine_hand_type_from_counts(counts: &std::collections::HashMap<u8, usize>) -> u8 {
    let mut counts_vec: Vec<_> = counts.values().cloned().collect();
    counts_vec.sort_unstable_by(|a, b| b.cmp(a)); // Sorting in descending order

    match counts_vec.as_slice() {
        &[5] => 8, // Five of a kind
        &[4, 1] => 7, // Four of a kind
        &[3, 2] => 6, // Full house
        &[3, 1, 1] => 4, // Three of a kind
        &[2, 2, 1] => 3, // Two pair
        &[2, 1, 1, 1] => 2, // One pair
        &[1, 1, 1, 1, 1] => 1, // High card
        _ => 0, // Default case, should not happen
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| {
                if self.hand_type == other.hand_type {
                    // Compare based on original_ranks if hand types are the same
                    self.original_ranks.iter().zip(other.original_ranks.iter()).find_map(|(a, b)| {
                        if a != b { Some(a.cmp(b)) } else { None }
                    }).unwrap_or(Ordering::Equal)
                } else {
                    // Existing comparison logic for ranks if hand types are different
                    self.ranks.iter().zip(other.ranks.iter()).find_map(|(a, b)| {
                        if a != b { Some(a.cmp(b)) } else { None }
                    }).unwrap_or(Ordering::Equal)
                }
            })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.ranks == other.ranks
    }
}

fn calculate_winnings(hands: &mut [Hand]) -> i32 {
    hands.sort();
    // Print out the hands for debugging
    for hand in hands.iter() {
        println!("{}: {:?} - Bid: {}", hand.hand_type, hand.cards, hand.bid);
    }

    hands.iter().enumerate().map(|(i, hand)| hand.bid * (i as i32 + 1)).sum()
}

fn read_hands_from_file(filename: &str) -> io::Result<Vec<Hand>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut hands = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let hand_str = parts[0];
            let bid = parts[1].parse::<i32>().unwrap_or(0);
            hands.push(Hand::new(hand_str, bid));
        }
    }

    Ok(hands)
}

fn main() -> io::Result<()> {
    let filename = "example.txt"; // replace with your file name
    let mut hands = read_hands_from_file(filename)?;

    let total_winnings = calculate_winnings(&mut hands);
    println!("Total winnings: {}", total_winnings);

    Ok(())
}
