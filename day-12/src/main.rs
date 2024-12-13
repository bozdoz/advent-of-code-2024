#![allow(unused)]

use std::{ collections::HashSet, fs, hash::Hash, time::Instant };
use lib::{ get_part, Grid, DIRS };

// needs external recursive function, as I can't have a recursive closure
fn flood(
    grid: &Grid,
    cell: (isize, isize),
    check: &char,
    visited: &mut HashSet<(isize, isize)>
) -> (usize, usize) {
    if visited.contains(&cell) {
        return (0, 0);
    }

    if let Some(item) = grid.get(cell) {
        if item == check {
            visited.insert(cell);
        }

        // update area
        let mut area = 1;
        let mut next_cells = vec![];
        // perimeter is any neighbours that aren't identical
        let mut perimeter = DIRS.iter().fold(0, |acc, dir| {
            let next = (cell.0 + dir.0, cell.1 + dir.1);

            match grid.get(next) {
                Some(other) if other == item => {
                    // side effect adding next cells
                    next_cells.push(next);
                    acc
                }
                _ => {
                    // doesn't match
                    acc + 1
                }
            }
        });

        for cell in next_cells {
            let neighbour = flood(grid, cell, check, visited);

            area += neighbour.0;
            perimeter += neighbour.1;
        }

        return (area, perimeter);
    }

    (0, 0)
}

fn part_one(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    // tried to move this to lib, but can't figure out how to implement this as iterator
    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let coords = (r as isize, c as isize);
            if visited.contains(&coords) {
                continue;
            }
            let val = flood(grid, coords, cell, &mut visited);

            // println!("{cell}: {} {}", val.0, val.1);

            total_price += val.0 * val.1;
        }
    }

    total_price
}

// little different from flood, but basically the same
// a lot of these params could probably be in a struct
fn collect_cells<'a>(
    grid: &Grid,
    cell: (isize, isize),
    check: &char,
    visited: &HashSet<(isize, isize)>,
    region: &'a mut HashSet<(isize, isize)>
) -> &'a mut HashSet<(isize, isize)> {
    if region.contains(&cell) || visited.contains(&cell) {
        return region;
    }

    if let Some(item) = grid.get(cell) {
        if item == check {
            region.insert(cell);
        }

        // check all dirs for same values
        for dir in DIRS {
            let next = (cell.0 + dir.0, cell.1 + dir.1);

            if let Some(other) = grid.get(next) {
                if other == check {
                    collect_cells(grid, next, check, visited, region);
                }
            }
        }
    }

    region
}

// this idea came from: https://www.reddit.com/r/adventofcode/comments/1hcxmpp/2024_day_12_part_2_visualisation_of_my_first/
fn scan_perimeters(region: &HashSet<(isize, isize)>) -> usize {
    let mut sides = 0;
    for &missing_dir in DIRS.iter() {
        let mut found: HashSet<(isize, isize)> = HashSet::new();
        for cell in region {
            if found.contains(cell) {
                continue;
            }

            // check if missing neighbor at dir, then get adjacent's with the same
            let check_dir = (cell.0 + missing_dir.0, cell.1 + missing_dir.1);

            if region.contains(&check_dir) {
                continue;
            }

            // this is a side, and we need to find adjacent's on the same side
            found.insert(*cell);
            sides += 1;

            // if top, go left, then right...
            let left = (missing_dir.1, missing_dir.0);
            let right = (-missing_dir.1, -missing_dir.0);

            for lr_dir in [left, right] {
                let mut cur = *cell;
                loop {
                    cur.0 += lr_dir.0;
                    cur.1 += lr_dir.1;
                    let check = &(cur.0 + missing_dir.0, cur.1 + missing_dir.1);

                    if region.contains(&cur) && !region.contains(check) {
                        // found adjacent
                        found.insert(cur);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    sides
}

fn part_two(grid: &Grid) -> usize {
    // get areas, then scan for perimeters in each
    let mut visited = HashSet::new();
    let mut regions = vec![];
    let mut total_price = 0;

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let coords = (r as isize, c as isize);

            let mut region = HashSet::new();

            collect_cells(grid, coords, cell, &visited, &mut region);

            // this is all crazy
            for &v in region.iter() {
                visited.insert(v);
            }
            regions.push(region);
        }
    }

    for region in regions.iter() {
        let perimeter = scan_perimeters(region);
        let area = region.len();

        total_price += area * perimeter;
    }

    total_price
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let grid = Grid::new_with_chars(&data);

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
        let ans = part_one(&Grid::new_with_chars(EXAMPLE));

        assert_eq!(ans, 1930);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Grid::new_with_chars(EXAMPLE));

        assert_eq!(ans, 1206);
    }
}
