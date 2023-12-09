use day_06::{beat_race_record, combine_nums, parse_file, read_lines};

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        match parse_file(lines) {
            Ok((times, distances)) => {
                // combine numbers into single number for each vector
                if (times.len() == 0) || (distances.len() == 0) {
                    eprintln!("No times or distances to process");
                } else {
                    let time = combine_nums(times);
                    let distance = combine_nums(distances);
                    match beat_race_record(time, distance) {
                        Ok(record) => println!("Number of ways to beat record: {record}"),
                        Err(e) => eprintln!("{e}"),
                    }
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
