use std::{num::ParseIntError, str::FromStr};

use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};

#[derive(Debug, Clone)]
struct OrderingRule(u32, u32);

impl FromStr for OrderingRule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("|").unwrap();
        Ok(Self(a.parse()?, b.parse()?))
    }
}

#[derive(Debug, Clone)]
struct Update(Vec<u32>);

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(",")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(nums))
    }
}

impl Update {
    fn satisfies_rule(&self, rule: &OrderingRule) -> bool {
        for (i, &n) in self.0.iter().enumerate() {
            if n == rule.0 {
                return true;
            }

            if n == rule.1 {
                return !self.0[i..].contains(&rule.0);
            }
        }

        true
    }

    fn fix_rule(&mut self, rule: &OrderingRule) -> bool {
        if let Some(start) = self.0.iter().position(|&n| n == rule.1) {
            if let Some(end) = self.0[start..].iter().position(|&n| n == rule.0) {
                self.0.swap(start, start + end);
                return true;
            }
        }

        false
    }

    fn midpoint(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}

struct Day5;

impl PuzzleSolution for Day5 {
    type Input = (Vec<OrderingRule>, Vec<Update>);
    type Output = u32;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.trim().lines().collect::<Vec<_>>();
        let parts = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

        let rules = parts[0]
            .iter()
            .map(|&s| OrderingRule::from_str(s).unwrap())
            .collect::<Vec<_>>();

        let updates = parts[1]
            .iter()
            .map(|&s| Update::from_str(s).unwrap())
            .collect::<Vec<_>>();

        (rules, updates)
    }

    fn part_1((rules, updates): &Self::Input) -> SolutionResult<Self::Output> {
        let sum = updates
            .iter()
            .filter(|u| rules.iter().all(|rule| u.satisfies_rule(rule)))
            .map(|upd| upd.midpoint())
            .sum();
        Ok(sum)
    }

    fn part_2((rules, updates): &Self::Input) -> SolutionResult<Self::Output> {
        let mut to_fix = updates
            .iter()
            .filter(|u| rules.iter().any(|rule| !u.satisfies_rule(rule)))
            .map(|u| u.clone())
            .collect::<Vec<_>>();

        for update in &mut to_fix {
            while rules.iter().any(|rule| update.fix_rule(rule)) {}
        }

        let sum = to_fix.iter().map(|upd| upd.midpoint()).sum();
        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::<Day5>(PuzzleDate::new(2024, 5));
}
