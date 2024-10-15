use aoc_lib::{PuzzleDate, PuzzleSolution};

struct Day2;

impl PuzzleSolution for Day2 {
    type Input = Vec<String>;

    fn parse_input(raw_input: Vec<String>) -> Self::Input {
        raw_input
    }

    fn part_1(_input: &Self::Input) -> Box<dyn std::fmt::Display> {
        Box::new("hello")
    }
}

fn main() {
    aoc_lib::cli::<Day2>(&PuzzleDate::new(2023, 2)).unwrap();
}
