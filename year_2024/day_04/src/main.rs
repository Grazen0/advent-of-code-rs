use aoc_lib::cli::{PuzzleSolution, SolutionResult};

const SEARCH: [char; 4] = ['X', 'M', 'A', 'S'];

fn right_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    grid[start_i][start_j..].starts_with(search)
}

fn left_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    if start_j < search.len() - 1 {
        return false;
    }

    for j in 0..search.len() {
        if !grid[start_i]
            .get(start_j - j)
            .is_some_and(|&ch| ch == search[j])
        {
            return false;
        }
    }

    true
}

fn up_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    let upper_segment = grid[..=start_i]
        .iter()
        .map(|row| row[start_j])
        .collect::<Vec<_>>();

    upper_segment.ends_with(search)
}

fn down_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    if start_i < search.len() - 1 {
        return false;
    }

    for i in 0..search.len() {
        if !grid
            .get(start_i - i)
            .is_some_and(|row| row[start_j] == search[i])
        {
            return false;
        }
    }

    true
}

fn down_right_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    for k in 0..search.len() {
        if !grid
            .get(start_i + k)
            .is_some_and(|row| row.get(start_j + k).is_some_and(|&ch| ch == search[k]))
        {
            return false;
        }
    }

    true
}
fn down_left_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    if start_j < search.len() - 1 {
        return false;
    }

    for k in 0..search.len() {
        if !grid
            .get(start_i + k)
            .is_some_and(|row| row.get(start_j - k).is_some_and(|&ch| ch == search[k]))
        {
            return false;
        }
    }

    true
}
fn up_right_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    if start_i < search.len() - 1 {
        return false;
    }

    for k in 0..search.len() {
        if !grid
            .get(start_i - k)
            .is_some_and(|row| row.get(start_j + k).is_some_and(|&ch| ch == search[k]))
        {
            return false;
        }
    }

    true
}
fn up_left_search(grid: &[Vec<char>], search: &[char], start_i: usize, start_j: usize) -> bool {
    if start_i < search.len() - 1 || start_j < search.len() - 1 {
        return false;
    }

    for k in 0..search.len() {
        if !grid
            .get(start_i - k)
            .is_some_and(|row| row.get(start_j - k).is_some_and(|&ch| ch == search[k]))
        {
            return false;
        }
    }

    true
}

struct Day4;

impl PuzzleSolution for Day4 {
    type Input = Vec<Vec<char>>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_1(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if right_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if left_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if down_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if up_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if down_right_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if down_left_search(grid, &SEARCH, i, j) {
                    count += 1;
                }

                if up_right_search(grid, &SEARCH, i, j) {
                    count += 1;
                }
                if up_left_search(grid, &SEARCH, i, j) {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    fn part_2(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut count = 0;

        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[i].len() - 1) {
                if grid[i][j] != 'A' {
                    continue;
                }

                let ul = grid[i - 1][j - 1];
                let ur = grid[i - 1][j + 1];
                let dr = grid[i + 1][j + 1];
                let dl = grid[i + 1][j - 1];

                if ((ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M'))
                    && ((ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M'))
                {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day4>(2024, 4);
}
