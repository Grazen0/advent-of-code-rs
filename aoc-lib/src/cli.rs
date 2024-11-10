use crate::{
    input,
    util::{self, BenchResult},
    PuzzleDate, PuzzleSolution, SolutionResult,
};
use clap::Parser;
use colored::Colorize;
use std::{env, fs};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    part: Option<u32>,

    #[arg(short, long)]
    visualize: bool,
}

fn run_part<F, I>(f: F, input: &I, part: u32)
where
    F: FnOnce(&I) -> SolutionResult,
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

fn cli_result<S: PuzzleSolution>(
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

                let input = input::fetch_input(&date, &env::var("AOC_SESSION_ID")?)?;
                let filename = input::input_cache_path(&date);
                util::write_dir_safe(filename, &input)?;

                util::goto_previous_line();
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

pub fn cli<S: PuzzleSolution>(date: PuzzleDate) {
    let args = Args::parse();
    cli_result::<S>(args, date).unwrap_or_else(|e| println!("{} {}", "Error:".red(), e));
}
