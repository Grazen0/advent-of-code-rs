use std::collections::{HashMap, HashSet};

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    fn rotated_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn inverted(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    fn rotated_counter_clockwise(&self) -> Self {
        self.inverted().rotated_clockwise()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0 + 1, self.1),
            Direction::Left => Self(self.0, self.1 - 1),
        }
    }

    fn is_within<T>(&self, grid: &Vec<Vec<T>>) -> bool {
        (0..(grid.len() as isize)).contains(&self.0)
            && (0..(grid[0].len() as isize)).contains(&self.1)
    }
}

struct Day12;

impl PuzzleSolution for Day12 {
    type Input = Vec<Vec<char>>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_1(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;
        let mut visited = HashSet::new();

        for root_i in 0..grid.len() {
            for root_j in 0..grid[0].len() {
                let root_pos = Position(root_i as isize, root_j as isize);

                if visited.contains(&root_pos) {
                    continue;
                }

                visited.insert(root_pos);

                let mut area = 0;
                let mut perimeter = 0;
                let mut stack = vec![root_pos];

                while !stack.is_empty() {
                    let pos = stack.pop().unwrap();

                    area += 1;

                    for direction in DIRECTIONS {
                        let npos = pos.step(direction);

                        if npos.is_within(&grid)
                            && grid[pos.0 as usize][pos.1 as usize]
                                == grid[npos.0 as usize][npos.1 as usize]
                        {
                            if visited.insert(npos) {
                                stack.push(npos);
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                sum += area * perimeter;
            }
        }

        Ok(sum)
    }

    fn part_2(grid: &Self::Input) -> SolutionResult<Self::Output> {
        let mut sum = 0;
        let mut visited = HashSet::new();
        let mut traversed_directions = HashMap::<Position, HashSet<Direction>>::new();

        for root_i in 0..grid.len() {
            for root_j in 0..grid[0].len() {
                let root_pos = Position(root_i as isize, root_j as isize);

                if visited.contains(&root_pos) {
                    continue;
                }

                visited.insert(root_pos);

                let mut area = 0;
                let mut sides = 0;
                let mut stack = vec![root_pos];

                while !stack.is_empty() {
                    let pos = stack.pop().unwrap();
                    let pos_ch = grid[pos.0 as usize][pos.1 as usize];

                    area += 1;

                    for direction in DIRECTIONS {
                        let npos = pos.step(direction);

                        if npos.is_within(&grid) && pos_ch == grid[npos.0 as usize][npos.1 as usize]
                        {
                            if visited.insert(npos) {
                                stack.push(npos);
                            }
                        }

                        let right = direction.rotated_clockwise();
                        let rpos = pos.step(right);

                        if (!npos.is_within(&grid)
                            || grid[npos.0 as usize][npos.1 as usize] != pos_ch)
                            && (!rpos.is_within(&grid)
                                || grid[rpos.0 as usize][rpos.1 as usize] != pos_ch)
                        {
                            // Convex corner
                            sides += 1;
                        }

                        if npos.is_within(&grid)
                            && grid[npos.0 as usize][npos.1 as usize] == pos_ch
                            && rpos.is_within(&grid)
                            && grid[rpos.0 as usize][rpos.1 as usize] == pos_ch
                        {
                            // Possible concave corner
                            let center = npos.step(right);

                            if grid[center.0 as usize][center.1 as usize] != pos_ch {
                                // Concave corner
                                sides += 1;
                            }
                        }
                    }
                }

                sum += area * sides;
            }
        }

        println!();
        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day12>(2024, 12);
}
