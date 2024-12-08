use aoc_lib::cli::{PuzzleSolution, SolutionResult};
use std::ops::RangeInclusive;

fn digits(mut n: u32) -> Vec<u32> {
    let mut digits = Vec::new();

    while n != 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }

    digits
}

struct Day4;

impl PuzzleSolution for Day4 {
    type Input = RangeInclusive<u32>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let (min, max) = raw_input.trim().split_once("-").unwrap();
        min.parse().unwrap()..=max.parse().unwrap()
    }

    fn part_1(range: &Self::Input) -> SolutionResult<Self::Output> {
        let count = range
            .clone()
            .into_iter()
            .filter(|passwd| {
                let windows = digits(*passwd)
                    .windows(2)
                    .map(|w| (w[0], w[1]))
                    .collect::<Vec<_>>();

                windows.iter().any(|(a, b)| a == b) && windows.iter().all(|(a, b)| a <= b)
            })
            .count();

        Ok(count)
    }
    fn part_2(range: &Self::Input) -> SolutionResult<Self::Output> {
        let count = range
            .clone()
            .into_iter()
            .filter(|passwd| {
                let windows = digits(*passwd)
                    .windows(2)
                    .map(|w| (w[0], w[1]))
                    .collect::<Vec<_>>();

                windows.iter().enumerate().any(|(i, &(a, b))| {
                    a == b
                        && (i == 0 || windows[i - 1].0 != a)
                        && (i == windows.len() - 1 || windows[i + 1].1 != a)
                }) && windows.iter().all(|(a, b)| a <= b)
            })
            .count();

        Ok(count)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day4>(2019, 4);
}
