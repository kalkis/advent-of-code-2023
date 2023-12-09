use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::zip,
    path::Path,
};

pub fn combine_nums(nums: Vec<f64>) -> f64 {
    let mut new_num = String::new();
    for n in nums {
        new_num.push_str(n.to_string().as_str());
    }
    new_num.parse::<f64>().unwrap()
}

pub fn product_of_race_records(times: Vec<f64>, distances: Vec<f64>) -> Option<f64> {
    if (times.len() == 0) || (distances.len() == 0) {
        None
    } else {
        let product: f64 = zip(times, distances)
            .map(|x| beat_race_record(x.0, x.1).unwrap_or(1.0))
            .reduce(|x, y| x * y)
            .unwrap();
        Some(product)
    }
}

pub fn parse_file(lines: Lines<BufReader<File>>) -> Result<(Vec<f64>, Vec<f64>), &'static str> {
    //let lines = read_to_string(filename).expect("Failed to read file");
    let mut times = vec![];
    let mut distances = vec![];
    for line in lines {
        match line {
            Ok(l) => {
                if l.starts_with("Time: ") {
                    times = parse_line(&l[5..])?;
                } else if l.starts_with("Distance:") {
                    distances = parse_line(&l[9..])?;
                }
            }
            Err(_) => return Err("Could not read line"),
        }
    }
    Ok((times, distances))
}

// shamelessly stolen from Rust By Example
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_line(line: &str) -> Result<Vec<f64>, &'static str> {
    Ok(line
        .split(' ')
        .filter(|c| !c.is_empty())
        .map(|s| s.parse::<f64>().expect("Failed to parse number in list"))
        .collect())
}

// equation of race
// let T be time limit of race
// let D be the distance to travel
// let t be the time button is held
// since distance = speed * time
// and speed is equal to t
// and time is equal to T - t
// D = t * (T - t)
// or -t^2 + Tt - D = 0
// to beat the race we need the values of t where
// Tt - t^2 > D
pub fn beat_race_record(time: f64, distance: f64) -> Result<f64, &'static str> {
    match solve_quadratic(-1.0, time, -distance) {
        Some(roots) => {
            if roots.0 == roots.1 {
                Ok(1.0)
            // if both roots are integers, subtract 1 to ensure we don't include lower bound as an answer
            } else if (roots.0.ceil() == roots.0) && (roots.1.ceil() == roots.1) {
                Ok((roots.0 - roots.1).abs() - 1.0)
            } else {
                Ok((roots.0.ceil() - roots.1.ceil()).abs())
            }
        }
        None => Err("No solution found"),
    }
}

// doesn't handle imaginary roots
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant: f64 = (b * b) - 4.0 * a * c;
    match discriminant.partial_cmp(&0.0) {
        Some(order) => match order {
            // roots are imaginary
            Ordering::Less => None,
            // 1 real solution
            Ordering::Equal => {
                let root1 = -b / (2.0 * a);
                let root2 = root1;
                Some((root1, root2))
            }
            // 2 real solutions
            Ordering::Greater => {
                let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
                let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
                Some((root1, root2))
            }
        },
        // if we got NAN, somehow
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quadratic() {
        assert_eq!(Some((10.0, 20.0)), solve_quadratic(-1.0, 30.0, -200.0));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Ok(vec![21.0, 35.0, 78.0, 99.0]),
            parse_line("21   35   78   99"),
        );
    }

    #[test]
    fn test_combine_nums() {
        assert_eq!(21357899.0, combine_nums(vec![21.0, 35.0, 78.0, 99.0]));
        assert_eq!(
            400121310111540.0,
            combine_nums(vec![400.0, 1213.0, 1011.0, 1540.0])
        );
    }

    #[test]
    fn test_product_of_race_records() {
        assert_eq!(
            Some(288.0),
            product_of_race_records(vec![7.0, 15.0, 30.0], vec![9.0, 40.0, 200.0])
        );
        assert_eq!(
            Some(1660968.0),
            product_of_race_records(
                vec![47.0, 98.0, 66.0, 98.0],
                vec![400.0, 1213.0, 1011.0, 1540.0]
            )
        )
    }
}
