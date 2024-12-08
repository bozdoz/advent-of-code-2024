use std::{ collections::{ HashMap, HashSet }, fs, time::Instant };
use lib::{ get_part, Grid };

fn get_antennas(grid: &Grid) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas = HashMap::new();

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas
                    .entry(cell)
                    .and_modify(|e: &mut Vec<(isize, isize)>| {
                        e.push((r as isize, c as isize));
                    })
                    .or_insert(vec![(r as isize, c as isize)]);
            }
        }
    }

    antennas
}

fn part_one(grid: &Grid) -> usize {
    let antennas = get_antennas(grid);
    let mut antinodes = HashSet::new();

    // every antenna creates antinodes with every other antenna
    for a in antennas.values() {
        // println!("{:?}", a);
        for i in 0..a.len() {
            let first = a[i];
            for j in i + 1..a.len() {
                let second = a[j];
                // get the diff and mirror it
                let diff = (first.0 - second.0, first.1 - second.1);

                // println!("{:?} {:?} diff: {:?}", first, second, diff);

                // add to first, sub from second?
                for (k, b) in [first, second].iter().enumerate() {
                    // used to swap from addition to subtraction
                    let m = [1, -1][k];
                    let antinode = (b.0 + diff.0 * m, b.1 + diff.1 * m);

                    if
                        antinode.0 >= 0 &&
                        antinode.0 < grid.height &&
                        antinode.1 >= 0 &&
                        antinode.1 < grid.width
                    {
                        // in range
                        // println!("-- antinode: {:?}", antinode);
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part_two(grid: &Grid) -> usize {
    let antennas = get_antennas(grid);
    let mut antinodes = HashSet::new();

    // every antenna creates antinodes with every other antenna
    for a in antennas.values() {
        // println!("{:?}", a);
        for i in 0..a.len() {
            let first = a[i];
            for j in i + 1..a.len() {
                let second = a[j];
                // all antennas are antinodes, so add them too
                antinodes.insert(first);
                antinodes.insert(second);

                // get the diff and mirror it
                let diff = (first.0 - second.0, first.1 - second.1);

                // println!("{:?} {:?} diff: {:?}", first, second, diff);

                // add to first, sub from second?
                for (k, b) in [first, second].iter().enumerate() {
                    // used to swap from addition to subtraction
                    let m = [1, -1][k];

                    let mut cur = *b;

                    loop {
                        cur = (cur.0 + diff.0 * m, cur.1 + diff.1 * m);

                        if cur.0 >= 0 && cur.0 < grid.height && cur.1 >= 0 && cur.1 < grid.width {
                            // in range
                            // println!("-- cur: {:?}", cur);
                            antinodes.insert(cur);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let grid = Grid::new(&data);

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
        let grid = Grid::new(EXAMPLE);
        let ans = part_one(&grid);

        assert_eq!(ans, 14);
    }

    #[test]
    fn test_simple_part_two() {
        let grid = Grid::new(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
        );
        let ans = part_two(&grid);

        assert_eq!(ans, 9);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 34);
    }
}
