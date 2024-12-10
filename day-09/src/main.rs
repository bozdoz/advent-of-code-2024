use std::{ fs, iter::repeat_n, time::Instant };
use lib::get_part;

// build a full vector of all digits, and fill with Some(usize) or None to represent empty "."
fn parse_data(data: &str) -> Vec<Option<usize>> {
    let mut out = vec![];
    let mut chars = data.chars();
    let mut id = 0;

    for &is_file in [true, false].iter().cycle() {
        if let Some(ch) = chars.next() {
            if let Some(digit) = ch.to_digit(10) {
                if is_file {
                    out.append(
                        &mut repeat_n(id, digit as usize)
                            .map(|x| Some(x))
                            .collect()
                    );
                    id += 1;
                } else {
                    // append empties
                    out.append(&mut repeat_n(None, digit as usize).collect());
                }
            }
        } else {
            break;
        }
    }

    out
}

fn part_one(data: &Vec<Option<usize>>) -> usize {
    // move files one at a time
    let mut copy = data.clone();
    let mut s = 0;
    let mut e = data.len() - 1;

    loop {
        // each numbers towards each other
        while copy[s].is_some() {
            s += 1;
        }

        while copy[e].is_none() {
            e -= 1;
        }

        if e <= s {
            break;
        }

        // s is none; e is some; swap
        copy.swap(s, e);
    }

    // map_while works because there are no empty gaps in between
    copy.iter()
        .map_while(|&x| x)
        .enumerate()
        .fold(0, |acc, (i, v)| { acc + i * v })
}

fn part_two(data: &Vec<Option<usize>>) -> usize {
    // move complete files EXACTLY ONCE from the right
    let mut copy = data.clone();
    // start is first None
    let s = copy
        .iter()
        .position(|x| x.is_none())
        .unwrap();
    let mut e = data.len() - 1;
    // keep track of which files have moved EXACTLY ONCE
    let mut moved = vec![false; copy.last().unwrap().unwrap()];

    // s never really changes here, unfortunately
    while s < e {
        if let Some(item) = copy[e] {
            let need = (0..=e)
                .rev()
                .take_while(|&x| { copy[x] == Some(item) })
                .count();

            if moved[item - 1] {
                e -= need;
                continue;
            }

            // look from left for empty space
            // println!("looking for: {item} of size {need}, with {s} and {e}");

            let mut cur = s;

            loop {
                while copy[cur].is_some() {
                    cur += 1;
                }
                if cur > e - need {
                    // can't find space; move to next number from the right
                    e -= need;
                    break;
                }
                let available = copy[cur..]
                    .iter()
                    .take_while(|x| x.is_none())
                    .count();

                if available >= need {
                    for i in cur..cur + need {
                        copy.swap(i, e);
                        // update end
                        e -= 1;
                    }

                    moved[item - 1] = true;
                    break;
                } else {
                    cur += available;
                }
            }
        } else {
            e -= 1;
        }
    }

    copy.iter()
        .enumerate()
        .fold(0, |acc, (i, opt)| {
            if let Some(v) = opt {
                // ignore None's
                return acc + i * v;
            }
            acc
        })
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let puzzle = parse_data(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&puzzle);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&puzzle);
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
        let data = parse_data(EXAMPLE);
        let ans = part_one(&data);

        assert_eq!(ans, 1928);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data(EXAMPLE);
        let ans = part_two(&data);

        assert_eq!(ans, 2858);
    }
}
