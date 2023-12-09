use day_09::{parse_line, predict_next_value, read_lines};

fn main() {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(l) = line {
                // to find previous value, just reverse the list
                let numbers: Vec<i64> = parse_line(&l).into_iter().rev().collect();
                sum += predict_next_value(&numbers);
            }
        }
    }
    println!("sum of previous values for all histories is: {sum}");
}
