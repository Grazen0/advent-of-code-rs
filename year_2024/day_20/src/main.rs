use std::collections::{HashMap, HashSet};

use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::{Direction, DIRECTIONS},
};

struct Day20;

#[derive(Debug, Clone)]
struct Day20Input {
    walls: HashSet<(usize, usize)>,
    grid_size: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
}

fn do_thing(input: &Day20Input, max_cheat: isize) -> HashMap<usize, usize> {
    let Day20Input {
        start,
        end,
        walls,
        grid_size,
    } = input;

    let mut times = HashMap::new();
    times.insert(start.clone(), 0);

    let mut pos = start.clone();

    while pos != *end {
        for direction in DIRECTIONS {
            let bound_check = match direction {
                Direction::Up => pos.0 > 0,
                Direction::Right => pos.1 < grid_size.1 - 1,
                Direction::Down => pos.0 < grid_size.0 - 1,
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

            if !times.contains_key(&new_pos) && !walls.contains(&new_pos) {
                let current_time = times[&pos];
                times.insert(new_pos, current_time + 1);
                pos = new_pos;
                break;
            }
        }
    }

    let mut saves = HashMap::<usize, usize>::new();

    for (cheat_start, &start_time) in &times {
        for i in -max_cheat..=max_cheat {
            let j_span = max_cheat - i.abs();

            for j in -j_span..=j_span {
                if cheat_start.0 as isize + i < 0 || cheat_start.1 as isize + j < 0 {
                    continue;
                }

                let cheat_end = (
                    (cheat_start.0 as isize + i) as usize,
                    (cheat_start.1 as isize + j) as usize,
                );
                let time_spent = i.abs() + j.abs();

                if let Some(&end_time) = times.get(&(cheat_end)) {
                    if end_time > start_time + time_spent {
                        let saved_time = end_time - start_time - time_spent;
                        let entry = saves.entry(saved_time as usize).or_default();
                        *entry += 1;
                    }
                }
            }
        }
    }

    saves
}

impl PuzzleSolution for Day20 {
    type Input = Day20Input;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        let lines: Vec<_> = raw_input.lines().collect();

        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();

        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let pos = (i, j);
                match ch {
                    '#' => {
                        walls.insert(pos);
                    }
                    'S' => start = Some(pos),
                    'E' => end = Some(pos),
                    _ => {}
                }
            }
        }

        Day20Input {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
            grid_size: (lines.len(), lines[0].len()),
        }
    }

    fn part_1(input: &Self::Input) -> SolutionResult<Self::Output> {
        let saves = do_thing(input, 2);
        let sum = saves
            .iter()
            .filter(|(&time, _)| time >= 100)
            .map(|(_, freq)| freq)
            .sum();

        Ok(sum)
    }

    fn part_2(input: &Self::Input) -> SolutionResult<Self::Output> {
        let saves = do_thing(input, 20);
        let sum = saves
            .iter()
            .filter(|(&time, _)| time >= 100)
            .map(|(_, freq)| freq)
            .sum();

        Ok(sum)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day20>(2024, 20);
}
