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

pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub height: isize,
    pub width: isize,
}

impl Grid {
    pub fn new(data: &str) -> Self {
        let cells: Vec<_> = data
            .lines()
            .map(|l| { l.chars().collect::<Vec<_>>() })
            .collect();

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
