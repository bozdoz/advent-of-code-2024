use std::{ fs, time::Instant, vec };
use lib::get_part;

struct Equation {
    test: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn is_truthy(&self, get_next: impl Fn(&mut Vec<(usize, usize)>, (usize, usize)) -> ()) -> bool {
        let len = self.numbers.len();

        // Vec<(value, next index)>
        let mut states = vec![(self.numbers[0], 1)];

        while let Some((acc, i)) = states.pop() {
            // handle finished state
            if i == len && acc == self.test {
                return true;
            }

            if i == len {
                continue;
            }

            // get next states
            get_next(&mut states, (acc, i));
        }

        false
    }
}

fn get_equations(data: &str) -> Vec<Equation> {
    data.lines()
        .map(|l| {
            let (a, b) = l.split_once(": ").expect("': '");

            let numbers = b
                .split(" ")
                .map(|n| n.parse().expect("number to be number"))
                .collect();

            Equation {
                test: a.parse().expect("test was number"),
                numbers,
            }
        })
        .collect()
}

fn part_one(equations: &Vec<Equation>) -> usize {
    equations
        .iter()
        .filter_map(|eq| {
            if
                eq.is_truthy(|states: &mut Vec<(usize, usize)>, next: (usize, usize)| {
                    let acc = next.0;
                    let i = next.1;
                    let num = eq.numbers[i];
                    states.push((acc + num, i + 1));
                    states.push((acc * num, i + 1));
                })
            {
                return Some(eq.test);
            }
            None
        })
        .sum()
}

fn part_two(equations: &Vec<Equation>) -> usize {
    equations
        .iter()
        .filter_map(|eq| {
            if
                eq.is_truthy(|states: &mut Vec<(usize, usize)>, next: (usize, usize)| {
                    // same as part one
                    let acc = next.0;
                    let i = next.1;
                    let num = eq.numbers[i];
                    states.push((acc + num, i + 1));
                    states.push((acc * num, i + 1));

                    // adds concatenated to part two
                    let concatenated = acc * (10usize).pow(num.ilog10() + 1) + num;
                    states.push((concatenated, i + 1));
                })
            {
                return Some(eq.test);
            }
            None
        })
        .sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let equations = get_equations(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&equations);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&equations);
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
        let data = get_equations(EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 3749);
    }

    #[test]
    fn test_part_two() {
        let data = get_equations(EXAMPLE);
        let ans = part_two(&data);

        assert_eq!(ans, 11387);
    }
}
