mod cli;
mod input;
mod util;

pub use cli::cli;
use std::fmt::Display;

pub type SolutionResult<T> = Result<T, SolutionError>;

#[derive(Debug)]
pub enum SolutionError {
    Unimplemented,
    BadInput,
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for SolutionError {}

impl Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unimplemented => write!(f, "not yet implemented"),
            Self::BadInput => write!(f, "bad input solution"),
            Self::Other(err) => err.fmt(f),
        }
    }
}

pub trait PuzzleSolution {
    type Input;
    type Output: Display;

    fn parse_input(raw_input: Vec<String>) -> Self::Input;

    fn part_1(_input: &Self::Input) -> SolutionResult<Self::Output> {
        Err(SolutionError::Unimplemented)
    }

    fn part_2(_input: &Self::Input) -> SolutionResult<Self::Output> {
        Err(SolutionError::Unimplemented)
    }

    fn visualize(_input: &Self::Input) {
        unimplemented!("unimplemented visualization");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleDate {
    year: u32,
    day: u32,
}

impl PuzzleDate {
    pub fn new(year: u32, day: u32) -> Self {
        Self { year, day }
    }
}
