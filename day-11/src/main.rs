use std::{ collections::HashMap, fs, mem, time::Instant };
use lib::get_part;

fn parse_data(data: &str) -> Vec<usize> {
    // first time trimming and usng filter_map with `ok`
    data.trim()
        .split(" ")
        .filter_map(|x| { x.parse::<usize>().ok() })
        .collect()
}

// wow, first macro (to cut 5 lines down to 1)
macro_rules! update_or_create {
    ($map:expr, $key:expr, $val:expr) => {
        {
            $map.entry($key)
                    .and_modify(|x| {
                        *x += $val;
                    })
                    .or_insert($val);
        }
    };
}

fn get_stone_count(data: &Vec<usize>, blinks: usize) -> usize {
    // numbers->counts
    let mut current: HashMap<usize, usize> = HashMap::new();

    for &datum in data {
        update_or_create!(current, datum, 1);
    }

    // take x blinks into eternity
    for _ in (0..).take(blinks) {
        // had to make sure these were the same types
        let mut next: HashMap<usize, usize> = HashMap::new();

        for (&k, &v) in current.iter() {
            if k == 0 {
                // pass the same number of items into the "1" key
                update_or_create!(next, 1, v);
                continue;
            }

            // get number of digits in the number
            let digits = k.ilog10() + 1;

            if digits % 2 == 0 {
                // split into 2
                let split = digits / 2;
                let pow = (10usize).pow(split);
                let left = k / pow;
                let right = k - left * pow;

                update_or_create!(next, left, v);
                update_or_create!(next, right, v);
            } else {
                // just multiply by 2024
                update_or_create!(next, k * 2024, v);
            }
        }

        // first mem::swap
        mem::swap(&mut current, &mut next);
    }

    current.values().sum()
}

// FYI: naive solution here: https://gist.github.com/bozdoz/934ee5fad305507e46b2367ef1e1e00c
fn part_one(data: &Vec<usize>) -> usize {
    get_stone_count(data, 25)
}

fn part_two(data: &Vec<usize>) -> usize {
    get_stone_count(data, 75)
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let data = parse_data(&data);

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
        let ans = part_one(&parse_data(EXAMPLE));

        assert_eq!(ans, 55312);
    }
}
