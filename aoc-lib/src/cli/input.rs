use crate::cli::PuzzleDate;
use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};

pub fn fetch_input(date: &PuzzleDate, session_id: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            date.year, date.day
        ))
        .header(reqwest::header::COOKIE, format!("session={}", session_id))
        .send()
        .and_then(|res| res.error_for_status())
        .and_then(|res| res.text())
}

pub fn input_cache_path(date: &PuzzleDate) -> PathBuf {
    let mut dir = ProjectDirs::from("me", "grazen", "aoc-lib")
        .expect("could not determine project directories")
        .cache_dir()
        .to_path_buf();

    dir.push(format!("{:02}-{:02}.txt", date.year, date.day));
    dir
}

pub fn read_cached_input(date: &PuzzleDate) -> io::Result<Option<String>> {
    let path = input_cache_path(date);
    if path.try_exists()? {
        fs::read_to_string(path).map(Some)
    } else {
        Ok(None)
    }
}
