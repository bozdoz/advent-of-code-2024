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

fn has_issues(report: &Vec<isize>) -> Option<isize> {
    let mut iter = report.iter();

    let diff = iter.next().expect("first") - iter.next().expect("second");

    if diff == 0 {
        return Some(1);
    }

    let order = if diff < 0 { Ordering::Less } else { Ordering::Greater };

    // reset the iterator
    let mut iter = report.iter();
    let mut current = iter.next().expect("we just used this");
    let mut i = 1;

    while let Some(a) = iter.next() {
        let diff = current.abs_diff(*a);
        // don't need diff < 0 because it's covered in the `cmp`
        if diff > 3 || current.cmp(a) != order {
            return Some(i);
        }
        current = a;
        i += 1;
    }

    // no issues
    None
}

fn part_one(reports: &Vec<Vec<isize>>) -> usize {
    reports
        .iter()
        .filter(|x| { has_issues(x).is_none() })
        .count()
}

fn part_two(reports: &Vec<Vec<isize>>) -> usize {
    reports
        .iter()
        .filter_map(|x| {
            // SUPER ULTRA LAZY just iterate everything...
            for i in 0..x.len() {
                // WOW: rust is difficult to fight with
                let clone: Vec<isize> = x
                    .iter()
                    .enumerate()
                    .filter_map(|(j, v)| {
                        if i == j {
                            return None;
                        }
                        Some(*v)
                    })
                    .collect();

                if has_issues(&clone).is_none() {
                    return Some(x);
                }
            }
            None
        })
        .count()
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
        let ans = part_two(&data);
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
    fn test_reddit() {
        let data = parse_data("1 9");
        let ans = part_one(&data);

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_one() {
        let data = parse_data(&EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 2);
    }

    #[test]
    fn test_one_bad() {
        assert_eq!(part_two(&parse_data("1 2 8 9")), 0);
        assert_eq!(part_two(&parse_data("9 1 2 3 9")), 0);
        assert_eq!(part_two(&parse_data("9 1 2 3")), 1);
        assert_eq!(part_two(&parse_data("1 9 2 3")), 1);
        assert_eq!(part_two(&parse_data("1 2 9 3")), 1);
        assert_eq!(part_two(&parse_data("1 2 3 9")), 1);
        assert_eq!(part_two(&parse_data("3 2 1 9")), 1);
        assert_eq!(part_two(&parse_data("3 2 9 1")), 1);
        assert_eq!(part_two(&parse_data("3 9 2 1")), 1);
        assert_eq!(part_two(&parse_data("9 3 2 1")), 1);
        assert_eq!(part_two(&parse_data("1 1 3 4")), 1);
        assert_eq!(part_two(&parse_data("1 2 2 4")), 1);
        assert_eq!(part_two(&parse_data("1 2 3 3")), 1);
        assert_eq!(part_two(&parse_data("3 2 1 1")), 1);
        assert_eq!(part_two(&parse_data("3 2 2 1")), 1);
        assert_eq!(part_two(&parse_data("3 3 2 1")), 1);
        assert_eq!(part_two(&parse_data("1 3 2 4")), 1);
        assert_eq!(part_two(&parse_data("1 4 2 7")), 1);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data(&EXAMPLE);
        let ans = part_two(&data);

        assert_eq!(ans, 4);
    }

    fn has_issues_test_failed(report: &Vec<isize>) -> Option<isize> {
        let mut iter = report.iter();

        let diff = iter.next().expect("first") - iter.next().expect("second");

        if diff == 0 || diff.abs() > 3 {
            return Some(1);
        }

        let order = if diff < 0 { Ordering::Less } else { Ordering::Greater };

        // get second again
        let mut iter = report.iter();
        let mut current = iter.next().expect("we just used this");
        let mut i = 1;

        while let Some(a) = iter.next() {
            let diff = current.abs_diff(*a);
            // don't need diff < 0 because it's covered in the `cmp`
            if diff > 3 || current.cmp(a) != order {
                return Some(i);
            }
            current = a;
            i += 1;
        }

        // no issues
        None
    }

    fn part_two_test_failed(reports: &Vec<Vec<isize>>) -> usize {
        reports
            .iter()
            .filter_map(|x| {
                if let Some(index) = has_issues_test_failed(x) {
                    dbg!(index);
                    // try again without the index
                    // WOW: rust is difficult to fight with
                    let clone: Vec<isize> = x
                        .iter()
                        .enumerate()
                        .filter_map(|(i, v)| {
                            if i == (index as usize) {
                                return None;
                            }
                            Some(*v)
                        })
                        .collect();

                    if has_issues_test_failed(dbg!(&clone)).is_some() {
                        dbg!(index - 1);
                        // LAZY
                        // try one last time with the OTHER index
                        let clone: Vec<isize> = x
                            .iter()
                            .enumerate()
                            .filter_map(|(i, v)| {
                                if i == ((index - 1) as usize) {
                                    return None;
                                }
                                Some(*v)
                            })
                            .collect();

                        if has_issues_test_failed(dbg!(&clone)).is_some() {
                            return None;
                        }
                    }
                }
                println!("{:?}", x);
                Some(x)
            })
            .count()
    }

    #[test]
    #[ignore]
    fn test_failed_part_two() {
        assert_eq!(part_two_test_failed(&parse_data("47 45 46 47 49")), 1);
    }
}
