use std::{collections::HashSet, fmt::Display};

use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::{Direction, DIRECTIONS},
};

struct Day18;

impl PuzzleSolution for Day18 {
    type Input = Vec<(usize, usize)>;
    type Output = Box<dyn Display>;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .trim()
            .lines()
            .map(|line| {
                let (l, r) = line.split_once(",").unwrap();
                (l.parse().unwrap(), r.parse().unwrap())
            })
            .collect()
    }

    fn part_1(walls: &Self::Input) -> SolutionResult<Self::Output> {
        const FALLEN_BYTES: usize = 1024;
        const GRID_SIZE: usize = 71;

        let walls = &walls[..FALLEN_BYTES];

        let mut queue = vec![((0, 0), 0)];
        let mut visited = HashSet::new();

        while let Some((pos, distance)) = queue.pop() {
            for direction in DIRECTIONS {
                if pos == (GRID_SIZE - 1, GRID_SIZE - 1) {
                    return Ok(Box::new(distance));
                }

                let bound_check = match direction {
                    Direction::Up => pos.0 > 0,
                    Direction::Right => pos.1 < GRID_SIZE - 1,
                    Direction::Down => pos.0 < GRID_SIZE - 1,
                    Direction::Left => pos.1 > 0,
                };

                if !bound_check {
                    continue;
                }

                let new_pos = match direction {
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0, pos.1 + 1),
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Left => (pos.0, pos.1 - 1),
                };

                if !visited.contains(&new_pos) && !walls.contains(&new_pos) {
                    visited.insert(new_pos);
                    queue.insert(0, (new_pos, distance + 1));
                }
            }
        }

        panic!("End could not be reached");
    }

    // TODO: optimize using a `previous` field for each cell
    fn part_2(walls: &Self::Input) -> SolutionResult<Self::Output> {
        const GRID_SIZE: usize = 71;

        let mut last_history = vec![(0, 0)];

        'main: for i in 1..=walls.len() {
            let walls = &walls[..i];
            let new_wall = walls.last().unwrap();

            let mut end = last_history
                .iter()
                .position(|pos| pos == new_wall)
                .unwrap_or(last_history.len());

            while end > 0 {
                let partial_history = &last_history[..end];
                let mut queue = vec![Vec::from(partial_history)];
                let mut visited: HashSet<_> = partial_history.iter().cloned().collect();

                while let Some(history) = queue.pop() {
                    for direction in DIRECTIONS {
                        let pos = history.last().unwrap();

                        if *pos == (GRID_SIZE - 1, GRID_SIZE - 1) {
                            last_history = history;
                            continue 'main;
                        }

                        let bound_check = match direction {
                            Direction::Up => pos.0 > 0,
                            Direction::Right => pos.1 < GRID_SIZE - 1,
                            Direction::Down => pos.0 < GRID_SIZE - 1,
                            Direction::Left => pos.1 > 0,
                        };

                        if !bound_check {
                            continue;
                        }

                        let new_pos = match direction {
                            Direction::Up => (pos.0 - 1, pos.1),
                            Direction::Right => (pos.0, pos.1 + 1),
                            Direction::Down => (pos.0 + 1, pos.1),
                            Direction::Left => (pos.0, pos.1 - 1),
                        };

                        if !visited.contains(&new_pos) && !walls.contains(&new_pos) {
                            visited.insert(new_pos);

                            let mut new_history = history.clone();
                            new_history.push(new_pos);
                            queue.insert(0, new_history);
                        }
                    }
                }

                end -= 1;
            }

            return Ok(Box::new(format!("{},{}", new_wall.0, new_wall.1)));
        }

        panic!("The path is always clear");
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day18>(2024, 18);
}
