use day_06::{parse_file, product_of_race_records, read_lines};

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        match parse_file(lines) {
            Ok((times, distances)) => match product_of_race_records(times, distances) {
                Some(product) => println!("Product of number of ways to beat race: {}", product),
                None => println!("Failed to calculate product"),
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}
