use std::fs::File;
use std::io::{self, BufRead};

// Define the structs for each map with i64
struct Seeds(Vec<(i64, i64)>); // Each tuple is (start, length)
struct SeedToSoilMap(Vec<(i64, i64, i64)>);
struct SoilToFertilizerMap(Vec<(i64, i64, i64)>);
struct FertilizerToWaterMap(Vec<(i64, i64, i64)>);
struct WaterToLightMap(Vec<(i64, i64, i64)>);
struct LightToTemperatureMap(Vec<(i64, i64, i64)>);
struct TemperatureToHumidityMap(Vec<(i64, i64, i64)>);
struct HumidityToLocationMap(Vec<(i64, i64, i64)>);

impl Seeds {
    fn parse(data: &str) -> Seeds {
        let mut seed_ranges = Vec::new();
        let ranges: Vec<i64> = data.split_whitespace()
                                   .filter_map(|num| num.parse().ok())
                                   .collect();

        for range in ranges.chunks(2) {
            if range.len() == 2 {
                seed_ranges.push((range[0], range[1]));
            }
        }
        Seeds(seed_ranges)
    }
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let mut seeds = Seeds(Vec::new());
    let mut seed_to_soil = SeedToSoilMap(Vec::new());
    let mut soil_to_fertilizer = SoilToFertilizerMap(Vec::new());
    let mut fertilizer_to_water = FertilizerToWaterMap(Vec::new());
    let mut water_to_light = WaterToLightMap(Vec::new());
    let mut light_to_temperature = LightToTemperatureMap(Vec::new());
    let mut temperature_to_humidity = TemperatureToHumidityMap(Vec::new());
    let mut humidity_to_location = HumidityToLocationMap(Vec::new());

    let mut current_map = None;

    for line in buffered.lines() {
        let line = line?;
        if line.starts_with("seeds:") {
            let seed_values = line.split(':')
                .nth(1)
                .unwrap_or("")
                .trim();
            seeds = Seeds::parse(seed_values); // Correctly parse seed ranges here
        } else if line.starts_with("seed-to-soil map:") {
            current_map = Some("seed-to-soil");
        } else if line.starts_with("soil-to-fertilizer map:") {
            current_map = Some("soil-to-fertilizer");
        } else if line.starts_with("fertilizer-to-water map:") {
            current_map = Some("fertilizer-to-water");
        } else if line.starts_with("water-to-light map:") {
            current_map = Some("water-to-light");
        } else if line.starts_with("light-to-temperature map:") {
            current_map = Some("light-to-temperature");
        } else if line.starts_with("temperature-to-humidity map:") {
            current_map = Some("temperature-to-humidity");
        } else if line.starts_with("humidity-to-location map:") {
            current_map = Some("humidity-to-location");
        } else if let Some(map_type) = &current_map {
            let nums: Vec<i64> = line.split_whitespace()
                                     .filter_map(|s| s.parse().ok())
                                     .collect();
            if nums.len() == 3 {
                match *map_type {
                    "seed-to-soil" => seed_to_soil.0
                        .push((nums[0], nums[1], nums[2])),
                    "soil-to-fertilizer" => soil_to_fertilizer.0
                        .push((nums[0], nums[1], nums[2])),
                    "fertilizer-to-water" => fertilizer_to_water.0
                        .push((nums[0], nums[1], nums[2])),
                    "water-to-light" => water_to_light.0
                        .push((nums[0], nums[1], nums[2])),
                    "light-to-temperature" => light_to_temperature.0
                        .push((nums[0], nums[1], nums[2])),
                    "temperature-to-humidity" => temperature_to_humidity.0
                        .push((nums[0], nums[1], nums[2])),
                    "humidity-to-location" => humidity_to_location.0
                        .push((nums[0], nums[1], nums[2])),
                    _ => {}
                }
            }
        }
    }

    let mut lowest_location = i64::MAX;
    println!("Seeds");
    for item in seeds.0.iter() {
        println!("{:?}", item);
    }
    for (start, length) in seeds.0.iter() {
        for num in *start..*start + *length {
            // Process the seed number 'num'
            // For example, perform the mappings here
            let soil_number = convert(num, &seed_to_soil.0);
            let fertilizer_number = convert(soil_number, &soil_to_fertilizer.0);
            let water_number = convert(fertilizer_number, &fertilizer_to_water.0);
            let light_number = convert(water_number, &water_to_light.0);
            let temperature_number = convert(light_number, &light_to_temperature.0);
            let humidity_number = convert(temperature_number, &temperature_to_humidity.0);
            let location_number = convert(humidity_number, &humidity_to_location.0);
            if location_number < lowest_location { 
                lowest_location = location_number;
            }
            //println!("Mapped: {:?} {:?} {:?} {:?} {:?} {:?} {:?}", soil_number, fertilizer_number, water_number, light_number, temperature_number, humidity_number, location_number);
        }
        println!("{:?}", start);
    }
    println!("lowest location: {}", lowest_location);

    // Example: Printing the mappings
    //println!("Seed to Soil Map:");
    //for mapping in seed_to_soil.0.iter() {
    //    println!("{:?}", mapping);
    //}

    Ok(())
}

fn convert(source: i64, map: &[(i64, i64, i64)]) -> i64 {
    for &(dest_start, src_start, length) in map {
        if source >= src_start && source < src_start + length {
            // Calculate the offset from the start of the source range
            let offset = source - src_start;
            // Apply the offset to the destination start to get the mapped number
            return dest_start + offset;
        }
    }
    // If the source number is not explicitly mapped, it maps to itself
    source
}

