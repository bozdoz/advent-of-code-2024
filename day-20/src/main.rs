use std::{ collections::HashMap, fs, time::Instant, vec };
use lib::{ get_part, DIRS, tup };

struct Race {
    start: isize,
    end: isize,
    // try something different
    cells: Vec<u8>,
    width: usize,
}

const SPACE: u8 = b'.';
const END: u8 = b'E';

impl Race {
    // copied from day 16
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

    // had this so wrong with something like (-3, 15) which should have been
    // off the map, if not for my previous logic
    fn move_from_cell(&self, dir: &(isize, isize), cell: isize) -> Option<isize> {
        // can't figure out the math, so converting to point
        let p = (cell / (self.width as isize), cell % (self.width as isize));

        let next = tup!(p + dir);

        if next.0 < 1 || next.1 < 1 || next.1 >= (self.width as isize) {
            return None;
        }

        // point back to cell index
        let next = next.0 * (self.width as isize) + next.1;

        // usng .get because *2 could overflow in any direction
        match self.cells.get(next as usize) {
            Some(&SPACE) | Some(&END) => { Some(next) }
            _ => { None }
        }
    }

    // this functions is a bit faster: 800us vs 1.2ms
    fn _run(&self) -> HashMap<isize, isize> {
        // do the race, keep track of every point you could cheat
        // keep track of every cell's time
        let mut times = vec![-1; self.cells.len()];
        // (to -> from)
        let mut cheats = vec![];
        let mut current = self.start;

        let double = (2, 2);
        let mut steps = 0;

        while current != self.end {
            // visited
            times[current as usize] = steps;
            steps += 1;

            let mut next = current;

            // move through the single-path track
            for dir in DIRS {
                let check = self.move_from_cell(&dir, current);

                if let Some(val) = check {
                    // no cheat in this direction
                    // check if visited
                    // otherwise ignore
                    if times[val as usize] == -1 {
                        next = val;
                    }
                } else {
                    // try to cheat (through 1 wall)
                    if let Some(cheat) = self.move_from_cell(&tup!(dir * double), current) {
                        // check if going backwards
                        if times[cheat as usize] == -1 {
                            cheats.push((cheat, current));
                        }
                    }
                }
            }

            // update next loop
            current = next;
        }

        times[self.end as usize] = steps;

        let mut savings = HashMap::new();

        for cheat in cheats {
            let to = times[cheat.0 as usize];
            let from = times[cheat.1 as usize];

            // minus 2 for the steps through the wall
            let diff = to - from - 2;

            savings
                .entry(diff)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }

        // all cheats should have costs and point to other cells with costs
        savings
    }

    fn _print(&self, cell: isize, marker: &str) {
        // converts cell index to (r,c)
        let find = ((cell as usize) / self.width, (cell as usize) % self.width);
        for (r, row) in self.cells.chunks(self.width).enumerate() {
            let mut out: Vec<String> = vec![];
            for (c, val) in row.iter().enumerate() {
                if (r, c) == find {
                    out.push(marker.to_string());
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

    fn run_a_second_time(&self, cheat_dist: isize) -> HashMap<isize, isize> {
        // do the race, keep track of every point you could cheat
        // keep track of every cell's time
        let mut times = vec![-1; self.cells.len()];
        // (to -> from -> steps)
        let mut cheats = vec![];

        let mut current = self.start;
        let mut steps = 0;

        // get all direction diffs
        let mut manhattans = vec![];

        // TODO: had these wrong, didn't abs them in nested checks
        for i in -cheat_dist..=cheat_dist {
            for j in -cheat_dist..=cheat_dist {
                if i.abs() + j.abs() <= cheat_dist {
                    // ignore adjacent walls with <2
                    if !(i.abs() < 2 && j.abs() < 2) {
                        manhattans.push((i, j));
                    }
                }
            }
        }

        while current != self.end {
            // visited check
            times[current as usize] = steps;
            steps += 1;

            // move through the single-path track
            let next = DIRS.iter()
                .find_map(|dir| {
                    self.move_from_cell(&dir, current).filter(|&val| {
                        // no cheat in this direction
                        // check if unvisited
                        // otherwise ignore
                        times[val as usize] == -1
                    })
                })
                .unwrap();

            // try to cheat (through X number of walls)
            for dir in manhattans.iter() {
                if let Some(cheat) = self.move_from_cell(&dir, current) {
                    // check if visited!
                    if times[cheat as usize] == -1 {
                        cheats.push((cheat, current, dir.0.abs() + dir.1.abs()));
                    }
                }
            }

            // update next loop
            current = next;
        }

        times[self.end as usize] = steps;

        let mut savings = HashMap::new();

        for cheat in cheats {
            let cheat_time = times[cheat.0 as usize];
            let race_time = times[cheat.1 as usize];

            // cheat time - non-cheat time - steps through the wall
            let diff = cheat_time - race_time - cheat.2;

            savings
                .entry(diff)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }

        savings
    }
}

fn part_one(race: &Race, at_least: usize) -> usize {
    race.run_a_second_time(2)
        .iter()
        .filter_map(|(&seconds, &count)| {
            if (seconds as usize) >= at_least {
                return Some(count as usize);
            }
            None
        })
        .sum()
}

fn part_two(race: &Race, at_least: usize) -> usize {
    race.run_a_second_time(20)
        .iter()
        .filter_map(|(&seconds, &count)| {
            if (seconds as usize) >= at_least {
                return Some(count as usize);
            }
            None
        })
        .sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let race = Race::new(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&race, 100);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&race, 100);
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
        let ans = part_one(&Race::new(EXAMPLE), 12);

        assert_eq!(ans, 8);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Race::new(EXAMPLE), 50);

        assert_eq!(ans, 285);
    }
}
