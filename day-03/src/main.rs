#![allow(unused)]
use std::{ time::Instant, fs };
use lib::get_part;
use regex::Regex;

fn parser(data: &str) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("I thought this was a valid regex");

    for a in re.captures_iter(data) {
        let (_full, [first, second]) = a.extract();
        out.push((first.parse().expect("first number"), second.parse().expect("second number")));
    }

    out
}

// ignores don'ts, applies do's
fn parser_two(data: &str) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").expect("re isn't valid?");
    let mulre = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("mulre isn't valid?");
    // track mul's initially
    let mut track = true;

    for a in re.captures_iter(data) {
        let (_full, [capture]) = a.extract();

        match capture {
            "do()" => {
                track = true;
            }
            "don't()" => {
                track = false;
            }
            _ => {
                // mul(digit, digit)
                if track {
                    let mul = mulre.captures(capture).expect("mul to have digits");

                    let (_full, [first, second]) = mul.extract();
                    out.push((
                        first.parse().expect("first number"),
                        second.parse().expect("second number"),
                    ));
                }
            }
        }
    }

    out
}

fn part_one(data: &Vec<(usize, usize)>) -> usize {
    data.iter().fold(0, |acc, &x| { acc + x.0 * x.1 })
}

fn part_two(data: &Vec<(usize, usize)>) -> usize {
    part_one(data)
}

fn main() {
    let (one, two) = get_part();

    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    if one {
        let now = Instant::now();
        let data = parser(&contents);
        let ans = part_one(&data);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let data_two = parser_two(&contents);
        let ans = part_two(&data_two);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parser() {
        assert_eq!(parser(&EXAMPLE), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_parser_two() {
        assert_eq!(parser_two(&EXAMPLE_2), vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_part_one() {
        let data = parser(&EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 161);
    }

    #[test]
    fn test_part_two() {
        let data = parser_two(&EXAMPLE_2);
        let ans = part_two(&data);

        assert_eq!(ans, 48);
    }
}
