use std::collections::{HashMap, HashSet};

use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::{Direction, MinPriorityQueue},
};

type Point = (usize, usize);

struct Day16;

#[derive(Debug, Clone)]
struct Day16Input {
    start: Point,
    end: Point,
    walls: HashSet<Point>,
}

impl PuzzleSolution for Day16 {
    type Input = Day16Input;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();

        for (i, line) in raw_input.trim().lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let pos = (i, j);

                match ch {
                    '#' => {
                        walls.insert(pos);
                    }
                    'S' => {
                        start = Some(pos);
                    }
                    'E' => {
                        end = Some(pos);
                    }
                    '.' => {}
                    _ => panic!("invalid input character"),
                }
            }
        }

        Day16Input {
            start: start.unwrap(),
            end: end.unwrap(),
            walls,
        }
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let mut visited = HashSet::new();
        let mut best_score = None;

        let mut queue = MinPriorityQueue::<(Point, Direction), usize>::new();
        queue.insert((input.start, Direction::Right), 0);

        while let Some(((pos, direction), score)) = queue.pop() {
            visited.insert(pos);

            if pos == input.end {
                if best_score.is_none_or(|bs| score < bs) {
                    best_score = Some(score);
                }
                continue;
            }

            let possible_directions = [
                direction,
                direction.rotated_clockwise(),
                direction.rotated_counter_clockwise(),
            ];

            for new_direction in possible_directions {
                let new_pos = match new_direction {
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0, pos.1 + 1),
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Left => (pos.0, pos.1 - 1),
                };

                if visited.contains(&new_pos) || input.walls.contains(&new_pos) {
                    continue;
                }

                let new_score = if direction == new_direction {
                    score + 1
                } else {
                    score + 1001
                };

                let queue_item = (new_pos, new_direction);

                if let Some(&existing_score) = queue.get_priority_of(&queue_item) {
                    if new_score < existing_score {
                        queue.update_priority(&queue_item, new_score);
                    }
                } else {
                    queue.insert(queue_item, new_score);
                }
            }
        }

        Ok(best_score.unwrap())
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        // TODO: optimize
        let mut visited_data = HashMap::<(Point, Direction), usize>::new();

        let mut queue = MinPriorityQueue::<(Point, Direction), usize>::new();
        queue.insert((input.start, Direction::Right), 0);

        let mut best_score = None;

        while let Some(((pos, direction), score)) = queue.pop() {
            visited_data.insert((pos, direction), score);

            if pos == input.end {
                if best_score.is_none_or(|bs| score < bs) {
                    best_score = Some(score);
                }
                continue;
            }

            let possible_directions = [
                direction,
                direction.rotated_clockwise(),
                direction.rotated_counter_clockwise(),
            ];

            for new_direction in possible_directions {
                let new_pos = match new_direction {
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0, pos.1 + 1),
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Left => (pos.0, pos.1 - 1),
                };

                if visited_data.contains_key(&(new_pos, new_direction))
                    || input.walls.contains(&new_pos)
                {
                    continue;
                }

                let new_score = if direction == new_direction {
                    score + 1
                } else {
                    score + 1001
                };

                let queue_item = (new_pos, new_direction);

                if let Some(&existing_score) = queue.get_priority_of(&queue_item) {
                    if new_score < existing_score {
                        queue.update_priority(&queue_item, new_score);
                    }
                } else {
                    queue.insert(queue_item, new_score);
                }
            }
        }

        let best_score = best_score.unwrap();

        let mut fi = HashSet::new();

        for ((initial_pos, initial_direction), score_at_pos) in visited_data {
            let mut visited = HashSet::new();
            let mut queue = MinPriorityQueue::<(Point, Direction), usize>::new();
            queue.insert((initial_pos, initial_direction), score_at_pos);

            let mut this_best_score = None;

            while let Some(((pos, direction), score)) = queue.pop() {
                visited.insert((pos, direction));

                if pos == input.end {
                    if this_best_score.is_none_or(|bs| score < bs) {
                        this_best_score = Some(score);
                    }
                    continue;
                }

                let possible_directions = [
                    direction,
                    direction.rotated_clockwise(),
                    direction.rotated_counter_clockwise(),
                ];

                for new_direction in possible_directions {
                    let new_pos = match new_direction {
                        Direction::Up => (pos.0 - 1, pos.1),
                        Direction::Right => (pos.0, pos.1 + 1),
                        Direction::Down => (pos.0 + 1, pos.1),
                        Direction::Left => (pos.0, pos.1 - 1),
                    };

                    if visited.contains(&(new_pos, new_direction)) || input.walls.contains(&new_pos)
                    {
                        continue;
                    }

                    let new_score = if direction == new_direction {
                        score + 1
                    } else {
                        score + 1001
                    };

                    let queue_item = (new_pos, new_direction);

                    if let Some(&existing_score) = queue.get_priority_of(&queue_item) {
                        if new_score < existing_score {
                            queue.update_priority(&queue_item, new_score);
                        }
                    } else {
                        queue.insert(queue_item, new_score);
                    }
                }
            }

            if this_best_score.is_some_and(|sc| sc == best_score) {
                fi.insert(initial_pos);
            }
        }

        Ok(fi.len())
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day16>(2024, 16);
}
