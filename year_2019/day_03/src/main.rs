use aoc_lib::cli::{PuzzleSolution, SolutionResult};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone)]
struct Instruction(i32, Direction);

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let distance = s[1..].parse().unwrap();

        let direction = match s.as_bytes()[0] {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            _ => panic!("invalid step direction"),
        };

        Self(distance, direction)
    }
}

#[derive(Debug, Clone, Default)]
struct Pipe {
    x: i32,
    y: i32,
    steps_taken: u32,
    history: HashMap<(i32, i32), u32>,
}

impl Pipe {
    fn log_position(&mut self) {
        self.history
            .entry((self.x, self.y))
            .or_insert(self.steps_taken);
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }

        self.steps_taken += 1;
    }
}

struct Day3;

impl PuzzleSolution for Day3 {
    type Input = (Vec<Instruction>, Vec<Instruction>);
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut pipe_steps = raw_input
            .lines()
            .map(|line| line.split(",").map(Instruction::from).collect::<Vec<_>>());

        let first = pipe_steps.next().unwrap();
        let second = pipe_steps.next().unwrap();
        (first, second)
    }

    fn part_1((instructions_a, instructions_b): &Self::Input) -> SolutionResult<Self::Output> {
        let mut pipe_a = Pipe::default();
        let mut pipe_b = Pipe::default();

        let mut intersections = Vec::new();

        pipe_a.log_position();

        for Instruction(distance, direction) in instructions_a {
            for _ in 0..*distance {
                pipe_a.step(*direction);
                pipe_a.log_position();
            }
        }

        for Instruction(distance, direction) in instructions_b {
            for _ in 0..*distance {
                pipe_b.step(*direction);

                if pipe_a.history.contains_key(&(pipe_b.x, pipe_b.y)) {
                    intersections.push((pipe_b.x, pipe_b.y));
                }
            }
        }

        Ok(intersections
            .iter()
            .map(|(x, y)| x.abs_diff(*y))
            .min()
            .unwrap())
    }
    fn part_2((instructions_a, instructions_b): &Self::Input) -> SolutionResult<Self::Output> {
        let mut pipe_a = Pipe::default();
        let mut pipe_b = Pipe::default();
        let mut intersections = Vec::new();

        pipe_a.log_position();

        for Instruction(distance, direction) in instructions_a {
            for _ in 0..*distance {
                pipe_a.step(*direction);
                pipe_a.log_position();
            }
        }

        for Instruction(distance, direction) in instructions_b {
            for _ in 0..*distance {
                pipe_b.step(*direction);

                if let Some(steps_a_taken) = pipe_a.history.get(&(pipe_b.x, pipe_b.y)) {
                    intersections.push(steps_a_taken + pipe_b.steps_taken);
                }
            }
        }

        Ok(intersections.into_iter().min().unwrap())
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day3>(2019, 3);
}
