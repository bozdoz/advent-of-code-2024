use std::{ collections::VecDeque, fs, time::Instant };
use lib::get_part;

fn evolve(secret: usize) -> usize {
    // 2^5 2^6 2^11
    // mix then prune
    let mut secret = ((secret << 6) ^ secret) % 16_777_216;

    secret = ((secret >> 5) ^ secret) % 16_777_216;

    ((secret << 11) ^ secret) % 16_777_216
}

fn evolve_loop(secret: usize, i: usize) -> usize {
    let mut secret = secret;

    for _ in 0..i {
        secret = evolve(secret);
    }

    secret
}

fn part_one(nums: &str) -> usize {
    nums.lines()
        .map(|x| evolve_loop(x.parse::<usize>().unwrap(), 2000))
        .sum()
}

// didn't use this
fn _prices_and_changes(secret: usize) -> [[i32; 4]; 10] {
    let mut out = [[-10; 4]; 10];
    // keep track of each completed and count of done
    let mut completed = [false; 10];
    let mut done = 0;
    let mut last_four = VecDeque::new();

    let mut update_last = |price: usize, diff: i32, i: i32| -> bool {
        last_four.push_back(diff);

        if i > 3 {
            last_four.pop_front();
        }

        // better to track iterations or last_four len?
        if i > 2 {
            // we have four
            if !completed[price] {
                completed[price] = true;
                done += 1;
                for i in 0..4 {
                    out[price][i] = last_four[i];
                }

                if done == 9 {
                    return true;
                }
            }
        }

        false
    };

    let mut secret = secret;
    let mut current_price = secret % 10;

    for i in 0..2000 {
        secret = evolve(secret);
        let next_price = secret % 10;

        let diff = (next_price as i32) - (current_price as i32);

        // can't use `done` in the loop, because it's used in the closure
        // so we return a bool here to break
        if update_last(next_price, diff, i) {
            break;
        }

        current_price = next_price;
    }

    out
}

const TWENTY_BITS: usize = (2usize).pow(20) - 1;

fn _print_seq(bin: &usize) {
    let mut bin = *bin;
    let mut seq = [0, 0, 0, 0];

    for i in (0..4).rev() {
        let last_five = (bin & 0b11111) as isize;
        seq[i] = last_five - 9;
        bin >>= 5;
    }

    println!("{seq:?}");
}

fn sell_at_seq(secret: usize, seq: usize) -> usize {
    let mut last_four: usize = 0;
    let mut secret = secret;
    let mut current_price = secret % 10;

    for _ in 0..3 {
        secret = evolve(secret);
        let next_price = secret % 10;

        let diff = (next_price as i32) - (current_price as i32);

        current_price = next_price;

        // push back
        last_four <<= 5;
        // push another
        last_four |= (9 + diff) as usize;
    }

    for _ in 3..2000 {
        secret = evolve(secret);
        let next_price = secret % 10;

        let diff = (next_price as i32) - (current_price as i32);

        current_price = next_price;

        // push back (5 is enough to push 18)
        last_four <<= 5;
        // push another
        // convert -9 -> 9 to 0 -> 18
        last_four |= (9 + diff) as usize;

        // keep 20 bits
        last_four &= TWENTY_BITS;

        if (last_four ^ seq) == 0 {
            return current_price;
        }
    }

    0
}

fn part_two(data: &str) -> usize {
    let nums = data
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut best = 0;
    let mut best_seq = 0;

    // TODO: this is brute force, but we could iterate all secrets once
    // and save a hashmap of every sequence way faster
    for a in -8..=5 {
        for b in -7..=6 {
            if ((a + b) as i32).abs() > 9 {
                continue;
            }
            for c in -6..=7 {
                if ((b + c) as i32).abs() > 9 {
                    continue;
                }
                if ((a + b + c) as i32).abs() > 9 {
                    continue;
                }
                for d in -5..=8 {
                    if ((c + d) as i32).abs() > 9 {
                        continue;
                    }
                    if ((b + c + d) as i32).abs() > 9 {
                        continue;
                    }
                    if ((a + b + c + d) as i32).abs() > 9 {
                        continue;
                    }
                    let seq = [a, b, c, d]
                        .into_iter()
                        // add 9 to convert -9 -> 9 to 0 -> 18
                        .map(|x| (9 + x) as usize)
                        // shift 5 to account for 18
                        .reduce(|acc, cur| (acc << 5) | cur)
                        .unwrap();

                    let sum = nums
                        .iter()
                        .map(|&x| {
                            // find four-price sequence in every number
                            let ans = sell_at_seq(x, seq);

                            ans
                        })
                        .sum();

                    if sum > best {
                        best = sum;
                        best_seq = seq;
                    }
                }
            }
        }
    }

    let mut seq = [0, 0, 0, 0];

    for i in (0..4).rev() {
        let last_five = (best_seq & 0b11111) as isize;
        // go back to -9..9
        seq[i] = last_five - 9;
        best_seq >>= 5;
    }

    println!("{best} {seq:?}");

    best
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

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
    fn test_evolve() {
        assert_eq!(evolve_loop(123, 1), 15887950);
        assert_eq!(evolve_loop(123, 2), 16495136);
    }

    #[test]
    fn test_prices_and_changes() {
        let changes = _prices_and_changes(123);

        assert_eq!(changes[6], [-1, -1, 0, 2])
    }

    #[test]
    fn test_part_one() {
        let ans = part_one(EXAMPLE);

        assert_eq!(ans, 37327623);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two("1
2
3
2024");

        assert_eq!(ans, 23);
    }
}
