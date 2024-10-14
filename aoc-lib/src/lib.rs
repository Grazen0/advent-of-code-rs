use clap::Parser;
use std::{fmt::Display, fs, path::Path};

pub trait PuzzleSolution {
    type Input;

    fn parse_input(raw_input: Vec<String>) -> Self::Input;

    fn part_1(_input: &Self::Input) -> Box<dyn Display> {
        todo!("unimplemented part 1");
    }

    fn part_2(_input: &Self::Input) -> Box<dyn Display> {
        todo!("unimplemented part 2")
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

// const SID: &'static str = "53616c7465645f5f155c22c9294728344416e6cfb1ec95637c01d2163082a95866dfaee8bb7fde010daf05a51f3342aec26250d9b33987b2985f2c296ddbc774";

pub fn fetch_input(date: &PuzzleDate, session_id: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            date.year, date.day
        ))
        .header(reqwest::header::COOKIE, format!("session={}", session_id))
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text())
}

fn get_input_for_date(
    date: &PuzzleDate,
    session_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let path_str = format!("./.cache/{:04}-{:02}.txt", date.year, date.day);
    let path = Path::new(&path_str);

    if path.try_exists()? {
        return Ok(fs::read_to_string(path_str)?);
    }

    Ok(fetch_input(date, session_id)?)
}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    part: Option<u32>,

    #[arg(short, long)]
    visualize: bool,
}

pub fn run_solution<S: PuzzleSolution>(
    date: &PuzzleDate,
) -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let raw_input = match args.input {
        None => get_input_for_date(date, "")?,
        Some(filename) => fs::read_to_string(filename)?,
    };

    let input = S::parse_input(raw_input.lines().map(|s| s.to_string()).collect());

    Ok(())
}
