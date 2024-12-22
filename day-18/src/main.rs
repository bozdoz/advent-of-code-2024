use std::{ collections::{ BinaryHeap, HashSet }, fs, time::Instant };
use lib::{ get_part, tup, DIRS };

struct Maze {
    //misnomer for end
    size: isize,
    bytes: Vec<(isize, isize)>,
}

impl Maze {
    fn new(data: &str, size: isize) -> Self {
        let bytes = data
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(",").unwrap();

                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        Self { size, bytes }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other.cmp(self) is min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(maze: &Maze, fallen: usize) -> usize {
    let obstacles: HashSet<&(isize, isize)> = HashSet::from_iter(maze.bytes.iter().take(fallen));

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    let mut steps = 0;
    let end = (maze.size, maze.size);
    let bounds = 0..=maze.size;

    while let Some(state) = heap.pop() {
        if visited.contains(&state.position) {
            continue;
        }

        if state.position == end {
            // done
            steps = state.cost;
            break;
        }

        // println!("{:?}", state);
        // get next states
        for dir in DIRS {
            // needs commas
            let next = tup!(state.position, +, dir);

            // check bounds & obsacles
            if !bounds.contains(&next.0) || !bounds.contains(&next.1) || obstacles.contains(&next) {
                continue;
            }

            heap.push(State {
                cost: state.cost + 1, // one more step
                position: next,
            });
        }

        visited.insert(state.position);
    }

    steps
}

fn part_two(maze: &Maze, start: usize) -> String {
    // repeat part one until we get '0'
    for i in start.. {
        if part_one(maze, i) == 0 {
            let pos = maze.bytes[i - 1];
            return format!("{},{}", pos.0, pos.1);
        }
    }
    "".to_string()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let maze = Maze::new(&data, 70);

    if one {
        let now = Instant::now();
        let ans = part_one(&maze, 1024);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&maze, 1025);
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
        let ans = part_one(&Maze::new(EXAMPLE, 6), 12);

        assert_eq!(ans, 22);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Maze::new(EXAMPLE, 6), 13);

        assert_eq!(ans, "6,1");
    }
}
