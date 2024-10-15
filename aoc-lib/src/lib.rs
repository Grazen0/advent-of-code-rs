use clap::Parser;
use std::{
    env,
    ffi::OsStr,
    fmt::Display,
    fs, io,
    path::Path,
    time::{Duration, Instant},
};

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

    pub fn to_date(&self) {
        todo!();
    }
}

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

fn read_cached_input(date: &PuzzleDate) -> io::Result<Option<String>> {
    let path_str = format!("./.cache/{:04}-{:02}.txt", date.year, date.day);
    let path = Path::new(&path_str);

    if path.try_exists()? {
        fs::read_to_string(path).map(Some)
    } else {
        Ok(None)
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
}

fn bench<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let now = Instant::now();
    let result = f();
    (result, now.elapsed())
}

fn run_part<I, F: FnOnce(&I) -> Box<dyn Display>>(f: F, input: &I) {
    let (result, elapsed) = bench(|| f(input));

    println!("Result: {}", result);
    println!("Elapsed: {:.2?}", elapsed);
}

fn write_dir_safe<P: AsRef<OsStr> + Sized, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    let path = Path::new(&path);
    fs::create_dir_all(path.parent().expect("path does not have a parent"))?;
    fs::write(path, contents)
}

pub fn cli<S: PuzzleSolution>(date: PuzzleDate) {
    let args = Args::parse();
    run_solution::<S>(args, date).unwrap_or_else(|e| eprintln!("Error: {}", e));
}

fn run_solution<S: PuzzleSolution>(
    args: Args,
    date: PuzzleDate,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = match args.input {
        Some(filename) => fs::read_to_string(filename)?,
        None => match read_cached_input(&date)? {
            Some(input) => input,
            None => {
                let input = fetch_input(&date, &env::var("AOC_SESSION_ID")?)?;
                fs::write("", &input)?;
                input
            }
        },
    };

    let input = S::parse_input(raw_input.lines().map(String::from).collect());

    if args.visualize {
        S::visualize(&input);
    } else {
        match args.part {
            None => {
                run_part(S::part_1, &input);
                run_part(S::part_2, &input);
            }
            Some(1) => run_part(S::part_1, &input),
            Some(2) => run_part(S::part_2, &input),
            Some(_) => todo!("add result for invalid part"),
        }
    }

    Ok(())
}
