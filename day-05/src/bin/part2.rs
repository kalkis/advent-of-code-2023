//use std::collections::HashMap;

use day_05::{parse_almanac, Almanac};

fn main() {
    let almanac: Almanac = parse_almanac("input.txt");
    let mut location: u64 = 0;
    let mut seed = almanac.get_seed_from_location(location);

    // very slow
    while !almanac.contains_seed(seed) {
        location += 1;
        seed = almanac.get_seed_from_location(location);
    }
    println!("Lowest location for which almanac has a seed: {location} for seed {seed}");
}
