use std::{ collections::{ HashMap, HashSet }, fs, time::Instant };
use lib::get_part;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Op {
    AND,
    XOR,
    OR,
}

#[derive(Debug)]
struct System<'a> {
    wires: HashMap<&'a str, u8>,
    gates: HashMap<&'a str, (&'a str, &'a str, Op)>,
    zeds: isize,
}

// TODO: figure out the difference between impl<'a>, System<'_>, and fn new<'a>
impl<'a> System<'a> {
    fn new(data: &'a str) -> Self {
        let mut wires: HashMap<&str, u8> = HashMap::new();
        let mut gates: HashMap<&str, (&str, &str, Op)> = HashMap::new();
        let mut zeds = 0;

        let (w, g) = data.split_once("\n\n").unwrap();

        for w in w.lines() {
            let (name, number) = w.split_once(": ").unwrap();

            wires.insert(name, if number == "1" { 1 } else { 0 });
        }

        for g in g.lines() {
            let arr = g.split(" ").collect::<Vec<_>>();
            let wire = arr[4];

            if wire.starts_with("z") {
                zeds += 1;
            }

            gates.insert(wire, (
                arr[0],
                arr[2],
                match arr[1] {
                    "AND" => Op::AND,
                    "XOR" => Op::XOR,
                    "OR" => Op::OR,
                    _ => panic!("What is this?! {}", arr[1]),
                },
            ));
        }

        Self {
            wires,
            gates,
            zeds,
        }
    }
}

fn part_one(system: &mut System) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let gates = &system.gates;
    let wires = &mut system.wires;

    while visited.len() < gates.len() {
        for (wire, v) in gates.iter() {
            if visited.contains(wire) {
                continue;
            }
            let (a, b, op) = v;

            let a = wires.get(a);
            let b = wires.get(b);

            if let Some(a) = a {
                if let Some(b) = b {
                    wires.insert(wire, match op {
                        Op::AND => a & b,
                        Op::XOR => a ^ b,
                        Op::OR => a | b,
                    });
                    visited.insert(wire);
                }
            }
        }
    }

    let mut ans: usize = 0;
    let mut zeds = system.zeds;

    while zeds >= 0 {
        zeds -= 1;

        // leading zeroes
        let val = format!("z{:0>2}", zeds);

        if let Some(v) = wires.get(val.as_str()) {
            ans <<= 1;
            ans |= *v as usize;
        }
    }

    ans
}

fn part_two(system: &System) -> String {
    //
    // logic from maneatingape
    //
    let mut nextgate = HashSet::new();

    for (_, (left, right, op)) in system.gates.iter() {
        nextgate.insert((left, op));
        nextgate.insert((right, op));
    }

    let mut swap = vec![];
    let x0 = &"x00";

    for (gate, (left, right, op)) in system.gates.iter() {
        match op {
            Op::AND => {
                // Check that all AND gates point to an OR, except for first AND.
                if left != x0 && right != x0 && !nextgate.contains(&(gate, &Op::OR)) {
                    swap.push(gate);
                }
            }
            Op::OR => {
                // Check that only XOR gates point to output, except for last carry which is OR.
                // OR can never point to OR.
                if (gate.starts_with("z") && gate != &"z45") || nextgate.contains(&(gate, &Op::OR)) {
                    swap.push(gate);
                }
            }
            Op::XOR => {
                // Check that first level XOR points to second level XOR, except for first XOR.
                if left.starts_with("x") || right.starts_with("x") {
                    if left != x0 && right != x0 && !nextgate.contains(&(gate, &Op::XOR)) {
                        swap.push(gate);
                    }
                } else if !gate.starts_with("z") {
                    // Second level XOR must point to output.
                    swap.push(gate);
                }
            }
        }
    }

    swap.sort();

    swap.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();
    let mut system = System::new(&data);

    if one {
        let now = Instant::now();
        let ans = part_one(&mut system);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&system);
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
        let ans = part_one(&mut System::new(EXAMPLE));

        assert_eq!(ans, 2024);
    }
}
