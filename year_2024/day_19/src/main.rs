use std::collections::{HashMap, HashSet};

use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::PriorityQueue,
};

#[derive(Debug, Clone, Default)]
struct Dessign(String);

impl Dessign {
    fn is_possible(&self, patterns: &[String]) -> bool {
        let mut seen = HashSet::new();
        let mut queue = PriorityQueue::new();
        queue.insert(0, 0);

        while let Some(ptr) = queue.pop_value() {
            for pat in patterns {
                if self.0[ptr..].starts_with(pat) {
                    let new_ptr = ptr + pat.len();

                    if new_ptr >= self.0.len() {
                        return true;
                    }

                    if seen.insert(new_ptr) {
                        queue.insert(new_ptr, new_ptr);
                    }
                }
            }
        }

        false
    }

    fn possibilities(&self, patterns: &[String], cache: &mut HashMap<String, usize>) -> usize {
        if let Some(&cached_result) = cache.get(&self.0) {
            return cached_result;
        }

        let mut count = 0;

        for pat in patterns {
            if self.0 == *pat {
                count += 1;
            } else if self.0.starts_with(pat) {
                let sub_dessign = Self(String::from(&self.0[pat.len()..]));
                count += sub_dessign.possibilities(patterns, cache);
            }
        }

        cache.insert(self.0.clone(), count);
        count
    }
}

struct Day19;

impl PuzzleSolution for Day19 {
    type Input = (Vec<String>, Vec<Dessign>);
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines: Vec<_> = raw_input.lines().collect();
        let patterns: Vec<_> = lines[0].split(", ").map(String::from).collect();
        let designs: Vec<_> = lines[2..]
            .iter()
            .map(|&s| Dessign(String::from(s)))
            .collect();

        (patterns, designs)
    }

    fn part_1((patterns, designs): &Self::Input) -> SolutionResult<Self::Output> {
        let count = designs.iter().filter(|d| d.is_possible(patterns)).count();
        Ok(count)
    }

    fn part_2((patterns, designs): &Self::Input) -> SolutionResult<Self::Output> {
        let mut cache = HashMap::new();

        let sum = designs
            .iter()
            .map(|d| d.possibilities(patterns, &mut cache))
            .sum();
        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day19>(2024, 19);
}
