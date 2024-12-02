use std::{ cmp::Ordering, fs, time::Instant };
use lib::get_part;

fn parse_data(data: &str) -> Vec<Vec<isize>> {
    data.lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|y| { y.parse().expect("I thought this was a number") })
                .collect()
        })
        .collect()
}

fn part_one(reports: &Vec<Vec<isize>>) -> usize {
    reports
        .iter()
        .filter_map(|x| {
            let mut iter = x.iter();

            let diff = iter.next().expect("first") - iter.next().expect("second");

            if diff == 0 {
                return None;
            }

            let order = if diff < 0 { Ordering::Less } else { Ordering::Greater };

            // get second again
            let mut iter = x.iter().skip(1);
            let mut current = iter.next().expect("we just used this");

            while let Some(a) = iter.next() {
                let diff = current.abs_diff(*a);
                // don't need diff < 0 because it's covered in the `cmp`
                if diff > 3 || current.cmp(a) != order {
                    if current.cmp(a) != order {
                        println!("{:?} {} {}", x, current, a);
                    }
                    return None;
                }
                current = a;
            }

            Some(x)
        })
        .count()
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let data = parse_data(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&data);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two();
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_parser() {
        assert_eq!(parse_data("1 2 3 4
5 6 7 8"), vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
    }

    #[test]
    fn test_part_one() {
        let data = parse_data(&EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 2);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}
