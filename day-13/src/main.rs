#![allow(non_snake_case)]

use std::{ cmp::Ordering, fs, ops::{ Add, Div, Mul }, time::Instant };
use lib::get_part;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(usize, usize);

struct Machine {
    A: Point,
    B: Point,
    P: Point,
}

fn parse_data(data: &str) -> Vec<Machine> {
    data.split("\n\n")
        .map(|block| {
            let lines: Vec<_> = block
                .lines()
                .map(|l| {
                    let (x, y) = l.split_once("X").unwrap().1.split_once(", Y").unwrap();

                    (x[1..].parse::<usize>().unwrap(), y[1..].parse::<usize>().unwrap())
                })
                .collect();

            Machine {
                A: Point(lines[0].0, lines[0].1),
                B: Point(lines[1].0, lines[1].1),
                P: Point(lines[2].0, lines[2].1),
            }
        })
        .collect()
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Div for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Point(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Mul<usize> for Point {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl PartialOrd for Point {
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0 && self.1 > other.1
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if *self > *other {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl Machine {
    // worked for part 1
    fn _least_tokens(&self) -> Option<Point> {
        // start with maximum B
        let max_a = {
            let a = self.P / self.A;
            a.0.min(a.1)
        };
        let b = self.P / self.B;
        // take the lowest possible in b and multiply B
        let mut lowest_b = b.0.min(b.1);
        let mut lowest_a = 0;

        while lowest_a <= max_a {
            let eh = self.A * lowest_a;
            let bee = self.B * lowest_b;
            let sum = eh + bee;

            if sum == self.P {
                return Some(Point(lowest_a, lowest_b));
            }

            if sum > self.P {
                // decrement b
                lowest_b -= 1;
            } else {
                lowest_a += 1;
            }
        }

        None
    }

    /**

u/CorvusCalvaria

Explanation:

Given the puzzle:

Button A: X+a1, Y+a2
Button B: X+b1, Y+b2
Prize: X=c1, Y=c2

This can be represented as a pair of straight lines, pointing down and to the right, as follows:

(b1)y = -(a1)x + c1
(b2)y = -(a2)x + c2

Where the intersection of these lines (i_x, i_y)tells you that you need i_x A presses and i_y B presses to reach the prize. If i_x or i_y is negative or not a whole number, then there's no solution (orange dots).

 */
    fn least_tokens_v2(&self) -> Option<Point> {
        // get intersection of lines
        // cross multiplication rule
        // I don't know why these are negative
        let a1 = -(self.A.0 as f64);
        let a2 = -(self.A.1 as f64);
        let b1 = -(self.B.0 as f64);
        let b2 = -(self.B.1 as f64);
        let p1 = self.P.0 as f64;
        let p2 = self.P.1 as f64;

        let x = (b1 * p2 - b2 * p1) / (a1 * b2 - a2 * b1);
        let y = (p1 * a2 - p2 * a1) / (a1 * b2 - a2 * b1);

        // find negative
        if x < 0.0 || y < 0.0 {
            return None;
        }

        // find decimals
        if x == x.trunc() && y == y.trunc() {
            // no decimals?
            return Some(Point(x as usize, y as usize));
        }

        None
    }
}

fn part_one(machines: &Vec<Machine>) -> usize {
    let mut cost = 0;
    for machine in machines.iter() {
        if let Some(point) = machine.least_tokens_v2() {
            cost += point.0 * 3 + point.1;
        }
    }

    cost
}

fn part_two(machines: &Vec<Machine>) -> usize {
    let mut cost = 0;
    for machine in machines {
        let thousand = Machine {
            A: machine.A,
            B: machine.B,
            P: Point(machine.P.0 + 10000000000000, machine.P.1 + 10000000000000),
        };

        if let Some(point) = thousand.least_tokens_v2() {
            cost += point.0 * 3 + point.1;
        }
    }
    cost
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let machines = parse_data(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&machines);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&machines);
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
        let ans = part_one(&parse_data(EXAMPLE));

        assert_eq!(ans, 480);
    }
}
