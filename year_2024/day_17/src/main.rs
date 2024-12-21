use std::fmt::Display;

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

#[derive(Debug, Clone)]
struct Day17Input {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StepStatus {
    Continue,
    Halt,
}

#[derive(Debug, Clone, Default)]
struct WeirdComputer {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl WeirdComputer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        loop {
            let status = self.step();

            match status {
                StepStatus::Halt => break,
                _ => {}
            }
        }
    }

    pub fn step(&mut self) -> StepStatus {
        let Some(opcode) = self.program.get(self.pc) else {
            return StepStatus::Halt;
        };

        self.pc += 1;

        match opcode {
            0 => {
                let pow = self.read_combo();
                self.a /= 2_u64.pow(pow as u32);
            }
            1 => {
                let operand = self.read_literal();
                self.b ^= operand as u64;
            }
            2 => {
                let val = self.read_combo();
                self.b = val % 8;
            }
            3 => {
                let dest = self.read_literal();

                if self.a != 0 {
                    self.pc = dest as usize;
                }
            }
            4 => {
                self.read_literal(); // Legacy reasons
                self.b ^= self.c;
            }
            5 => {
                let val = self.read_combo();
                self.output.push((val % 8) as u8);
            }
            6 => {
                let pow = self.read_combo();
                self.b = self.a / 2_u64.pow(pow as u32);
            }
            7 => {
                let pow = self.read_combo();
                self.c = self.a / 2_u64.pow(pow as u32);
            }
            _ => panic!("invalid opcode: {}", opcode),
        }

        StepStatus::Continue
    }

    fn read_literal(&mut self) -> u8 {
        let val = self.program[self.pc];
        self.pc += 1;
        val
    }

    fn read_combo(&mut self) -> u64 {
        let val = self.read_literal();

        match val {
            0..=3 => val as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand: {}", val),
        }
    }
}

fn concat_octals(arr: &[u8]) -> u64 {
    let mut result = 0;

    for (i, &n) in arr.iter().rev().enumerate() {
        if n >= 8 {
            panic!("Octal number should be between 0 and 7");
        }

        result |= (n as u64) << (3 * i);
    }

    result
}

struct Day17;

impl PuzzleSolution for Day17 {
    type Input = Day17Input;
    type Output = Box<dyn Display>;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.lines().collect::<Vec<_>>();
        let regs = lines[..3]
            .iter()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect::<Vec<_>>();

        let program = lines
            .last()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Day17Input {
            reg_a: regs[0],
            reg_b: regs[1],
            reg_c: regs[2],
            program,
        }
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut computer = WeirdComputer::new();
        computer.a = input.reg_a;
        computer.b = input.reg_b;
        computer.c = input.reg_c;
        computer.program = input.program.clone();

        computer.run();

        Ok(Box::new(
            computer
                .output
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ))
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut digits = vec![0; input.program.len()];
        let mut i = 0;

        while i < digits.len() {
            let mut computer = WeirdComputer::new();
            computer.a = concat_octals(&digits);
            computer.b = input.reg_b;
            computer.c = input.reg_c;
            computer.program = input.program.clone();

            loop {
                let status = computer.step();

                match status {
                    StepStatus::Continue => {}
                    StepStatus::Halt => break,
                }
            }

            let i_rev = digits.len() - 1 - i;

            if computer.output.get(i_rev) == computer.program.get(i_rev) {
                i += 1;
                continue;
            }

            if digits[i] >= 7 {
                digits[i] = 0;
                i -= 1;
            }

            digits[i] += 1;
        }

        Ok(Box::new(concat_octals(&digits)))
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day17>(2024, 17);
}
