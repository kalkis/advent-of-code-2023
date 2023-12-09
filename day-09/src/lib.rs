use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// shamelessly stolen from Rust By Example
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn deltas(numbers: &Vec<i64>) -> Vec<i64> {
    numbers
        // iterate over overlapping windows of size n
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

pub fn predict_next_value(numbers: &Vec<i64>) -> i64 {
    let diffs = deltas(numbers);
    if diffs.iter().all(|&d| d == 0) {
        // if all differences are 0, return last number in list
        return numbers[numbers.len() - 1];
    }
    // add last number in list to last number of next iteration
    numbers[numbers.len() - 1] + predict_next_value(&diffs)
}

pub fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            vec![1, 4, 5, 2, 16, 20, 304, -23, 19],
            parse_line("1 4 5 2 16 20 304 -23 19")
        );
    }

    #[test]
    fn test_deltas() {
        let input1 = vec![0, 3, 6, 9, 12, 15];
        let input2 = vec![10, 13, 16, 21, 30, 45];
        let input3 = vec![
            5, 0, -5, -10, -15, -20, -25, -30, -35, -40, -45, -50, -55, -60, -65, -70, -75, -80,
            -85, -90, -95,
        ];
        let input4 = vec![
            18, 20, 20, 18, 14, 8, 0, -10, -22, -36, -52, -70, -90, -112, -136, -162, -190, -220,
            -252, -286, -322,
        ];
        assert_eq!(vec![3, 3, 3, 3, 3], deltas(&input1));
        assert_eq!(vec![3, 3, 5, 9, 15], deltas(&input2));
        assert_eq!(
            vec![-5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5],
            deltas(&input3)
        );
        assert_eq!(
            vec![
                2, 0, -2, -4, -6, -8, -10, -12, -14, -16, -18, -20, -22, -24, -26, -28, -30, -32,
                -34, -36
            ],
            deltas(&input4)
        );
    }

    #[test]
    fn test_predict_next_value() {
        let input1 = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(18, predict_next_value(&input1));
        let input2: Vec<i64> = input1.into_iter().rev().collect();
        assert_eq!(-3, predict_next_value(&input2));
        let input3 = vec![
            5, 0, -5, -10, -15, -20, -25, -30, -35, -40, -45, -50, -55, -60, -65, -70, -75, -80,
            -85, -90, -95,
        ];
        assert_eq!(-100, predict_next_value(&input3));
        let input4: Vec<i64> = input3.into_iter().rev().collect();
        assert_eq!(10, predict_next_value(&input4));
    }
}
