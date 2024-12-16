use std::{collections::HashSet, hash::Hash};

use aoc_lib::cli::{PuzzleSolution, SolutionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Day15Input {
    initial_position: (usize, usize),
    stones: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    steps: Vec<Direction>,
}

struct Day15;

impl PuzzleSolution for Day15 {
    type Input = Day15Input;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines = raw_input.lines().collect::<Vec<_>>();

        let parts = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
        let grid = parts[0];
        let steps_str = parts[1];

        let mut initial_position: Option<(usize, usize)> = None;
        let mut stones = HashSet::new();
        let mut walls = HashSet::new();

        for (i, line) in grid.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let pos = (i, j);

                match ch {
                    'O' => {
                        stones.insert(pos);
                    }
                    '@' => {
                        initial_position = Some(pos);
                    }
                    '#' => {
                        walls.insert(pos);
                    }
                    _ => {}
                }
            }
        }

        let steps = steps_str
            .join("")
            .chars()
            .map(|ch| match ch {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => panic!("invalid step character"),
            })
            .collect::<Vec<_>>();

        Day15Input {
            initial_position: initial_position.unwrap(),
            steps,
            walls,
            stones,
        }
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let walls = input.walls.clone();
        let mut pos = input.initial_position;
        let mut stones = input.stones.clone();

        for step in &input.steps {
            let mut next_pos = pos;

            match step {
                Direction::Up => next_pos.0 -= 1,
                Direction::Right => next_pos.1 += 1,
                Direction::Down => next_pos.0 += 1,
                Direction::Left => next_pos.1 -= 1,
            }

            if walls.contains(&next_pos) {
                continue;
            }

            if stones.contains(&next_pos) {
                // Rock found
                let mut swap_pos = next_pos;

                let do_swap = loop {
                    match step {
                        Direction::Up => swap_pos.0 -= 1,
                        Direction::Right => swap_pos.1 += 1,
                        Direction::Down => swap_pos.0 += 1,
                        Direction::Left => swap_pos.1 -= 1,
                    }

                    if walls.contains(&swap_pos) {
                        break false;
                    }

                    if !stones.contains(&swap_pos) {
                        break true;
                    }
                };

                if do_swap {
                    stones.insert(swap_pos);
                    stones.remove(&next_pos);
                    pos = next_pos;
                }
            } else {
                pos = next_pos;
            }
        }

        let sum = stones.iter().map(|stone| 100 * stone.0 + stone.1).sum();
        Ok(sum)
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut walls = HashSet::new();

        for &wall in &input.walls {
            walls.insert((wall.0, 2 * wall.1));
            walls.insert((wall.0, 2 * wall.1 + 1));
        }

        let mut pos = (input.initial_position.0, 2 * input.initial_position.1);
        let mut stones_left = input
            .stones
            .iter()
            .map(|&stone| (stone.0, 2 * stone.1))
            .collect::<HashSet<_>>();

        for step in &input.steps {
            let next_pos = match step {
                Direction::Up => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0, pos.1 + 1),
                Direction::Down => (pos.0 + 1, pos.1),
                Direction::Left => (pos.0, pos.1 - 1),
            };

            if try_clearing(next_pos, &mut stones_left, &walls, *step, true) {
                pos = next_pos;
            }
        }

        let sum = stones_left
            .iter()
            .map(|stone| 100 * stone.0 + stone.1)
            .sum();
        Ok(sum)
    }
}

fn try_clearing(
    pos: (usize, usize),
    stones_left: &mut HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
    direction: Direction,
    mutate: bool,
) -> bool {
    if walls.contains(&pos) {
        return false;
    }

    let Some(stone) = stones_left
        .get(&pos)
        .or_else(|| stones_left.get(&(pos.0, pos.1 - 1)))
        .map(|s| s.clone())
    else {
        return true;
    };

    // So... there's a stone

    match direction {
        Direction::Left => {
            if try_clearing((stone.0, stone.1 - 1), stones_left, walls, direction, true) {
                if mutate {
                    stones_left.remove(&stone);
                    stones_left.insert((stone.0, stone.1 - 1));
                }
                true
            } else {
                false
            }
        }
        Direction::Right => {
            if try_clearing((stone.0, stone.1 + 2), stones_left, walls, direction, true) {
                if mutate {
                    stones_left.remove(&stone);
                    stones_left.insert((stone.0, stone.1 + 1));
                }
                true
            } else {
                false
            }
        }
        Direction::Up => {
            if try_clearing((stone.0 - 1, stone.1), stones_left, walls, direction, false)
                && try_clearing(
                    (stone.0 - 1, stone.1 + 1),
                    stones_left,
                    walls,
                    direction,
                    false,
                )
            {
                if mutate {
                    try_clearing((stone.0 - 1, stone.1), stones_left, walls, direction, true);
                    try_clearing(
                        (stone.0 - 1, stone.1 + 1),
                        stones_left,
                        walls,
                        direction,
                        true,
                    );

                    stones_left.remove(&stone);
                    stones_left.insert((stone.0 - 1, stone.1));
                }
                true
            } else {
                false
            }
        }
        Direction::Down => {
            if try_clearing((stone.0 + 1, stone.1), stones_left, walls, direction, false)
                && try_clearing(
                    (stone.0 + 1, stone.1 + 1),
                    stones_left,
                    walls,
                    direction,
                    false,
                )
            {
                if mutate {
                    try_clearing((stone.0 + 1, stone.1), stones_left, walls, direction, true);
                    try_clearing(
                        (stone.0 + 1, stone.1 + 1),
                        stones_left,
                        walls,
                        direction,
                        true,
                    );

                    stones_left.remove(&stone);
                    stones_left.insert((stone.0 + 1, stone.1));
                }
                true
            } else {
                false
            }
        }
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day15>(2024, 15);
}
