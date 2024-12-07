use std::collections::{HashMap, HashSet};

use aoc_lib::{PuzzleDate, PuzzleSolution, SolutionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotated_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn backwards(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("invalid guard character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point2D {
    i: usize,
    j: usize,
}

impl Point2D {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.i -= 1,
            Direction::Right => self.j += 1,
            Direction::Down => self.i += 1,
            Direction::Left => self.j -= 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    position: Point2D,
    direction: Direction,
}

struct Day6;

impl PuzzleSolution for Day6 {
    type Input = (Vec<Vec<Cell>>, Guard);
    type Output = usize;

    fn parse_input(input: String) -> Self::Input {
        let mut guard = None;

        let cells = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, ch)| match ch {
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        _ => {
                            guard = Some(Guard {
                                position: Point2D::new(i, j),
                                direction: Direction::from(ch),
                            });
                            Cell::Empty
                        }
                    })
                    .collect()
            })
            .collect();

        (cells, guard.unwrap())
    }

    fn part_1((grid, guard): &Self::Input) -> SolutionResult<Self::Output> {
        let mut guard = guard.clone();
        let mut seen_positions = HashSet::new();

        loop {
            seen_positions.insert(guard.position.clone());

            let bound_condition = match guard.direction.backwards() {
                Direction::Up => guard.position.i == 0,
                Direction::Right => guard.position.j >= grid[0].len() - 1,
                Direction::Down => guard.position.i >= grid.len() - 1,
                Direction::Left => guard.position.j == 0,
            };

            if bound_condition {
                break;
            }

            let mut new_position = guard.position.clone();
            new_position.step(&guard.direction);

            if let Cell::Wall = grid[new_position.i][new_position.j] {
                guard.direction = guard.direction.rotated_clockwise();
            } else {
                guard.position = new_position;
            }
        }

        Ok(seen_positions.len())
    }

    fn part_2((grid, guard): &Self::Input) -> SolutionResult<Self::Output> {
        let mut guard = guard.clone();
        let mut seen_states = HashMap::<Point2D, HashSet<Direction>>::new();
        let mut added_walls = HashSet::new();

        'main: loop {
            let mut tmp_position = guard.position.clone();

            // Go backwards
            loop {
                let bound_condition = match guard.direction.backwards() {
                    Direction::Up => tmp_position.i == 0,
                    Direction::Right => tmp_position.j >= grid[0].len() - 1,
                    Direction::Down => tmp_position.i >= grid.len() - 1,
                    Direction::Left => tmp_position.j == 0,
                };

                if bound_condition {
                    break;
                }

                tmp_position.step(&guard.direction.backwards());

                if let Cell::Wall = grid[tmp_position.i][tmp_position.j] {
                    break;
                } else {
                    let entry = seen_states.entry(tmp_position).or_default();
                    entry.insert(guard.direction);

                    if entry.contains(&guard.direction.backwards().rotated_clockwise()) {
                        let mut wall_position = tmp_position.clone();
                        wall_position.step(&guard.direction.backwards().rotated_clockwise());
                        added_walls.insert(wall_position);
                    }
                }
            }

            // Go forwards
            loop {
                let entry = seen_states.entry(guard.position).or_default();
                entry.insert(guard.direction);

                if entry.contains(&guard.direction.rotated_clockwise()) {
                    let mut wall_position = guard.position.clone();
                    wall_position.step(&guard.direction);
                    added_walls.insert(wall_position);
                }

                let bound_condition = match guard.direction {
                    Direction::Up => guard.position.i == 0,
                    Direction::Right => guard.position.j >= grid[0].len() - 1,
                    Direction::Down => guard.position.i >= grid.len() - 1,
                    Direction::Left => guard.position.j == 0,
                };

                if bound_condition {
                    // End of it
                    break 'main;
                }

                guard.position.step(&guard.direction);

                if let Cell::Wall = grid[guard.position.i][guard.position.j] {
                    guard.position.step(&guard.direction.backwards());
                    guard.direction = guard.direction.rotated_clockwise();
                    break;
                }
            }
        }

        Ok(added_walls.len())
    }
}

fn main() {
    aoc_lib::cli::<Day6>(PuzzleDate::new(2024, 6));
}
