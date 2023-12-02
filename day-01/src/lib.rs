use std::{collections::HashMap, fs::read_to_string};

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn parse_calibration_value(line: String) -> u32 {
    let filtered: Vec<u32> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let first_digit = filtered.first().unwrap();
    let second_digit = filtered.last().unwrap();

    (first_digit * 10) + second_digit
}

pub fn calibration_sum(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        sum += parse_calibration_value(line)
    }
    sum
}

pub fn calibration_sum_2(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        sum += parse_calibration_value_2(line)
    }
    sum
}

pub fn parse_calibration_value_2(line: String) -> u32 {
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut values: HashMap<usize, u32> = HashMap::new();
    for (k, v) in numbers {
        let found: Vec<usize> = line.match_indices(k).map(|(i, _)| i).collect();
        for i in found {
            values.insert(i, v);
        }
    }
    let first = values.keys().min().unwrap();
    let last = values.keys().max().unwrap();
    (values.get(first).unwrap() * 10) + values.get(last).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = vec![
            String::from("1abc2"),       // 12
            String::from("pqr3stu8vwx"), // 38
            String::from("a1b2c3d4e5f"), // 15
            String::from("treb7uchet"),  // 77
        ];
        assert_eq!(parse_calibration_value(input[1].clone()), 38);
        assert_eq!(parse_calibration_value(input[3].clone()), 77);
        assert_eq!(calibration_sum(input), 142);
    }

    #[test]
    fn example2() {
        let input = vec![
            String::from("two1nine"),              // 29
            String::from("eightwothree"),          // 83
            String::from("abcone2threexyz"),       // 13
            String::from("xtwone3four"),           // 24
            String::from("4nineeightseven2"),      // 42
            String::from("zoneight234"),           // 14
            String::from("7pqrstsixteen"),         // 76
            String::from("8twosvdmcntf1hfive393"), // 83
        ];
        assert_eq!(parse_calibration_value_2(input[1].clone()), 83);
        assert_eq!(parse_calibration_value_2(input[5].clone()), 14);
        assert_eq!(parse_calibration_value_2(input[6].clone()), 76);
        assert_eq!(parse_calibration_value_2(input[7].clone()), 83);
        assert_eq!(calibration_sum_2(input), 364);
    }
}
