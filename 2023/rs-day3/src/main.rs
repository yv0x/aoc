use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct IdGenerator {
    current_id: usize,
}

impl IdGenerator {
    fn new() -> Self {
        IdGenerator { current_id: 0 }
    }

    fn next_id(&mut self) -> usize {
        let id = self.current_id;
        self.current_id += 1;
        id
    }
}

struct NumberWithCoordinates {
    id: usize,
    number: String,
    coordinates: Vec<(usize, usize)>,
    found: bool,
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut grid = Vec::new();
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    let mut numbers_with_coords = Vec::new();
    let mut total_sum = 0;
    let mut gear_sum = 0;
    let mut id_gen = IdGenerator::new();
    let mut last_row = None;
    let mut last_number = None;
    let mut id = 0;  // Initialize the ID

    for (row, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
        let mut number = String::new();


        let mut col = 0;
        //let mut coordinate = (0, 0); 

        // Iterate through each character in the line
        for ch in line.chars() {
            if ch.is_digit(10) {
                number.push(ch);
                coordinates.push((row, col));
                // Check if the row number is different from the last iteration
                if last_row != Some(row) {
                    id = id_gen.next_id();  // Fetch a new ID
                    last_row = Some(row);   // Update the last row number
                }   
                if last_number != Some(number.clone()) {
                    id = id_gen.next_id();  // Fetch a new ID
                    last_number = Some(number.clone());   // Update the last row number
                }
            } else {
                // If the accumulated string is not empty, print it
                if !number.is_empty() {
                    //println!("Number: {}", number); 
                    numbers_with_coords.push(NumberWithCoordinates {
                        id: id,
                        number: number.clone(),
                        coordinates: coordinates.clone(),
                        found: false,
                    });
                    number.clear();
                    coordinates.clear();
                }
            }
            // Accumulate digits to form a number
            col += 1;
        }


        // Check for a number at the end of the line
        if !number.is_empty() {
            //println!("{}", number);
            numbers_with_coords.push(NumberWithCoordinates {
                id: id,
                number: number.clone(),
                coordinates: coordinates.clone(),
                found: false,
            });
            number.clear();
            coordinates.clear();

        }
    }

    // Process the grid
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_symbol(grid[i][j]) {
                //println!("Symbol found at {}, {}", i, j);
                let result = check_and_print_neighbors(i, j, &mut numbers_with_coords); 
                match result {
                    Some(product) => {
                        println!("The product of the two numbers is: {}", product);
                        gear_sum += product;
                    }
                    None => println!("Did not find exactly two numbers"),
                }
            }
        }
    }
    //for nwc in &numbers_with_coords {
    //    println!("Number: {}, ID: {}, Coordinates: {:?}, Found: {:?}", nwc.number, nwc.id, nwc.coordinates, nwc.found);
    //    if nwc.found {
    //        total_sum += nwc.number.parse::<i32>().unwrap();
    //    }
    //}
    //println!("Total: {}", total_sum);
    println!("Total: {}", gear_sum);

    Ok(())
}

fn is_symbol(c: char) -> bool {
  matches!(
    c,
    '*' ) 
}

fn check_and_print_neighbors(i: usize, j: usize, numbers_with_coords: &mut [NumberWithCoordinates]) -> Option<i32> {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut found_numbers = Vec::new();
    // Iterate over each NumberWithCoordinates
    for nwc in numbers_with_coords {
        let nwc_number = &nwc.number;

        // Iterate over each coordinate in the NumberWithCoordinates
        for &(x, y) in &nwc.coordinates {
            for (dx, dy) in directions.iter() {
                let new_x = i as i32 + dx;
                let new_y = j as i32 + dy;

                //println!("Checking {}, {}", new_x, new_y);
                // Check if there is a digit at the adjacent position
                if x as i32 == new_x && y as i32 == new_y {
                    //println!("{}", nwc_number);
                    if !found_numbers.contains(nwc_number) {
                        found_numbers.push(nwc_number.to_string());
                    }
                    nwc.found = true;
                }
            }
        }
    }
    match found_numbers.len() {
        0 => println!("No numbers found."),
        1 => println!("Only one number found: {}", found_numbers[0]),
        _ => println!("Two or more numbers found: {:?}", found_numbers),
    }
    if found_numbers.len() == 2 {
        Some(found_numbers[0].parse::<i32>().unwrap() * found_numbers[1].parse::<i32>().unwrap())
    } else {
        None
    }
}

