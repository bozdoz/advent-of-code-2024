use std::{ collections::{ BinaryHeap, HashSet }, fs, time::Instant, usize };
use lib::{ get_part, DIRS };

struct Maze {
    start: isize,
    end: isize,
    // try something different
    cells: Vec<u8>,
    width: usize,
}

const SPACE: u8 = b'.';
const END: u8 = b'E';

impl Maze {
    fn new(data: &str) -> Self {
        let mut start: isize = 0;
        let mut end: isize = 0;
        let mut cells = vec![];
        let mut width = 0;

        for (r, row) in data.lines().enumerate() {
            if width == 0 {
                width = row.len();
            }
            for (c, cell) in row.chars().enumerate() {
                cells.push(cell as u8);

                match cell {
                    'S' => {
                        start = (r * width + c) as isize;
                    }
                    'E' => {
                        end = (r * width + c) as isize;
                    }
                    _ => {}
                }
            }
        }

        Self { start, end, cells, width }
    }

    fn move_from_cell(&self, dir: &(isize, isize), cell: isize) -> Option<isize> {
        let next = if dir.0 == 0 { cell + dir.1 } else { cell + dir.0 * (self.width as isize) };

        match self.cells[next as usize] {
            SPACE | END => { Some(next) }
            _ => { None }
        }
    }
}

fn _print_maze(maze: &Maze) {
    // use chunks to simulate 2d grid
    for c in maze.cells.chunks(maze.width) {
        println!(
            "{:?}",
            c
                .iter()
                .map(|&x|
                    char
                        ::from_u32(x as u32)
                        .unwrap()
                        .to_string()
                )
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

fn _print_x_in_maze(maze: &Maze, cell: isize) {
    // converts cell index to (r,c)
    let find = ((cell as usize) / maze.width, (cell as usize) % maze.width);
    for (r, row) in maze.cells.chunks(maze.width).enumerate() {
        let mut out: Vec<String> = vec![];
        for (c, val) in row.iter().enumerate() {
            if (r, c) == find {
                out.push("X".to_string());
            } else {
                out.push(
                    char
                        ::from_u32(*val as u32)
                        .unwrap()
                        .to_string()
                );
            }
        }
        println!("{}", out.join(""));
    }
}

#[derive(PartialEq, Eq)]
struct State {
    // pos, direction
    current: (isize, usize),
    cost: usize,
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

fn part_one(maze: &Maze) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        current: (maze.start, 1),
        cost: 0,
    });

    let mut best = usize::MAX;

    while let Some(state) = heap.pop() {
        let (cell, dir) = state.current;

        // part 1 we exit early if cell was visited
        if visited.contains(&cell) {
            continue;
        }

        // check if done
        if cell == maze.end {
            best = state.cost.min(best);
            continue;
        }

        // check if already worse than best
        if state.cost > best {
            continue;
        }

        // get next states
        for (i, d) in DIRS.iter().enumerate() {
            // check only 3 directions
            let ignore_dir = (dir + 2) % 4;

            if i == ignore_dir {
                continue;
            }

            // check if can move in direction
            if let Some(next) = maze.move_from_cell(d, state.current.0) {
                // remove if visited
                if visited.contains(&next) {
                    continue;
                }

                // check if direction change
                let dir_cost = if i == dir { 0 } else { 1000 };
                // update cost according to step + possible direction change

                heap.push(State {
                    current: (next, i),
                    cost: state.cost + 1 + dir_cost,
                });
            }
        }

        // cell has been fully visited
        visited.insert(cell);
    }

    // what a signature
    best
}

#[derive(PartialEq, Eq)]
struct VisitedState {
    current: (isize, usize),
    cost: usize,
    visited: Vec<isize>,
}

impl Ord for VisitedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for VisitedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_two(maze: &Maze) -> usize {
    // let mut heap = vec![];
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut winners = vec![];
    let mut best = usize::MAX;

    heap.push(VisitedState {
        current: (maze.start, 1),
        cost: 0,
        visited: vec![maze.start],
    });

    while let Some(state) = heap.pop() {
        let (cell, dir) = state.current;

        // part 2 can prune if we've found better (can't omit visited cells)
        if state.cost > best {
            // drop anything if we already have better
            continue;
        }

        // check if done
        if cell == maze.end {
            if state.cost < best {
                // new winners
                best = state.cost;
                winners = vec![state];
            } else if state.cost == best {
                winners.push(state);
            }
            continue;
        }

        // get next states
        for (i, d) in DIRS.iter().enumerate() {
            // check only 3 directions
            let ignore_dir = (dir + 2) % 4;

            if i == ignore_dir {
                continue;
            }

            // check if can move in direction
            if let Some(next) = maze.move_from_cell(d, state.current.0) {
                // remove if visited
                if visited.contains(&(next, i)) {
                    continue;
                }

                // check if direction change
                let dir_cost = if i == dir { 0 } else { 1000 };
                // update cost according to step + possible direction change

                let mut next_visited = state.visited.clone();

                next_visited.push(next);

                heap.push(VisitedState {
                    current: (next, i),
                    cost: state.cost + 1 + dir_cost,
                    visited: next_visited,
                });
            }
        }

        visited.insert(state.current);
    }

    let mut common = HashSet::new();

    for winner in winners {
        for cell in winner.visited {
            common.insert(cell);
        }
    }

    common.len()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let maze = Maze::new(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&maze);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&maze);
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
        let ans = part_one(&Maze::new(EXAMPLE));

        assert_eq!(ans, 7036);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Maze::new(EXAMPLE));

        assert_eq!(ans, 45);
    }
}
