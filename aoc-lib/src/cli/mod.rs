use clap::Parser;
use colored::Colorize;
use std::{env, fmt::Display, fs};
use util::BenchResult;

mod input;
mod util;

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
            Self::BadInput => write!(f, "bad input"),
            Self::Other(err) => err.fmt(f),
        }
    }
}

pub trait PuzzleSolution {
    type Input;
    type Output: Display;

    fn parse_input(raw_input: String) -> Self::Input;

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

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    part: Option<u32>,

    #[arg(short, long)]
    visualize: bool,

    #[arg(short, long, default_value = "AOC_SESSION_ID")]
    session_cookie_var: String,
}

fn run_part<F, I, T: Display>(f: F, input: &I, part: u32)
where
    F: FnOnce(&I) -> SolutionResult<T>,
{
    println!(
        "{}",
        format!("{:=^32}", format!(" Part {} ", part))
            .bold()
            .yellow()
    );

    println!("{}", "Running...".bright_black());

    let BenchResult(result, elapsed) = util::bench(|| f(input));

    util::goto_previous_line();

    match result {
        Ok(answer) => println!("{} {}", "Result:".bright_green(), answer),
        Err(e) => println!("{} {}", "Error:".red(), e),
    }

    println!("{}", format!("{:.2?} elapsed", elapsed).white());
}

fn run_solution_unwrapped<S: PuzzleSolution>(
    args: Args,
    date: PuzzleDate,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        format!("Advent of Code {}, day {}", date.year, date.day)
            .bold()
            .bright_blue()
    );

    let raw_input = match args.input {
        Some(filename) => fs::read_to_string(filename)?,
        None => match input::read_cached_input(&date)? {
            Some(input) => input,
            None => {
                println!("{}", "Fetching input...".bright_black());

                let input = input::fetch_input(&date, &env::var(args.session_cookie_var)?)?;
                let filename = input::input_cache_path(&date);
                util::write_dir_safe(filename, &input)?;

                util::goto_previous_line();
                input
            }
        },
    };

    let input = S::parse_input(raw_input);

    if args.visualize {
        S::visualize(&input);
    } else {
        match args.part {
            None => {
                run_part(S::part_1, &input, 1);
                run_part(S::part_2, &input, 2);
            }
            Some(1) => run_part(S::part_1, &input, 1),
            Some(2) => run_part(S::part_2, &input, 2),
            Some(_) => return Err("invalid part selection".into()),
        }
    }

    Ok(())
}

pub fn run_solution<S: PuzzleSolution>(year: u32, day: u32) {
    let args = Args::parse();
    let date = PuzzleDate::new(year, day);

    let result = run_solution_unwrapped::<S>(args, date);

    if let Err(e) = result {
        println!("{} {}", "Error:".red(), e);
    }
}
