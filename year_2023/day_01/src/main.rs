use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};

struct Day1;

impl PuzzleSolution for Day1 {
    type Input = Vec<String>;

    fn parse_input(raw_input: Vec<String>) -> Self::Input {
        raw_input
    }

    fn part_1(_input: &Self::Input) -> SolutionResult {
        Ok(Box::new("Hello, world!"))
    }
}

fn main() {
    aoc_lib::cli::<Day1>(PuzzleDate::new(2023, 1));
}
