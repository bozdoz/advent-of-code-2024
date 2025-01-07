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

fn part_one(maze: &Maze, fallen: usize) -> usize {
    let obstacles: HashSet<&(isize, isize)> = HashSet::from_iter(maze.bytes.iter().take(fallen));

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    // converting from struct State { cost, position } to (cost, (x, y)) was 2x faster
    heap.push((0, (0, 0)));

    let mut steps = 0;
    let end = (maze.size, maze.size);
    let bounds = 0..=maze.size;

    while let Some((cost, position)) = heap.pop() {
        if visited.contains(&position) {
            continue;
        }

        if position == end {
            // done
            steps = cost;
            break;
        }

        // println!("{:?}", state);
        // get next states
        for dir in DIRS {
            // needs commas
            let next = tup!(position, +, dir);

            // check bounds & obsacles
            if !bounds.contains(&next.0) || !bounds.contains(&next.1) || obstacles.contains(&next) {
                continue;
            }

            heap.push((cost + 1, next));
        }

        visited.insert(position);
    }

    steps
}

fn part_two(maze: &Maze, start: usize) -> String {
    // repeat part one until we get '0'
    // for i in start.. {
    //     if part_one(maze, i) == 0 {
    //         let pos = maze.bytes[i - 1];

    //         println!("{i}");

    //         return format!("{},{}", pos.0, pos.1);
    //     }
    // }
    // "".to_string()

    let mut obstacles: HashSet<&(isize, isize)> = HashSet::from_iter(maze.bytes.iter().take(start));

    let end = (maze.size, maze.size);
    let bounds = 0..=maze.size;

    // moving this outside and using .clear() saved 100ms
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    // each iteration adds a new obstacle
    // we should be able to check if a new obstacle hit our path
    'throwingsnow: for i in start..maze.bytes.len() {
        let obs = &maze.bytes[i];

        obstacles.insert(obs);

        visited.clear();
        heap.clear();

        // maybe should start somewhere else?
        heap.push((0, (0, 0)));

        while let Some((cost, position)) = heap.pop() {
            if visited.contains(&position) {
                continue;
            }

            if position == end {
                continue 'throwingsnow;
            }

            // println!("{:?}", state);
            // get next states
            for dir in DIRS {
                // needs commas
                let next = tup!(position + dir);

                // check bounds & obsacles
                if
                    !(
                        bounds.contains(&next.0) &&
                        bounds.contains(&next.1) &&
                        !obstacles.contains(&next)
                    )
                {
                    continue;
                }

                heap.push((cost + 1, next));
            }

            visited.insert(position);
        }

        println!("{i}");

        return format!("{},{}", obs.0, obs.1);
    }

    return "FAIL".to_string();
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
