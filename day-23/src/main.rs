#![allow(unused)]

use std::{ collections::{ HashMap, HashSet }, fs, time::Instant };
use lib::get_part;

fn get_networks(data: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in data.lines() {
        let (a, b) = line.split_once("-").unwrap();

        map.entry(a)
            .and_modify(|x| {
                x.insert(b);
            })
            .or_insert_with(|| { HashSet::from([b]) });

        map.entry(b)
            .and_modify(|x| {
                x.insert(a);
            })
            .or_insert_with(|| { HashSet::from([a]) });
    }

    map
}

fn part_one(networks: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut trios = HashSet::new();

    for (a, a_set) in networks.iter() {
        // println!("{a} {a_set:?}");

        for b in a_set.iter() {
            for c in networks[b].iter() {
                if a_set.contains(c) {
                    let mut nets = [a, b, c];
                    nets.sort();

                    trios.insert(nets);
                }
            }
        }
    }

    trios
        .iter()
        .filter(|x| {
            for a in x.iter() {
                if a.starts_with('t') {
                    return true;
                }
            }

            false
        })
        .count()
}

fn part_two(networks: &HashMap<&str, HashSet<&str>>) -> String {
    let mut biggest = HashMap::new();

    for (a, a_set) in networks.iter() {
        let mut visited = HashSet::from([a]);
        let mut cur = HashSet::from([a]);
        let mut queue = a_set.into_iter().collect::<Vec<_>>();

        while let Some(v) = queue.pop() {
            if visited.contains(&v) {
                continue;
            }

            if networks[v].contains(a) {
                // keep going
                cur.insert(v);

                for b in networks[v].iter() {
                    queue.push(b);
                }
            }

            visited.insert(v);
        }

        biggest.insert(a, cur);
    }

    let mut counts = HashMap::new();

    // check for intersections in all queues
    for (a, set) in biggest.iter() {
        for b in set {
            if a == b {
                continue;
            }
            let other = &biggest[b];
            let mut intersection = set.intersection(other).collect::<Vec<_>>();

            intersection.sort();

            counts
                .entry(intersection)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }
    }

    let mut biggest = (0, vec![]);

    for (k, v) in counts {
        if v > biggest.0 {
            biggest = (v, k);
        }
    }

    biggest.1
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let networks = get_networks(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&networks);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&networks);
        println!("Part two: {} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let ans = part_one(&get_networks(EXAMPLE));

        assert_eq!(ans, 7);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&get_networks(EXAMPLE));

        assert_eq!(ans, "co,de,ka,ta");
    }
}
