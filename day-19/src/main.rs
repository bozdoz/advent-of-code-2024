use std::{ collections::{ BinaryHeap, HashMap, HashSet }, fs, time::Instant, vec };
use lib::get_part;

#[derive(Debug)]
struct Towels<'a> {
    available: HashSet<&'a str>,
    desired: Vec<&'a str>,
    largest: usize,
}

impl<'a> Towels<'a> {
    fn new(data: &'a str) -> Self {
        let (a, d) = data.split_once("\n\n").expect("empty line");

        let available = HashSet::from_iter(a.split(", "));
        let desired = d.lines().collect();

        let largest = available
            .iter()
            .map(|x| x.len())
            .max()
            .unwrap();

        Self { available, desired, largest }
    }

    fn dfs(&self, search: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(&count) = memo.get(search) {
            return count;
        }
        let mut count = 0;

        // cuts about half the time: 35ms to 12ms
        let len = search.len().min(self.largest);

        // get a range for the string slice
        for e in 1..=len {
            // find a pattern
            if self.available.contains(&search[..e]) {
                // if we're at the end, then this is 1 pattern,
                // otherwise, search the rest of the string
                count += if e == search.len() { 1 } else { self.dfs(&search[e..], memo) };
            }
        }

        memo.insert(search, count);

        count
    }

    fn possible(&self, desired: &str) -> usize {
        self.dfs(desired, &mut HashMap::new())
    }
}

#[derive(PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other.cmp(self) is min-heap
        // self.cmp(other) is max-heap
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn can_make_towel(towels: &Towels, desired: &str) -> bool {
    // start with window 1
    // expand and iterate back at window 1
    // (index, chunk)
    let mut visited = vec![false; desired.len()];
    let mut heap = BinaryHeap::new();

    heap.push(Range {
        start: 0,
        end: 1,
    });

    while let Some(range) = heap.pop() {
        if range.start == desired.len() {
            return true;
        }

        if visited[range.start] {
            continue;
        }

        // get next states
        for end in range.end..=desired.len() {
            let chunk = &desired[range.start..end];

            if towels.available.contains(&chunk) {
                heap.push(Range {
                    start: end,
                    end: end + 1,
                });
            }
        }

        visited[range.start] = true;
    }

    false
}

fn part_one(towels: &Towels) -> usize {
    towels.desired
        .iter()
        .filter(|d| can_make_towel(towels, d))
        .count()
}

fn part_two(towels: &Towels) -> usize {
    towels.desired
        .iter()
        .map(|d| towels.possible(d))
        .sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let towels = Towels::new(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&towels);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&towels);
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
        let ans = part_one(&Towels::new(EXAMPLE));

        assert_eq!(ans, 6);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Towels::new(EXAMPLE));

        assert_eq!(ans, 16);
    }
}
