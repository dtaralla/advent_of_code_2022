use anyhow::Result;
use reqwest::{blocking, header};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

pub static CACHE_DIR: &str = "downloaded_inputs";

pub trait AdventOfCodeRunnable
where
    Self: Display,
{
    fn matches(&self, day: u8, year: u16) -> bool;
    fn get_input(&self, oauth_session_id: &str, is_second: bool) -> Result<String>;
    fn run(&self, input: &str) -> Result<String>;
    fn run2(&self, input: &str) -> Result<String>;
}

pub fn get_input(session_id: &str, day: u8, year: u16, _is_second: bool) -> Result<String> {
    let local_cached_file = PathBuf::from_str(&format!("{}/{}_{}.txt", CACHE_DIR, year, day))?;

    let mut input = String::new();
    if local_cached_file.is_file() {
        // Read cached input
        OpenOptions::new()
            .read(true)
            .open(local_cached_file)?
            .read_to_string(&mut input)?;
    } else {
        // Download the input
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!("session={}", session_id))?,
        );
        let c = blocking::ClientBuilder::default()
            .default_headers(headers)
            .build()?;
        input = c
            .execute(
                c.get(format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day
                ))
                .build()?,
            )?
            .text()?;

        // Cache it
        std::fs::write(local_cached_file, &input)?;
    }

    Ok(input)
}
