use std::collections::HashSet;

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

fn trailhead_score(grid: &[Vec<u32>], i: usize, j: usize, seen_tops: &mut HashSet<(usize, usize)>) {
    if grid[i][j] == 9 {
        seen_tops.insert((i, j));
    } else {
        if i > 0 && grid[i - 1][j] == grid[i][j] + 1 {
            trailhead_score(grid, i - 1, j, seen_tops);
        }

        if i < grid.len() - 1 && grid[i + 1][j] == grid[i][j] + 1 {
            trailhead_score(grid, i + 1, j, seen_tops);
        }

        if j > 0 && grid[i][j - 1] == grid[i][j] + 1 {
            trailhead_score(grid, i, j - 1, seen_tops);
        }

        if j < grid[i].len() - 1 && grid[i][j + 1] == grid[i][j] + 1 {
            trailhead_score(grid, i, j + 1, seen_tops);
        }
    }
}

fn trailhead_rating(grid: &[Vec<u32>], i: usize, j: usize) -> usize {
    if grid[i][j] == 9 {
        1
    } else {
        let mut score = 0;

        if i > 0 && grid[i - 1][j] == grid[i][j] + 1 {
            score += trailhead_rating(grid, i - 1, j);
        }

        if i < grid.len() - 1 && grid[i + 1][j] == grid[i][j] + 1 {
            score += trailhead_rating(grid, i + 1, j);
        }

        if j > 0 && grid[i][j - 1] == grid[i][j] + 1 {
            score += trailhead_rating(grid, i, j - 1);
        }
        if j < grid[i].len() - 1 && grid[i][j + 1] == grid[i][j] + 1 {
            score += trailhead_rating(grid, i, j + 1);
        }

        score
    }
}

struct Day10;

impl PuzzleSolution for Day10 {
    type Input = Vec<Vec<u32>>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .trim()
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn part_1(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    let mut seen_tops = HashSet::new();
                    trailhead_score(grid, i, j, &mut seen_tops);
                    sum += seen_tops.len();
                }
            }
        }

        Ok(sum)
    }

    fn part_2(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    sum += trailhead_rating(grid, i, j);
                }
            }
        }

        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day10>(2024, 10);
}
