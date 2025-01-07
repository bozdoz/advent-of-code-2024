use std::{ time::Instant, fs };
use lib::get_part;

struct SafetyManual<'a> {
    rules: Vec<(&'a str, &'a str)>,
    pages: Vec<Vec<&'a str>>,
}

impl<'a> SafetyManual<'a> {
    // first where; I don't understand it
    fn new<'b>(data: &'b str) -> Self where 'b: 'a {
        let (a, b) = data.split_once("\n\n").expect("What, no space!?");

        let rules: Vec<(&str, &str)> = a
            .lines()
            .map(|l| l.split_once("|").expect("What, no pipe!?"))
            .collect();

        let pages: Vec<Vec<&str>> = b
            .lines()
            .map(|l| l.split(",").collect())
            .collect();

        Self { rules, pages }
    }

    fn get_failed(&self) -> Vec<bool> {
        let len = self.pages.len();
        let mut failed = vec![false; len];

        for (first, second) in self.rules.iter() {
            for p in 0..len {
                if failed[p] {
                    continue;
                }
                if let Some(s) = self.pages[p].iter().position(|x| x == second) {
                    // found second, let's see if first comes after
                    if self.pages[p][s..].contains(first) {
                        // println!("Found {} after {}; failed: {:?}", first, second, manual.pages[p]);
                        failed[p] = true;
                    }
                }
            }
        }

        failed
    }
}

fn part_one(manual: &SafetyManual) -> usize {
    let failed = manual.get_failed();

    manual.pages
        .iter()
        .enumerate()
        .filter_map(|p| {
            if failed[p.0] {
                return None;
            }
            let mid = p.1[p.1.len() / 2];

            // println!("Mid: {}", mid);

            Some(mid.parse::<usize>().expect("Thought it was a number :("))
        })
        .sum()
}

fn part_two(manual: &SafetyManual) -> usize {
    let failed = manual.get_failed();

    let failed_pages: Vec<_> = manual.pages
        .iter()
        .enumerate()
        .filter_map(|p| {
            if failed[p.0] {
                return Some(p.1.clone());
            }
            None
        })
        .collect();

    let mut sum = 0;
    // first for mut?
    for mut page in failed_pages {
        loop {
            let mut swapped = false;

            'outer: for &(first, second) in manual.rules.iter() {
                for i in 0..page.len() {
                    if page[i] == first {
                        continue 'outer;
                    }
                    if page[i] == second {
                        // look for first and swap
                        for j in i..page.len() {
                            if page[j] == first {
                                page.swap(i, j);
                                swapped = true;
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            if !swapped {
                sum += page[page.len() / 2].parse::<usize>().expect("it to be a number");
                break;
            }
        }
    }

    sum
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let data = SafetyManual::new(&contents);

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
    fn test_part_one() {
        let data = SafetyManual::new(EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 143);
    }

    #[test]
    fn test_part_two() {
        let data = SafetyManual::new(EXAMPLE);
        let ans = part_two(&data);

        assert_eq!(ans, 123);
    }
}
