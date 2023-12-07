use day_05::{parse_almanac, Almanac};

fn main() {
    let almanac: Almanac = parse_almanac("input.txt");
    let mut locations: Vec<u64> = Vec::new();

    for seed in almanac.seeds.iter() {
        locations.push(almanac.get_location_from_seed(*seed));
    }
    let min_location = locations.iter().min().unwrap();
    println!("Lowest location for almanac's seeds: {min_location}");
}
