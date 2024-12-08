use aoc_lib::cli::{PuzzleSolution, SolutionResult};

#[derive(Debug, Clone)]
struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        let diff_sign = (self.0[1] - self.0[0]).signum();

        self.0.windows(2).all(|window| {
            let diff = window[1] - window[0];
            (1..=3).contains(&diff.abs()) && diff.signum() == diff_sign
        })
    }

    fn is_dampened_safe(&self) -> bool {
        self.is_safe()
            || (0..self.0.len()).any(|i| {
                let mut dampened_clone = self.clone();
                dampened_clone.0.remove(i);
                dampened_clone.is_safe()
            })
    }
}

struct Day2;

impl PuzzleSolution for Day2 {
    type Input = Vec<Report>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .lines()
            .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .map(Report)
            .collect()
    }

    fn part_1(reports: &Self::Input) -> SolutionResult<Self::Output> {
        let count = reports.iter().filter(|r| r.is_safe()).count();
        Ok(count)
    }

    fn part_2(reports: &Self::Input) -> SolutionResult<Self::Output> {
        let count = reports.iter().filter(|r| r.is_dampened_safe()).count();
        Ok(count)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day2>(2024, 2);
}
