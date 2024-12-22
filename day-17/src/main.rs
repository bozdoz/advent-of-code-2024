use std::{ fs, thread, time::Instant };

trait Instruction {
    fn run(program: &mut Program, operand: usize) -> ();
}

struct Combo {}

impl Combo {
    fn from(program: &Program, value: usize) -> usize {
        match value {
            0..=3 => value,
            4 => program.a,
            5 => program.b,
            6 => program.c,
            _ => panic!("Why is this a combo? {value}"),
        }
    }
}

/**
 * performs division.
 * The numerator is the value in the A register.
 * The denominator is found by raising 2 to the power of the instruction's combo operand.
 */
struct Adv {}

impl Instruction for Adv {
    fn run(program: &mut Program, operand: usize) {
        program.a = Self::div(program, operand);
    }
}

impl Adv {
    fn div(program: &Program, operand: usize) -> usize {
        let lhs = program.a;
        // combo
        let rhs = (2usize).pow(Combo::from(program, operand) as u32);

        lhs / rhs
    }
}

/**
 * calculates the bitwise XOR of register B and the instruction's literal operand,
 * then stores the result in register B.
 */
struct Bxl {}

impl Instruction for Bxl {
    fn run(program: &mut Program, operand: usize) {
        let lhs = program.b;
        let rhs = operand;

        program.b = lhs ^ rhs;
    }
}

/**
 * calculates the value of its combo operand modulo 8, then outputs that value.
 */
struct Out {}

impl Instruction for Out {
    fn run(program: &mut Program, operand: usize) {
        // combo
        let rhs = Combo::from(program, operand);

        program.output.push(rhs % 8);
    }
}

/**
 * does nothing if the A register is 0.
 * However, if the A register is not zero,
 * it jumps by setting the instruction pointer to the value of its literal operand;
 * if this instruction jumps, the instruction pointer is not increased by 2
 * after this instruction.
 */
struct Jnz {}

impl Instruction for Jnz {
    fn run(program: &mut Program, operand: usize) {
        if program.a == 0 {
            return;
        }
        program.pointer = operand;
    }
}

/**
 * calculates the value of its combo operand modulo 8
 * (thereby keeping only its lowest 3 bits),
 * then writes that value to the B register.
 */
struct Bst {}

impl Instruction for Bst {
    fn run(program: &mut Program, operand: usize) {
        program.b = Combo::from(&program, operand) % 8;
    }
}

/**
 * calculates the bitwise XOR of register B and register C,
 * then stores the result in register B.
 */
struct Bxc {}

impl Instruction for Bxc {
    fn run(program: &mut Program, _operand: usize) {
        program.b = program.b ^ program.c;
    }
}

/**
 * Same as Adv but for B register
 */
struct Bdv {}

impl Instruction for Bdv {
    fn run(program: &mut Program, operand: usize) {
        program.b = Adv::div(program, operand);
    }
}

/**
 * Same as Adv but for C register
 */
struct Cdv {}

impl Instruction for Cdv {
    fn run(program: &mut Program, operand: usize) {
        program.c = Adv::div(program, operand);
    }
}

#[derive(Debug)]
struct Program {
    a: usize,
    b: usize,
    c: usize,
    pointer: usize,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl Program {
    // this is so awkward
    fn new(data: &str) -> Self {
        let mut lines = data.lines().filter_map(|l| {
            // needs and_then for the empty line
            l.split_once(": ").and_then(|x| Some(x.1))
        });

        let a = lines.next().unwrap().parse().unwrap();
        let b = lines.next().unwrap().parse().unwrap();
        let c = lines.next().unwrap().parse().unwrap();

        let input = lines
            .next()
            .unwrap()
            .split(",")
            .filter_map(|d| d.parse().ok())
            .collect();

        Self {
            a,
            b,
            c,
            pointer: 0,
            input,
            output: vec![],
        }
    }

    fn run(&mut self) -> Option<()> {
        loop {
            // first time using question marks
            let inst: &usize = self.input.get(self.pointer)?;
            // dereference to avoid borrowing as immutable
            let next: usize = *self.input.get(self.pointer + 1)?;

            // bump pointer by 2
            self.pointer += 2;

            (match inst {
                0 => Adv::run,
                1 => Bxl::run,
                2 => Bst::run,
                3 => Jnz::run,
                4 => Bxc::run,
                5 => Out::run,
                6 => Bdv::run,
                7 => Cdv::run,
                _ => panic!("what instruction is this? {inst}"),
            })(self, next);
        }
    }

    fn reset(&mut self) {
        // ignoring "a"
        self.pointer = 0;
        self.b = 0;
        self.c = 0;
        // first time using clear
        self.output.clear();
    }
}

fn part1(program: &mut Program) -> String {
    program.run();

    program.output
        .iter()
        .map(|x| { x.to_string() })
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(program: &mut Program) -> usize {
    let mut queue = vec![];

    queue.push((0, program.input.len() - 1));

    let mut answers = vec![];

    while let Some(state) = queue.pop() {
        // get next states
        for i in 0..8 {
            // got this from maneatingape
            let next_a = (state.0 << 3) | i;

            program.reset();
            program.a = next_a;
            program.run();

            // println!("{next_a} {:?}", program.output);

            if program.output[0] == program.input[state.1] {
                // check if done
                if state.1 == 0 {
                    answers.push(next_a);
                } else {
                    queue.push((next_a, state.1 - 1));
                }
            }
        }
    }

    // 216_148_338_630_253
    *answers.iter().min().unwrap_or(&0)
}

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("./src/input.txt").unwrap();

    let mut program = Program::new(&input);

    let now = Instant::now();
    let ans = part1(&mut program);

    println!("Part one: {:?} {:?}", ans, now.elapsed());

    program.reset();

    let now = Instant::now();
    let ans = part2(&mut program);
    println!("Part two: {:?} {:?}", ans, now.elapsed());

    println!("Time: {:?}", start.elapsed());
}

fn _crazy_idea_i_had() {
    let _start: usize = 36_000_000_000;
    let _inc = 800_000_000;
    let mut handles = vec![];

    for _ in 0..5 {
        handles.push(
            thread::spawn(
                move || {
                    // part2()
                }
            )
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
