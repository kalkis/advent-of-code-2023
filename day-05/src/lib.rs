use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct MapRange {
    source: u64,
    dest: u64,
    range: u64,
}

impl MapRange {
    pub fn within(&self, n: u64) -> bool {
        (self.source <= n) && ((self.source + self.range) > n)
    }

    pub fn lookup(&self, n: u64) -> Option<u64> {
        if !self.within(n) {
            None
        } else {
            Some((n - self.source) + self.dest)
        }
    }

    pub fn within_dest(&self, n: u64) -> bool {
        (self.dest <= n) && ((self.dest + self.range) > n)
    }

    pub fn reverse_lookup(&self, n: u64) -> Option<u64> {
        if !self.within_dest(n) {
            None
        } else {
            Some((n - self.dest) + self.source)
        }
    }

    pub fn new(source: u64, dest: u64, range: u64) -> Self {
        Self {
            source: source,
            dest: dest,
            range: range,
        }
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &MapRange) -> Ordering {
        self.source.cmp(&other.source)
    }
}

#[derive(Debug, PartialEq)]
pub struct AlmanacMap {
    ranges: Vec<MapRange>,
}

impl AlmanacMap {
    pub fn lookup(&self, input: u64) -> u64 {
        for range in &self.ranges {
            match range.lookup(input) {
                Some(r) => return r,
                None => (),
            }
        }
        // if none was found, the destination is the same as the source
        input
    }

    pub fn reverse_lookup(&self, input: u64) -> u64 {
        for range in &self.ranges {
            match range.reverse_lookup(input) {
                Some(r) => return r,
                None => (),
            }
        }
        input
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    pub fn within(&self, n: u64) -> bool {
        (n >= self.start) && (n < self.start + self.length)
    }

    pub fn new(start: u64, length: u64) -> Self {
        Self {
            start: start,
            length: length,
        }
    }
}

impl Ord for SeedRange {
    fn cmp(&self, other: &SeedRange) -> Ordering {
        self.start.cmp(&other.start)
    }
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_ranges: Vec<SeedRange>,
    pub maps: Vec<AlmanacMap>,
}

impl Almanac {
    pub fn insert_map(&mut self, map: AlmanacMap) {
        self.maps.push(map)
    }

    pub fn new() -> Self {
        Almanac {
            seeds: Vec::new(),
            // for part 2
            seed_ranges: Vec::new(),
            maps: Vec::new(),
        }
    }

    pub fn get_location_from_seed(&self, seed: u64) -> u64 {
        let mut result: u64 = seed;
        for m in self.maps.iter() {
            result = m.lookup(result);
        }
        result
    }

    pub fn contains_seed(&self, seed: u64) -> bool {
        self.seed_ranges.iter().any(|s| s.within(seed))
    }

    pub fn get_seed_from_location(&self, location: u64) -> u64 {
        let mut result: u64 = location;
        // iterate from last map back to first
        for m in self.maps.iter().rev() {
            result = m.reverse_lookup(result);
        }
        result
    }
}

// this function sucks
pub fn parse_almanac(filename: &str) -> Almanac {
    let mut almanac = Almanac::new();
    let mut in_map = false;
    let mut current_map: Vec<MapRange> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if l.starts_with("seeds: ") {
                    let seeds = parse_seeds(&l);
                    almanac.seed_ranges = build_seed_ranges(&seeds);
                    // for part 1
                    almanac.seeds = seeds;
                } else if l.contains("map") {
                    // will start to process map on next iteration
                    in_map = true;
                // a blank line indicates end of the map
                } else if l.is_empty() {
                    in_map = false;
                    if current_map.len() > 0 {
                        current_map.sort();
                        almanac.insert_map(AlmanacMap {
                            ranges: current_map.clone(),
                        });
                        current_map.clear();
                    }
                } else if in_map {
                    current_map.push(parse_map_range(&l));
                }
            }
        }
        // handles last map, since there is no extra line after it
        // very jank
        if current_map.len() > 0 {
            current_map.sort();
            almanac.insert_map(AlmanacMap {
                ranges: current_map.clone(),
            });
        }
    }
    almanac
}

