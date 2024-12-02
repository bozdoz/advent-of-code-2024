use std::{ time::Instant, fs };
use lib::get_part;

fn parse_input(s: &str) -> (Vec<usize>, Vec<usize>) {
    let mut first = vec![];
    let mut second = vec![];
    for line in s.lines() {
        let (a, b) = line.split_once("   ").expect("three spaces");

        first.push(a.parse().unwrap());
        second.push(b.parse().unwrap());
    }

    first.sort();
    second.sort();

    (first, second)
}

fn part_one(data: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut sum = 0;
    let (first, second) = data;

    for i in 0..first.len() {
        let diff = first[i].max(second[i]) - first[i].min(second[i]);

        sum += diff;
    }

    sum
}

fn part_two(data: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut sum = 0;
    let (first, second) = data;
    let mut last = (0, 0);
    let mut j = 0;

    for a in first.iter() {
        // if this the same number as the last one, add the last sum
        if *a == last.0 {
            sum += last.0 * last.1;
            continue;
        }
        // reset last match + count
        last = (*a, 0);

        for b in second[j..].iter() {
            // we've gone past the number
            if b > a {
                break;
            }
            // begins count
            if b == a {
                last.1 += 1;
            }
            // increment slice (useless optimization)
            j += 1;
        }

        sum += last.0 * last.1;
    }

    sum
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let data = parse_input(&contents);

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
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_parser() {
        let parsed = parse_input("1   4
3   3
2   5
");

        assert_eq!(parsed, (vec![1, 2, 3], vec![3, 4, 5]));
    }

    #[test]
    fn test_part_one() {
        let parsed = parse_input(EXAMPLE);
        let ans = part_one(&parsed);

        assert_eq!(ans, 11);
    }

    #[test]
    fn test_part_two() {
        let parsed = parse_input(EXAMPLE);
        let ans = part_two(&parsed);

        assert_eq!(ans, 31);
    }
}
