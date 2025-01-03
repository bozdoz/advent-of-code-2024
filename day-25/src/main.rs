use std::{ time::Instant, fs };

#[derive(Debug)]
struct Tumbler {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

impl Tumbler {
    fn new(data: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];

        for item in data.split("\n\n") {
            let mut lines = item.lines();

            let is_lock = lines.next().unwrap() == "#####";
            let mut val = [0; 5];

            for _ in 0..5 {
                for (i, c) in lines.next().unwrap().chars().enumerate() {
                    if c == '#' {
                        val[i] += 1;
                    }
                }
            }

            if is_lock {
                locks.push(val);
            } else {
                keys.push(val);
            }
        }

        Self {
            keys,
            locks,
        }
    }
}

fn part_one(tumbler: &Tumbler) -> usize {
    // for key in tumbler.keys.iter() {
    //     'locks: for lock in tumbler.locks.iter() {
    //         for i in 0..5 {
    //             if key[i] + lock[i] > 5 {
    //                 continue 'locks;
    //             }
    //         }
    //         out += 1;
    //     }
    // }

    // refactored as idiomatic rust
    tumbler.keys
        .iter()
        .flat_map(|k| tumbler.locks.iter().map(move |l| (k, l)))
        .filter(|(k, l)| (0..5).all(|i| k[i] + l[i] < 6))
        .count()
}

fn main() {
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let tumbler = Tumbler::new(&data);

    let now = Instant::now();
    let ans = part_one(&tumbler);
    println!("Part one: {:?} {:?}", ans, now.elapsed());

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let ans = part_one(&Tumbler::new(EXAMPLE));

        assert_eq!(ans, 3);
    }
}
