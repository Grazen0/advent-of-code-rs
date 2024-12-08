use std::collections::HashMap;

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

struct Day1;

impl PuzzleSolution for Day1 {
    type Input = (Vec<i32>, Vec<i32>);
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .lines()
            .map(|line| {
                let parts = line
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                (parts[0], parts[1])
            })
            .unzip()
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let (mut left, mut right) = input.clone();
        left.sort();
        right.sort();

        let sum = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(a, b)| a.abs_diff(b) as usize)
            .sum::<usize>();
        Ok(sum)
    }

    fn part_2((left, right): &Self::Input) -> SolutionResult<Self::Output> {
        let mut freqs = HashMap::<i32, usize>::new();

        for &n in right {
            let entry = freqs.entry(n).or_default();
            *entry += 1;
        }

        let sum = left
            .into_iter()
            .map(|&l| (l as usize) * freqs.get(&l).unwrap_or(&0))
            .sum::<usize>();
        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day1>(2024, 1);
}
