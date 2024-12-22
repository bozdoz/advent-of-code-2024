use std::{
    collections::HashMap,
    fmt::{ Debug, Display },
    fs,
    ops::{ Add, Mul },
    time::Instant,
    vec,
};
use lib::get_part;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

struct World<'a> {
    height: isize,
    width: isize,
    robots: &'a Vec<Robot>,
}

impl Display for World<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fill = f.fill();

        let should_fill = fill != ' ';

        let mut map = HashMap::new();

        for robot in self.robots.iter() {
            map.entry(robot.position)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }

        let mut out: Vec<String> = vec![];

        for r in 0..self.height {
            let mut row: Vec<String> = vec![];
            for c in 0..self.width {
                row.push(
                    map
                        .get(&Point(c, r))
                        .and_then(|v| {
                            if should_fill { Some(fill.to_string()) } else { Some(v.to_string()) }
                        })
                        .unwrap_or_else(|| {
                            if should_fill { " ".to_string() } else { ".".to_string() }
                        })
                );
            }
            out.push(row.join(""));
        }

        out.push("".to_string());

        f.write_str(&out.join("\n"))
    }
}

impl World<'_> {
    fn after(&self, time: usize) -> Vec<Robot> {
        self.robots
            .iter()
            .map(|r| {
                Robot {
                    position: self.wrap(r.after(time)),
                    ..*r // first time spreading?
                }
            })
            .collect()
    }

    fn wrap(&self, position: Point) -> Point {
        let (mut x, mut y) = (position.0 % self.width, position.1 % self.height);

        if x < 0 {
            x = self.width + x;
        }
        if y < 0 {
            y = self.height + y;
        }

        Point(x, y)
    }
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * (rhs as isize), self.1 * (rhs as isize))
    }
}

impl Robot {
    fn after(&self, time: usize) -> Point {
        self.position + self.velocity * time
    }
}

fn parse_data(data: &str) -> Vec<Robot> {
    // `r` allows single backslash
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("regex to work out");
    let mut robots = vec![];

    for capture in re.captures_iter(data) {
        let (_, [a, b, c, d]) = capture.extract();

        robots.push(Robot {
            position: Point(a.parse().unwrap(), b.parse().unwrap()),
            velocity: Point(c.parse().unwrap(), d.parse().unwrap()),
        });
    }

    robots
}

fn part_one(world: &World) -> usize {
    let mut quads = [0; 4];
    let v = world.height / 2;
    let h = world.width / 2;

    for robot in world.after(100) {
        if robot.position.0 < h {
            if robot.position.1 < v {
                // position here really doesn't matter; might be able to improve this
                quads[0] += 1;
            }
            if robot.position.1 > v {
                quads[2] += 1;
            }
        }
        if robot.position.0 > h {
            if robot.position.1 < v {
                quads[1] += 1;
            }
            if robot.position.1 > v {
                quads[3] += 1;
            }
        }
    }

    quads.iter().product()
}

// from chatgpt
fn _extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = _extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    return (gcd, x, y);
}

// from chatgpt
fn _modular_inverse(a: isize, m: isize) -> Option<isize> {
    let (gcd, x, _) = _extended_gcd(a, m);
    if gcd != 1 {
        return None;
    }
    // CHATGPT follow-up: Ensure the result is positive
    Some(((x % m) + m) % m)
}

fn _chinese_remainder_theorem(given: &Vec<(isize, isize)>) -> isize {
    let m: isize = given
        .iter()
        .map(|x| x.1)
        .product();

    let mi: Vec<isize> = given
        .iter()
        .map(|x| m / x.1)
        .collect();

    let sum: isize = given
        .iter()
        .zip(mi.iter())
        .map(|(a, &b)| {
            let i = _modular_inverse(b, a.1).expect("The answer to be able to be calculated");

            a.0 * b * i
        })
        .sum();

    sum % m
}

// ! This did not work at first; chatgpt had to help multiple times
// let x1 = (22, 103);
// let x2 = (79, 101);
// chinese_remainder_theorem(&vec![x1, x2])

// give me the fewest seconds elapsed to see the tree
fn part_two() -> usize {
    // 22 is when it appeared vertically centered
    // 79 is when it appeared horizontally centered
    // the other numbers are height and width (when it wraps)
    for time in 0.. {
        if time % 103 == 22 && time % 101 == 79 {
            return time;
        }
    }
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let robots = parse_data(&data);
    let world = World {
        width: 101,
        height: 103,
        robots: &robots,
    };

    if one {
        let now = Instant::now();
        let ans = part_one(&world);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two();
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    // run with `cargo test single -- --nocapture`
    #[test]
    fn test_single() {
        let world = World {
            height: 7,
            width: 11,
            robots: &vec![Robot { position: Point(2, 4), velocity: Point(2, -3) }],
        };

        for time in 0..=5 {
            let world = World {
                robots: &world.after(time),
                ..world
            };

            println!("After {time} seconds:\n{world}");
        }
    }

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let world = World {
            width: 11,
            height: 7,
            robots: &parse_data(EXAMPLE),
        };
        let ans = part_one(&world);

        assert_eq!(ans, 12);
    }

    #[test]
    fn test_chinese() {
        let ans = _chinese_remainder_theorem(&vec![(22, 103), (79, 101)]);

        assert_eq!(ans, 8159);
    }

    // run with `cargo test two -- --nocapture --include-ignored`
    #[test]
    #[ignore = "CI doesn't have input, and this was just to display repeating patterns"]
    fn test_part_two() {
        let data = fs::read_to_string("./src/input.txt").unwrap();

        let world = World {
            width: 101,
            height: 103,
            robots: &parse_data(&data),
        };

        // 125 & 180
        // 22 (103) & 79 (101)
        let mut iterations = 0;
        for time in 0..=9999 {
            if time % 103 == 22 && time % 101 == 79 {
                iterations += 1;
                let world = World {
                    robots: &world.after(time),
                    ..world
                };

                // did :#>8 to get a "fill" value for higher contrast
                println!("After {time} seconds:\n{world:#>8}");
            }
        }

        println!("Iterations: {iterations}")
    }
}
