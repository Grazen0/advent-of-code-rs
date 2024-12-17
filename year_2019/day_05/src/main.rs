use aoc_lib::cli::{PuzzleSolution, SolutionResult};

struct Day5;

impl PuzzleSolution for Day5 {
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
        todo!()
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day5>(2019, 5);
}