pub fn parse_map_range(line: &str) -> MapRange {
    let map_line: Vec<u64> = line
        .split(' ')
        .map(|c| u64::from_str_radix(c, 10).expect("Should be a number"))
        .collect();
    MapRange::new(map_line[1], map_line[0], map_line[2])
}

pub fn parse_seeds(line: &str) -> Vec<u64> {
    let line = &line[7..]; // skips the "seeds: " part of the line
    line.split(' ')
        .map(|s| u64::from_str_radix(s, 10).expect("Should be a number"))
        .collect()
}

pub fn build_seed_ranges(seeds: &Vec<u64>) -> Vec<SeedRange> {
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    seeds.chunks_exact(2).for_each(|chunk| {
        seed_ranges.push(SeedRange {
            start: chunk[0],
            length: chunk[1],
        })
    });
    seed_ranges.sort();
    seed_ranges
}

// shamelessly stolen from Rust By Example
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let expected: Vec<u64> = vec![79, 14, 55, 13];
        assert_eq!(expected, parse_seeds(input));
    }

    #[test]
    fn test_parse_map_range() {
        let input = "50 98 2";
        let expected = MapRange::new(98, 50, 2);
        assert_eq!(expected, parse_map_range(input));
    }

    #[test]
    fn test_map_range() {
        let map_range = MapRange {
            source: 50,
            dest: 100,
            range: 25,
        };

        assert!(map_range.within(60));
        assert!(!map_range.within(80));
        assert_eq!(Some(120), map_range.lookup(70));
        assert_eq!(None, map_range.lookup(100));
    }

    #[test]
    fn test_build_seed_ranges() {
        let seeds: Vec<u64> = vec![79, 14, 55, 13];
        let expected: Vec<SeedRange> = vec![SeedRange::new(55, 13), SeedRange::new(79, 14)];
        assert_eq!(expected, build_seed_ranges(&seeds));
    }

    #[test]
    fn test_almanac() {
        let input = "test.txt";
        let almanac = Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_ranges: vec![SeedRange::new(55, 13), SeedRange::new(79, 14)],
            maps: vec![
                AlmanacMap {
                    ranges: vec![MapRange::new(50, 52, 48), MapRange::new(98, 50, 2)],
                },
                AlmanacMap {
                    ranges: vec![
                        MapRange::new(0, 39, 15),
                        MapRange::new(15, 0, 37),
                        MapRange::new(52, 37, 2),
                    ],
                },
                AlmanacMap {
                    ranges: vec![
                        MapRange::new(0, 42, 7),
                        MapRange::new(7, 57, 4),
                        MapRange::new(11, 0, 42),
                        MapRange::new(53, 49, 8),
                    ],
                },
                AlmanacMap {
                    ranges: vec![MapRange::new(18, 88, 7), MapRange::new(25, 18, 70)],
                },
                AlmanacMap {
                    ranges: vec![
                        MapRange::new(45, 81, 19),
                        MapRange::new(64, 68, 13),
                        MapRange::new(77, 45, 23),
                    ],
                },
                AlmanacMap {
                    ranges: vec![MapRange::new(0, 1, 69), MapRange::new(69, 0, 1)],
                },
                AlmanacMap {
                    ranges: vec![MapRange::new(56, 60, 37), MapRange::new(93, 56, 4)],
                },
            ],
        };
        assert_eq!(almanac, parse_almanac(input));

        assert_eq!(82, almanac.get_location_from_seed(79));
        assert_eq!(79, almanac.get_seed_from_location(82));
        assert!(almanac.contains_seed(80));
        assert!(almanac.contains_seed(62));
        assert!(!almanac.contains_seed(0));
        assert!(!almanac.contains_seed(14));
    }

    #[test]
    fn test_seed_range() {
        let seed_range = SeedRange::new(55, 13);
        assert!(seed_range.within(56));
        assert!(!seed_range.within(54));
    }
}
