use std::{ collections::{ HashMap }, fmt::Display, fs, time::Instant };
use lib::{ get_part, tup };

#[derive(Debug, Clone, Copy, PartialEq)]
enum Thing {
    Box,
    Wall,
    LBox,
    RBox,
}

struct Map {
    robot: (isize, isize),
    grid: HashMap<(isize, isize), Thing>,
    width: usize,
    height: usize,
    cell_width: usize,
}

impl Map {
    fn new(data: &str) -> Self {
        let mut robot: (isize, isize) = (0, 0);
        let mut grid: HashMap<(isize, isize), Thing> = HashMap::new();
        let mut height = 0;
        let mut width = 0;

        for (r, row) in data.lines().enumerate() {
            height += 1;
            if width == 0 {
                width = row.len();
            }
            for (c, cell) in row.chars().enumerate() {
                let pos = (r as isize, c as isize);
                match cell {
                    '@' => {
                        robot = pos;
                    }
                    '#' => {
                        grid.insert(pos, Thing::Wall);
                    }
                    'O' => {
                        grid.insert(pos, Thing::Box);
                    }
                    _ => {}
                }
            }
        }

        Self {
            robot,
            grid,
            width,
            height,
            cell_width: 1,
        }
    }

    fn double_the_width(&mut self) {
        // width of 1 -> 2
        // everything duplicates except robot, which just moves
        self.width *= 2;
        self.cell_width = 2;
        self.robot.1 *= 2;

        let mut next_grid = HashMap::new();

        for (&(r, c), &thing) in self.grid.iter() {
            match thing {
                Thing::Wall => {
                    next_grid.insert((r, c * 2), thing);
                    next_grid.insert((r, c * 2 + 1), thing);
                }
                Thing::Box => {
                    next_grid.insert((r, c * 2), Thing::LBox);
                    next_grid.insert((r, c * 2 + 1), Thing::RBox);
                }
                _ => {}
            }
        }

        self.grid = next_grid;
    }

    fn move_robot(&mut self, dir: (isize, isize)) {
        let mut pos = self.robot;

        // look in the direction for ANY empty space before a wall
        let mut found_boxes = false;
        loop {
            pos.0 += dir.0;
            pos.1 += dir.1;

            match self.grid.get(&pos) {
                None => {
                    // can move
                    break;
                }
                Some(Thing::Box) | Some(Thing::LBox) | Some(Thing::RBox) => {
                    // might be able to move
                    found_boxes = true;
                    continue;
                }
                Some(Thing::Wall) => {
                    // can't move
                    return;
                }
            }
        }

        self.robot.0 += dir.0;
        self.robot.1 += dir.1;

        if !found_boxes {
            return;
        }

        if self.cell_width == 2 {
            let mut cur = pos;

            loop {
                let next = tup!(cur - dir);

                // let's move each thing as it is
                let thing = self.grid.remove(&next).expect("Next to be box");

                self.grid.insert(cur, thing);

                if next == self.robot {
                    // robot has pushed everything
                    break;
                }

                cur = next;
            }
        } else {
            // move first obstacle to position
            // println!("moving {:?} -> {:?}", self.robot, pos);
            self.grid.remove(&self.robot);
            self.grid.insert(pos, Thing::Box);
        }
    }

    // dir is ONLY up or down; left or right can just use the `move_robot`
    fn move_robot_2x_big(&mut self, dir: (isize, isize)) {
        let pos = self.robot;

        // check positions
        let mut check_positions = vec![tup!(pos + dir)];

        let mut boxes = vec![];
        while let Some(peek) = check_positions.pop() {
            match self.grid.get(&peek) {
                Some(Thing::LBox) => {
                    boxes.push(peek);
                    // check above/below and one to the right
                    let next = tup!(peek + dir);
                    check_positions.push(next);
                    check_positions.push((next.0, next.1 + 1));
                }
                Some(Thing::RBox) => {
                    // push left box
                    boxes.push((peek.0, peek.1 - 1));
                    // check above/below and one to the left
                    let next = tup!(peek + dir);
                    check_positions.push(next);
                    check_positions.push((next.0, next.1 - 1));
                }
                Some(Thing::Wall) => {
                    // exit, can't move
                    return;
                }
                _ => {}
            }
        }

        // let's move the robot and each box
        self.robot.0 += dir.0;
        self.robot.1 += dir.1;

        // remove all boxes
        for b in boxes.iter() {
            // remove left
            self.grid.remove(b);
            // remove right
            self.grid.remove(&(b.0, b.1 + 1));
        }

        // add all boxes one step in the direction
        for b in boxes.iter() {
            let n = tup!(b + dir);
            self.grid.insert(n, Thing::LBox);
            self.grid.insert((n.0, n.1 + 1), Thing::RBox);
        }
    }

