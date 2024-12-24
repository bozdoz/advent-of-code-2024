use std::env;

pub fn get_part() -> (bool, bool) {
    let args = env::args().skip(1);

    let mut hasone = false;
    let mut hastwo = false;

    for arg in args {
        if arg.contains(&String::from("one")) {
            hasone = true;
        }
        if arg.contains(&String::from("two")) {
            hastwo = true;
        }
    }

    if !hasone && !hastwo {
        // run them both by default
        hasone = true;
        hastwo = true;
    }

    (hasone, hastwo)
}

// (r, c) differences, clockwise
pub const DIRS: [(isize, isize); 4] = [
    (-1, 0), // top
    (0, 1), // right
    (1, 0), // bottom
    (0, -1), // left
];

pub struct Grid<T = char> {
    pub cells: Vec<Vec<T>>,
    pub height: isize,
    pub width: isize,
}

impl<T> Grid<T> {
    fn _get_grid(data: &str, mapper: impl FnMut(&str) -> Vec<T>) -> Self {
        let cells: Vec<_> = data.lines().map(mapper).collect();

        // surprisingly not valid if added directly to the `Self` block below
        let height = cells.len() as isize;
        let width = cells[0].len() as isize;

        Self {
            cells,
            height,
            width,
        }
    }
}

impl Grid<char> {
    pub fn new_with_chars(data: &str) -> Self {
        Grid::_get_grid(data, |l| { l.chars().collect::<Vec<_>>() })
    }
}

impl Grid<u32> {
    pub fn new_with_u32(data: &str) -> Self {
        Grid::_get_grid(data, |l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
    }
}

// NOTE: if I wanted to dereference this, I could do:
// where T: Copy, T: Clone
// Or maybe also:
// where T: Copy + Clone
impl<T> Grid<T> {
    pub fn get(&self, pos: (isize, isize)) -> Option<&T> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.height || pos.1 >= self.width {
            return None;
        }

        Some(&self.cells[pos.0 as usize][pos.1 as usize])
    }
}

// token trees below
#[macro_export]
macro_rules! tup {
    // unfortunate; can't use `state.position` as a `tt`
    ($lhs:expr, $op:tt, $rhs:expr) => {
        {
            ($lhs.0 $op $rhs.0, $lhs.1 $op $rhs.1)
        }
    };
    ($lhs:tt $op:tt $rhs:tt) => {
        {
            ($lhs.0 $op $rhs.0, $lhs.1 $op $rhs.1)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tup_add() {
        assert_eq!(tup!((1, 2) + (2, 3)), (3, 5));
    }

    #[test]
    fn test_struct_mul() {
        struct State {
            position: (isize, isize),
        }
        let a = State { position: (3, 5) };

        assert_eq!(tup!(a.position, *, (2, 3)), (6, 15));
    }
}
