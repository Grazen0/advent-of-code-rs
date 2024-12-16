use std::{
    fs::{self, File},
    io::Write,
    ops::Range,
};

use aoc_lib::{
    cli::{PuzzleSolution, SolutionResult},
    helper::structs::Vector2D,
};

fn parse_vector(s: &str) -> Vector2D<i32> {
    let (x, y) = s.split_once(",").unwrap();
    Vector2D {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: Vector2D<i32>,
    velocity: Vector2D<i32>,
}

fn count_quadrant(robots: &[Robot], x_range: Range<i32>, y_range: Range<i32>) -> usize {
    robots
        .iter()
        .filter(|robot| x_range.contains(&robot.position.x) && y_range.contains(&robot.position.y))
        .count()
}

struct Day14;

impl PuzzleSolution for Day14 {
    type Input = Vec<Robot>;
    type Output = usize;

    fn parse_input(raw_input: String) -> Self::Input {
        raw_input
            .lines()
            .map(|line| {
                let (p, v) = line.split_once(" ").unwrap();

                Robot {
                    position: parse_vector(p.split_once("=").unwrap().1),
                    velocity: parse_vector(v.split_once("=").unwrap().1),
                }
            })
            .collect::<Vec<_>>()
    }

    fn part_1(robots: &Self::Input) -> SolutionResult<Self::Output> {
        const WIDTH: i32 = 101;
        const HEIGHT: i32 = 103;
        const STEPS: i32 = 100;

        let mut robots = robots.clone();

        for robot in &mut robots {
            robot.position += robot.velocity * STEPS;
            robot.position.x = robot.position.x.rem_euclid(WIDTH);
            robot.position.y = robot.position.y.rem_euclid(HEIGHT);
        }

        let top_left = count_quadrant(&robots, 0..(WIDTH / 2), 0..(HEIGHT / 2));
        let top_right = count_quadrant(&robots, (WIDTH / 2 + 1)..WIDTH, 0..(HEIGHT / 2));
        let bottom_left = count_quadrant(&robots, 0..(WIDTH / 2), (HEIGHT / 2 + 1)..HEIGHT);
        let bottom_right =
            count_quadrant(&robots, (WIDTH / 2 + 1)..WIDTH, (HEIGHT / 2 + 1)..HEIGHT);

        Ok(top_left * top_right * bottom_right * bottom_left)
    }

    fn part_2(robots: &Self::Input) -> SolutionResult<Self::Output> {
        const WIDTH: i32 = 101;
        const HEIGHT: i32 = 103;

        let mut robots = robots.clone();

        let _ = fs::create_dir("robots");

        for step in 1..=(WIDTH * HEIGHT + 10) {
            for robot in &mut robots {
                robot.position += robot.velocity;
                robot.position.x = robot.position.x.rem_euclid(WIDTH);
                robot.position.y = robot.position.y.rem_euclid(HEIGHT);
            }

            let mut file = File::options()
                .write(true)
                .create(true)
                .open(format!("robots/{}.txt", step))
                .unwrap();

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let pos = Vector2D::new(x, y);

                    if robots.iter().any(|robot| robot.position == pos) {
                        file.write(b"##").unwrap();
                    } else {
                        file.write(b"  ").unwrap();
                    }
                }

                file.write(b"\n").unwrap();
            }
        }

        Ok(42)
    }
}

fn main() {
    aoc_lib::cli::run_solution::<Day14>(2024, 14);
}