    fn get_gps_coords(self) -> impl Iterator<Item = isize> {
        // part 1 counts box; 2 counts LBox
        let find = if self.cell_width == 1 { Thing::Box } else { Thing::LBox };

        // first `move`?
        self.grid.into_iter().filter_map(move |((r, c), thing)| {
            if thing == find {
                return Some(r * 100 + c);
            }
            None
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: Vec<String> = vec![];
        for r in 0..self.height {
            let mut row: Vec<&str> = vec![];
            for c in 0..self.width {
                if self.robot == (r as isize, c as isize) {
                    row.push("@");
                } else {
                    row.push(match self.grid.get(&(r as isize, c as isize)) {
                        Some(Thing::Wall) => "#",
                        Some(Thing::Box) => "O",
                        Some(Thing::LBox) => "[",
                        Some(Thing::RBox) => "]",
                        _ => ".",
                    });
                }
            }
            out.push(row.join(""));
        }
        f.write_str(out.join("\n").as_str())
    }
}

fn parse_data(data: &str) -> (Map, &str) {
    let (map, moves) = data.split_once("\n\n").expect("Space");
    let map = Map::new(map);

    (map, moves)
}

fn part_one(map: &Map, moves: &str) -> isize {
    let mut clone = Map {
        // manual cloning because HashMap doesn't implement Copy
        grid: HashMap::from_iter(map.grid.iter().map(|(&k, &v)| { (k, v) })),
        ..*map
    };

    for m in moves.chars() {
        match m {
            '<' => {
                clone.move_robot((0, -1));
            }
            '^' => {
                clone.move_robot((-1, 0));
            }
            '>' => {
                clone.move_robot((0, 1));
            }
            'v' => {
                clone.move_robot((1, 0));
            }
            _ => {}
        }
        // println!("move {m}");
        // println!("{clone}");
    }

    clone.get_gps_coords().sum()
}

fn part_two(map: &Map, moves: &str) -> isize {
    let mut clone = Map {
        // manual cloning because HashMap doesn't implement Copy
        grid: HashMap::from_iter(map.grid.iter().map(|(&k, &v)| { (k, v) })),
        ..*map
    };

    clone.double_the_width();

    for m in moves.chars() {
        match m {
            '<' => {
                clone.move_robot((0, -1));
            }
            '^' => {
                clone.move_robot_2x_big((-1, 0));
            }
            '>' => {
                clone.move_robot((0, 1));
            }
            'v' => {
                clone.move_robot_2x_big((1, 0));
            }
            _ => {}
        }
        // println!("move {m}");
        // println!("{clone}");
    }

    clone.get_gps_coords().sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let (map, moves) = parse_data(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&map, moves);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&map, moves);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_smaller() {
        let (map, moves) = parse_data(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
        );
        let ans = part_one(&map, moves);

        assert_eq!(ans, 2028);
    }

    #[test]
    fn test_gps() {
        let map = Map::new("#######
#...O..
#......
");
        assert_eq!(map.get_gps_coords().sum::<isize>(), 104);
    }

    #[test]
    fn test_part_one() {
        let (map, moves) = parse_data(EXAMPLE);
        let ans = part_one(&map, moves);

        assert_eq!(ans, 10092);
    }

    #[test]
    fn test_wider() {
        let (map, moves) = parse_data(
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
        );

        part_two(&map, moves);
    }

    #[test]
    fn test_part_two() {
        let (map, moves) = parse_data(EXAMPLE);
        let ans = part_two(&map, moves);

        assert_eq!(ans, 9021);
    }
}
