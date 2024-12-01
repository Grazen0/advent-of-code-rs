use std::collections::HashMap;

use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};

struct Day1;

impl PuzzleSolution for Day1 {
    type Input = (Vec<i32>, Vec<i32>);

    fn parse_input(raw_input: Vec<String>) -> Self::Input {
        raw_input
            .iter()
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

    fn part_1(input: &Self::Input) -> SolutionResult {
        let (mut left, mut right) = input.clone();
        left.sort();
        right.sort();

        let sum = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(a, b)| a.abs_diff(b))
            .sum::<u32>();
        Ok(Box::new(sum))
    }

    fn part_2((left, right): &Self::Input) -> SolutionResult {
        let mut freqs = HashMap::<i32, usize>::new();

        for &n in right {
            let entry = freqs.entry(n).or_default();
            *entry += 1;
        }

        let sum = left
            .into_iter()
            .map(|&l| (l as usize) * freqs.get(&l).unwrap_or(&0))
            .sum::<usize>();
        Ok(Box::new(sum))
    }
}

fn main() {
    aoc_lib::cli::<Day1>(PuzzleDate::new(2024, 1));
}
