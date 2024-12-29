use std::{ cmp::Ordering, collections::HashMap, fs, iter::repeat, time::Instant };
use lib::{ get_part, Grid };

// using char just for debugging
struct Keypad {
    current: char,
    map: HashMap<(char, char), Vec<char>>,
}

impl Clone for Keypad {
    fn clone(&self) -> Self {
        Self {
            // today I realized I have no clue what clone or copy do
            map: self.map.clone(),
            ..*self
        }
    }
}

// half of the problem was getting the initial paths right
fn get_map_of_keys(keys: &str, empty: (usize, usize)) -> HashMap<(char, char), Vec<char>> {
    let grid = Grid::new_with_chars(keys);
    let mut map = HashMap::new();

    // utility closure for repeating chars
    let get_path = |a: char, n: usize, b: char, x: usize| -> Vec<char> {
        repeat(a).take(n).chain(repeat(b).take(x)).collect::<Vec<char>>()
    };

    // get all paths to and from all keys
    for (r, c, &cell) in grid.iter() {
        // ignore empty
        if (r, c) == empty {
            continue;
        }
        // iterate every other cell
        for (y, x, &other) in grid.iter() {
            if (y, x) == empty || cell == other {
                continue;
            }

            let ud = r.abs_diff(y);
            let lr = c.abs_diff(x);

            // prefer left-up, left-down, down-right, up-right
            // avoid empty
            // group similar directions together
            let mut path = match (r.cmp(&y), c.cmp(&x)) {
                // left-up
                (Ordering::Greater, Ordering::Greater) => get_path('<', lr, '^', ud),
                // left-down
                (Ordering::Less, Ordering::Greater) => get_path('<', lr, 'v', ud),
                // down-right
                (Ordering::Less, Ordering::Less) => get_path('v', ud, '>', lr),
                // up-right
                (Ordering::Greater, Ordering::Less) => get_path('^', ud, '>', lr),
                (Ordering::Equal, Ordering::Less) => repeat('>').take(lr).collect(),
                (Ordering::Equal, Ordering::Greater) => repeat('<').take(lr).collect(),
                (Ordering::Less, Ordering::Equal) => repeat('v').take(ud).collect(),
                (Ordering::Greater, Ordering::Equal) => repeat('^').take(ud).collect(),
                _ => panic!("equal, equal!? {r} {c} {y} {x} {cell} {other}"),
            };

            // reverse any that go over the empty
            if (r == empty.0 || c == empty.1) && (y == empty.0 || x == empty.1) {
                path = path.into_iter().rev().collect::<Vec<_>>();
            }

            map.insert((cell, other), path);
        }
    }

    // print out a beautifully sorted list of paths
    // let mut sorted = map.keys().collect::<Vec<_>>();

    // sorted.sort();

    // for key in sorted {
    //     println!("{key:?} {:?}", map.get(key).unwrap());
    // }

    map
}

impl Keypad {
    fn new_directional() -> Self {
        Self {
            map: get_map_of_keys(" ^A\n<v>", (0, 0)),
            current: 'A',
        }
    }

    fn new_numeric() -> Self {
        Self {
            map: get_map_of_keys("789\n456\n123\n 0A", (3, 0)),
            current: 'A',
        }
    }

    fn move_to(&mut self, key: char) -> Option<&Vec<char>> {
        if self.current == key {
            // we're already here; now what?
            return None;
        }
        let path = self.map
            .get(&(self.current, key))
            .unwrap_or_else(|| { panic!("what is this? {key}") });

        // update current
        self.current = key;

        Some(path)
    }

    /**
     * Move from A, to a bunch of places, then press A
     * this is chunked into separate paths for memoization
     */
    fn move_sequence(&self, keys: &Vec<char>) -> Vec<Vec<char>> {
        let mut cur = 'A';
        let mut path = vec![];

        for &next in keys {
            let mut inner = vec![];
            if let Some(p) = self.map.get(&(cur, next)) {
                inner.extend(p);
                cur = next;
            }
            inner.push('A');

            path.push(inner);
        }

        path
    }
}

