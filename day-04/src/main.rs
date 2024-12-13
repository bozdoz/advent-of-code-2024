#![allow(unused)]

use std::{ time::Instant, fs };
use lib::{ get_part, Grid };

// (r, c) differences, clockwise
const DIRECTIS: [(isize, isize); 8] = [
    (-1, 0), // top
    (-1, 1), // tr
    (0, 1), // right
    (1, 1), // br
    (1, 0), // bottom
    (1, -1), // bl
    (0, -1), // left
    (-1, -1), // tl
];

// this needs &'static or an explicit number for length of array:
// const SEARCH: [char; 3] = ['M', 'A', 'S'];
const SEARCH: &'static [char] = &['M', 'A', 'S'];

fn part_one(grid: &Grid) -> usize {
    let mut count = 0;

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'X' {
                let r = r as isize;
                let c = c as isize;

                for dir in DIRECTIS {
                    let mut nextr = r;
                    let mut nextc = c;

                    for &ch in SEARCH {
                        nextr += dir.0;
                        nextc += dir.1;

                        // first time the formatter has done this to me
                        if
                            nextr == -1 ||
                            nextc == -1 ||
                            nextr >= grid.height ||
                            nextc >= grid.width
                        {
                            break;
                        }

                        if grid.cells[nextr as usize][nextc as usize] == ch {
                            if ch == 'S' {
                                // we did it!
                                count += 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    count
}

// looking for an X shape
const DIAGONALS: [(isize, isize); 4] = [
    (-1, -1), // tl
    (1, 1), // br
    (-1, 1), // tr
    (1, -1), // bl
];

fn part_two(grid: &Grid) -> usize {
    let mut count = 0;

    // The SAM detector
    for (r, row) in grid.cells.iter().enumerate() {
        'nextcell: for (c, &cell) in row.iter().enumerate() {
            if cell == 'A' {
                let r = r as isize;
                let c = c as isize;

                for dirs in DIAGONALS.chunks(2) {
                    let mut acceptable = vec!['S', 'M'];

                    for dir in dirs {
                        let nextr = r + dir.0;
                        let nextc = c + dir.1;

                        if
                            nextr == -1 ||
                            nextc == -1 ||
                            nextr >= grid.height ||
                            nextc >= grid.width
                        {
                            continue 'nextcell;
                        }

                        let ch = &grid.cells[nextr as usize][nextc as usize];

                        if acceptable.contains(ch) {
                            // remove from acceptable and search next diagonal
                            acceptable.retain(|x| x != ch);
                        } else {
                            continue 'nextcell;
                        }
                    }
                }
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let grid = Grid::new_with_chars(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&grid);
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
        let grid = Grid::new_with_chars(EXAMPLE);
        let ans = part_one(&grid);

        assert_eq!(ans, 18);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new_with_chars(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 9);
    }
}
