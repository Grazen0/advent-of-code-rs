use aoc_lib::cli::{PuzzleSolution, SolutionResult};

struct Day1;

fn actual_fuel(mut mass: i32) -> i32 {
    let mut sum = 0;

    loop {
        mass = (mass / 3) - 2;

        if mass > 0 {
            sum += mass;
        } else {
            break;
        }
    }

    sum
}

impl PuzzleSolution for Day1 {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        Ok(input.iter().map(|n| (n / 3) - 2).sum())
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        Ok(input.iter().map(|n| actual_fuel(*n)).sum())
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day1>(2019, 1);
}
