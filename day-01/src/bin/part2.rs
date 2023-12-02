use day_01::{calibration_sum_2, read_lines};

fn main() {
    let input: Vec<String> = read_lines("./input.txt");
    println!("{}", calibration_sum_2(input));
}
