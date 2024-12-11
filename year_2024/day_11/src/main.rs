use std::collections::HashMap;

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

fn blink_stones_n(stones: &mut HashMap<usize, usize>, blink_count: usize) {
    for _ in 0..blink_count {
        blink_stones(stones);
    }
}

fn blink_stones(stones: &mut HashMap<usize, usize>) {
    let mut new_stones = HashMap::new();

    for (&n, &freq) in &*stones {
        if n == 0 {
            *new_stones.entry(1).or_default() += freq;
        } else {
            let digits = n.to_string();

            if digits.len() % 2 == 0 {
                let mid = digits.len() / 2;
                let left = digits[..mid].parse().unwrap();
                let right = digits[mid..].parse().unwrap();

                *new_stones.entry(left).or_default() += freq;
                *new_stones.entry(right).or_default() += freq;
            } else {
                *new_stones.entry(2024 * n).or_default() += freq;
            }
        }
    }

    *stones = new_stones;
}

struct Day11;

impl PuzzleSolution for Day11 {
    type Input = HashMap<usize, usize>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut stones = HashMap::new();

        for n in raw_input.split_whitespace().map(|s| s.parse().unwrap()) {
            *stones.entry(n).or_default() += 1;
        }

        stones
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut stones = input.clone();
        blink_stones_n(&mut stones, 25);
        Ok(stones.values().sum())
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut stones = input.clone();
        blink_stones_n(&mut stones, 75);
        Ok(stones.values().sum())
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day11>(2024, 11);
}
