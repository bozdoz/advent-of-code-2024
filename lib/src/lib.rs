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
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
    }
}

// NOTE: if I wanted to dereference this, I could do:
// where T: Copy, T: Clone
impl<T> Grid<T> {
    pub fn get(&self, pos: (isize, isize)) -> Option<&T> {
        if pos.0 == -1 || pos.1 == -1 || pos.0 >= self.height || pos.1 >= self.width {
            return None;
        }

        Some(&self.cells[pos.0 as usize][pos.1 as usize])
    }
}
