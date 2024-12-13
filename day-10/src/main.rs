use std::{ collections::HashSet, fs, time::Instant };
use lib::{ get_part, Grid, DIRS };

fn parts_one_and_two<T>(
    grid: &Grid<u32>,
    update_complete: impl Fn(&mut HashSet<T>, Vec<(usize, usize)>) -> ()
) -> usize {
    // find the zeroes
    let mut states = vec![];
    let mut complete = HashSet::new();

    for (r, row) in grid.cells.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                states.push(vec![(r, c)]);
            }
        }
    }

    while let Some(state) = states.pop() {
        // finished
        if state.len() == 10 {
            update_complete(&mut complete, state);
            continue;
        }

        // get next states
        // next number is the length of state
        let next = state.len();
        let pos = state[next - 1];

        // search all directions for next
        for dir in DIRS {
            let n = ((pos.0 as isize) + dir.0, (pos.1 as isize) + dir.1);

            if let Some(&v) = grid.get(n) {
                if v == (next as u32) {
                    let mut clone = state.clone();
                    clone.push((n.0 as usize, n.1 as usize));
                    states.push(clone);
                }
            }
        }
    }

    complete.len()
}

fn part_one(grid: &Grid<u32>) -> usize {
    parts_one_and_two(grid, |complete, state| {
        // 0's and 9's need to be unique
        complete.insert((state[0], state[9]));
    })
}

fn part_two(grid: &Grid<u32>) -> usize {
    parts_one_and_two(grid, |complete, state| {
        // each node needs to be unique
        complete.insert(state);
    })
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    let grid = Grid::new_with_u32(&data);

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
    fn test_simple() {
        let grid = Grid::new_with_u32("5550555
5551555
5552555
6543456
7555557
8555558
9555559");
        let ans = part_one(&grid);

        assert_eq!(ans, 2);

        let grid = Grid::new_with_u32("5590559
5551598
5552557
6543456
7655987
8765555
9875555");
        let ans = part_one(&grid);

        assert_eq!(ans, 4);
    }

    #[test]
    fn test_part_one() {
        let grid = Grid::new_with_u32(EXAMPLE);
        let ans = part_one(&grid);

        assert_eq!(ans, 36);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new_with_u32(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 81);
    }
}
