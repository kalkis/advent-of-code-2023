use day_01::{calibration_sum, read_lines};

fn main() {
    let input: Vec<String> = read_lines("./input.txt");
    println!("{}", calibration_sum(input));
}
