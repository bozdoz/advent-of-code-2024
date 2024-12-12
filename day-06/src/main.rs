use std::{ collections::HashSet, fs, time::Instant };
use lib::get_part;

// (r, c) differences, clockwise
const DIRS: [(isize, isize); 4] = [
    (-1, 0), // top
    (0, 1), // right
    (1, 0), // bottom
    (0, -1), // left
];

struct Grid {
    cells: Vec<Vec<char>>,
    height: isize,
    width: isize,
    start: (isize, isize),
}

impl Grid {
    fn new(data: &str) -> Self {
        let cells: Vec<_> = data
            .lines()
            .map(|l| { l.chars().collect::<Vec<_>>() })
            .collect();

        let height = cells.len() as isize;
        let width = cells[0].len() as isize;

        let start = cells
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                for (c, &cell) in row.iter().enumerate() {
                    if cell == '^' {
                        return Some((r as isize, c as isize));
                    }
                }
                None
            })
            .expect("find_map to do its job");

        Self { cells, height, width, start }
    }

    fn get(&self, pos: (isize, isize)) -> Option<char> {
        if pos.0 == -1 || pos.1 == -1 || pos.0 >= self.height || pos.1 >= self.width {
            return None;
        }

        Some(self.cells[pos.0 as usize][pos.1 as usize])
    }

    fn try_obstacle(&self, obstacle: (isize, isize), start: (isize, isize), dir: usize) -> bool {
        let mut cur = start;
        let mut visited = vec![vec![0; self.width as usize]; self.height as usize];

        visited[start.0 as usize][start.1 as usize] |= (2usize).pow(dir as u32);

        // traverse and update visited to detect loops
        let mut d = dir;
        loop {
            let bin = (2usize).pow((d % 4) as u32);
            let dir = DIRS[d % 4];

            loop {
                let next = (cur.0 + dir.0, cur.1 + dir.1);

                if next == obstacle {
                    break;
                }

                if let Some(c) = self.get(next) {
                    if c == '#' {
                        // hit a wall
                        break;
                    }

                    // first time using mutable reference?
                    let cell = &mut visited[next.0 as usize][next.1 as usize];

                    // check if the cell has the direction we're currently moving in
                    if (*cell & bin) == bin {
                        return true;
                    }

                    // add direction to the visited list
                    *cell |= bin;
                } else {
                    // exited the map!!
                    return false;
                }

                cur = next;
            }

            // change direction
            d += 1;
        }
    }
}

fn part_one(grid: &mut Grid) -> usize {
    // find the guard `^` and walk clockwise
    let mut cur = grid.start;
    let travelled = 'X';

    // not sure if there's a better way to repeatedly iterate the DIRS
    loop {
        for dir in DIRS {
            loop {
                let next = (cur.0 + dir.0, cur.1 + dir.1);

                if let Some(c) = grid.get(next) {
                    if c == '#' {
                        // hit a wall
                        break;
                    }
                    grid.cells[next.0 as usize][next.1 as usize] = travelled;
                } else {
                    // exited the map!!
                    // for row in &grid.cells {
                    //     println!("{:?}", row);
                    // }
                    return grid.cells.iter().fold(0, |acc, row| {
                        acc +
                            row
                                .iter()
                                .filter(|&&cell| cell == travelled)
                                .count()
                    });
                }

                cur = next;
            }
        }
    }
}

fn part_two(grid: &Grid) -> usize {
    // never put an obstacle at the start
    let mut cur = grid.start;
    // could probably just be a 2d vec...
    let mut visited = HashSet::new();

    let mut count = 0;
    visited.insert(cur);

    loop {
        for (i, dir) in DIRS.iter().enumerate() {
            loop {
                let next = (cur.0 + dir.0, cur.1 + dir.1);

                if let Some(c) = grid.get(next) {
                    if c == '#' {
                        // hit a wall
                        break;
                        // already assessed as an obstacle
                    } else if !visited.contains(&next) {
                        // try to place an obstacle
                        if grid.try_obstacle(next, cur, i) {
                            count += 1;
                        }

                        visited.insert(next);
                    }
                } else {
                    // exited the map!!
                    return count;
                }

                cur = next;
            }
        }
    }
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let mut grid = Grid::new(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&mut grid);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&grid);
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
        let mut grid = Grid::new(EXAMPLE);
        let ans = part_one(&mut grid);

        assert_eq!(ans, 41);
    }

    #[test]
    fn test_loop() {
        let grid = Grid::new(EXAMPLE);
        let ans = grid.try_obstacle((6, 3), grid.start, 0);

        assert_eq!(ans, true);

        let ans = grid.try_obstacle((5, 4), grid.start, 0);

        assert_eq!(ans, false);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 6);
    }
}
