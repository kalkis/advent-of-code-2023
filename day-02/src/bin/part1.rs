use day_02::{sum_valid_games, Colour};
use std::collections::HashMap;

fn main() {
    let valid: HashMap<Colour, u32> =
        HashMap::from([(Colour::Red, 12), (Colour::Blue, 14), (Colour::Green, 13)]);
    let sum = sum_valid_games("input.txt", valid);

    println!("Sum of ids for valid games is: {sum}");
}
