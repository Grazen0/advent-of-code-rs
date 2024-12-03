use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};
use intcode::IntcodeMachine;

struct Day2;

impl PuzzleSolution for Day2 {
    type Input = Vec<i64>;
    type Output = i64;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut machine = IntcodeMachine::new();
        machine.load_program(input);
        machine.memory_mut()[1] = 12;
        machine.memory_mut()[2] = 2;

        machine.run_until_halt().unwrap();

        Ok(machine.memory()[0])
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut machine = IntcodeMachine::new();
                machine.load_program(input);
                machine.memory_mut()[1] = noun;
                machine.memory_mut()[2] = verb;

                machine.run_until_halt().unwrap();

                if machine.memory()[0] == 19690720 {
                    return Ok(100 * noun + verb);
                }
            }
        }

        Err(aoc_lib::SolutionError::BadInput)
    }
}

fn main() {
    aoc_lib::cli::<Day2>(PuzzleDate::new(2019, 2));
}