/* In summary, there are the following keypads:

    One directional keypad that you are using.
    Two directional keypads that robots are using.
    One numeric keypad (on a door) that a robot is using.
*/
// TODO: I didn't need any of this
fn get_keypads(num: usize) -> Vec<Keypad> {
    let direction = Keypad::new_directional();

    let mut keypads = vec![Keypad::new_numeric(), direction.clone()];

    for _ in 0..num {
        keypads.push(direction.clone());
    }

    keypads
}

fn part_one(data: &str) -> usize {
    let mut keypads = get_keypads(2);
    let keypad_count = keypads.len();
    let mut complexities = 0;

    for code in data.lines() {
        // get numeric from code
        let num = code
            .chars()
            .take_while(|x| x.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .expect("I thought code was a number");

        let mut len = 0;

        for key in code.chars() {
            // starts at 1
            let mut path = vec![key];
            for (i, pad) in keypads.iter_mut().enumerate() {
                if i == keypad_count - 1 {
                    // count the steps you take; not the robots
                    len += path.len();
                }
                // is fold better than declaring `let mut next_path`?
                // path gets updated for next keypad
                path = path.iter().fold(vec![], |mut acc, &p| {
                    // is there a better way to update and return Vec<char>?
                    if let Some(travelled) = pad.move_to(p) {
                        acc.extend(travelled);
                    }
                    // always push A
                    acc.push('A');
                    acc
                });
            }
        }

        // println!("{num} {len}\n");

        complexities += num * len;
    }

    complexities
}

// reworked this quite a bit: keypads is not mutable! (i.e. make it so we don't re-assign `.current`)
fn dfs(
    key: Vec<char>,
    i: usize,
    keypads: &Vec<Keypad>,
    // TIL: why this is not `mut memo`: we don't need/want to reassign `memo`
    memo: &mut HashMap<(Vec<char>, usize), usize>
) -> usize {
    // check if we're done
    if i == keypads.len() - 1 {
        return key.len();
    }

    // annoying clone here
    let k = (key.clone(), i);

    // ? did we do this before?
    if memo.contains_key(&k) {
        return *memo.get(&k).unwrap();
    }

    // ? get next
    // refactored from Vec<char> to Vec<Vec<>>
    // so we can take advantage of memoization
    let next = keypads[i].move_sequence(&key);

    let len = next
        .into_iter()
        .map(|p| dfs(p, i + 1, keypads, memo))
        .sum();

    // ? save to memo and return
    memo.insert(k, len);

    len
}

fn part_two(data: &str, num: usize) -> usize {
    let keypads = get_keypads(num);
    let mut memo = HashMap::new();

    data.lines()
        .map(|code| {
            // chatgpt for string to numeric
            let num = code
                .chars()
                .take_while(|x| x.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .expect("I thought code was a number");

            dfs(code.chars().collect::<Vec<_>>(), 0, &keypads, &mut memo) * num
        })
        .sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let data = fs::read_to_string("./src/input.txt").unwrap();

    if one {
        let now = Instant::now();
        let ans = part_one(&data);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&data, 25);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_map() {
        let keypad = Keypad::new_numeric();
        assert_eq!(keypad.map.get(&('7', '8')), Some(&vec!['>']));
        assert_eq!(keypad.map.get(&('7', '9')), Some(&vec!['>', '>']));
        assert_eq!(keypad.map.get(&('8', 'A')), Some(&vec!['v', 'v', 'v', '>']));
        assert_eq!(keypad.map.get(&('7', 'A')), Some(&vec!['>', '>', 'v', 'v', 'v']));
    }

    #[test]
    fn test_zero() {
        let ans = part_one("0");

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_one() {
        let ans = part_one(EXAMPLE);

        assert_eq!(ans, 126384);
    }

    #[test]
    fn test_part_two() {
        // test that it's equal to part one's implementation
        let ans = part_two(EXAMPLE, 2);

        assert_eq!(ans, 126384);
    }
}
