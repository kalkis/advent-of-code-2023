use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
pub enum Colour {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Cube {
    pub colour: Colour,
    pub quantity: u32,
}

impl Cube {
    pub fn from(colour: Colour, quantity: u32) -> Self {
        Cube {
            colour: colour,
            quantity: quantity,
        }
    }

    pub fn is_valid(&self, valid_cubes: &HashMap<Colour, u32>) -> bool {
        match valid_cubes.get(&self.colour) {
            Some(n) => self.quantity <= *n,
            None => false,
        }
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quantity.cmp(&other.quantity)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Round {
    pub cubes: Vec<Cube>,
}

impl Round {
    pub fn from(cubes: Vec<Cube>) -> Self {
        Self { cubes: cubes }
    }

    pub fn new() -> Self {
        Self { cubes: vec![] }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }

    pub fn is_valid(&self, valid_cubes: &HashMap<Colour, u32>) -> bool {
        self.cubes.iter().all(|c| c.is_valid(&valid_cubes))
    }

    pub fn calculate_power(&self) -> u32 {
        let mut power = 1;
        for c in &self.cubes {
            power *= c.quantity;
        }
        power
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn from(id: u32, rounds: Vec<Round>) -> Self {
        Self {
            id: id,
            rounds: rounds,
        }
    }

    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            rounds: vec![],
        }
    }

    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round)
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_min_cubes(&self) -> Round {
        let mut max_seen: HashMap<Colour, Cube> = HashMap::new();
        for round in &self.rounds {
            for cube in &round.cubes {
                if cube > max_seen.entry(cube.colour).or_insert(*cube) {
                    max_seen.insert(cube.colour, *cube);
                }
            }
        }

        let min_cubes: Vec<Cube> = max_seen.into_values().collect();
        Round::from(min_cubes)
    }
}

pub fn sum_valid_games(filename: &str, valid: HashMap<Colour, u32>) -> u32 {
    let mut sum = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(game) = parse_game(&l) {
                    if game.rounds.iter().all(|r| r.is_valid(&valid)) {
                        sum += game.id;
                    }
                }
            }
        }
    }
    sum
}

pub fn sum_game_powers(filename: &str) -> u32 {
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(game) = parse_game(&l) {
                    sum += game.get_min_cubes().calculate_power();
                }
            }
        }
    }
    sum
}

fn parse_game(line: &str) -> Result<Game, &'static str> {
    let line = &line[5..];
    let parts: Vec<&str> = line.split(": ").collect();
    let id = u32::from_str_radix(parts[0], 10).expect("could not parse id");
    let rounds: Vec<&str> = parts[1].split("; ").collect();
    let mut game = Game::new(id);
    for r in rounds {
        let cubes: Vec<&str> = r.split(", ").collect();
        let mut round = Round::new();
        for c in cubes {
            let cube: Vec<&str> = c.split(' ').collect();
            let quantity =
                u32::from_str_radix(cube[0], 10).expect("could not parse number of cubes");
            let colour: Colour = parse_colour(cube[1])?;
            round.add_cube(Cube::from(colour, quantity));
        }
        game.add_round(round);
    }
    Ok(game)
}

fn parse_colour(colour: &str) -> Result<Colour, &'static str> {
    match colour {
        "green" => Ok(Colour::Green),
        "blue" => Ok(Colour::Blue),
        "red" => Ok(Colour::Red),
        _ => Err("Failed to parse colour of cube"),
    }
}

// shamelessly stolen from Rust By Example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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
    fn test_calculate_power() {
        let input = Round::from(vec![
            Cube::from(Colour::Red, 4),
            Cube::from(Colour::Green, 2),
            Cube::from(Colour::Blue, 6),
        ]);

        assert_eq!(48, input.calculate_power());
    }

    #[test]
    fn test_get_min_cubes() {
        let input = Game::from(
            1,
            vec![
                Round::from(vec![
                    Cube::from(Colour::Blue, 3),
                    Cube::from(Colour::Red, 4),
                ]),
                Round::from(vec![
                    Cube::from(Colour::Red, 1),
                    Cube::from(Colour::Green, 2),
                    Cube::from(Colour::Blue, 6),
                ]),
                Round::from(vec![Cube::from(Colour::Green, 2)]),
            ],
        );

        let expected = Round::from(vec![
            Cube::from(Colour::Red, 4),
            Cube::from(Colour::Green, 2),
            Cube::from(Colour::Blue, 6),
        ]);

        assert_eq!(expected, input.get_min_cubes());
    }
}
