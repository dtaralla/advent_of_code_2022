use anyhow::{Error, Result};
use aoc_core::AdventOfCodeRunnable;
use clap::{arg, value_parser, ArgAction, Command};
use macro_support::{advent_of_code, declare_exercise_modules, get_available_exercises};
use std::thread::sleep;
use std::time;

#[advent_of_code(1, 2022)]
struct CalorieCounting;

declare_exercise_modules!();

fn main() -> Result<()> {
    let matches = Command::new("AdventOfCode Runner")
        .about("Utility to run advent of code implementations")
        .version("v0.1.0")
        .author("David Taralla (@dtaralla on GitHub)")
        .subcommand(Command::new("clearcache").about("Clear the cache of downloaded inputs"))
        .subcommand(Command::new("ls").about("Lists all days that can be run"))
        .subcommand(
            Command::new("run")
                .about("Runs the given exercise")
                .arg(
                    arg!(--id <ID> "The OAUTH session ID (cookie) for adventofcode.com")
                        .required(true),
                )
                .arg(arg!(<YEAR> "Year of the exercise to run").value_parser(value_parser!(u16)))
                .arg(arg!(<DAY> "Day of the exercise to run").value_parser(value_parser!(u8)))
                .arg(
                    arg!(-s --second "Whether to execute the Second part of the exercise")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    let es: Vec<Box<dyn AdventOfCodeRunnable>> = get_available_exercises!();

    if matches.subcommand_matches("clearcache").is_some() {
        std::fs::remove_dir_all(aoc_core::CACHE_DIR)?;
        sleep(time::Duration::from_millis(100));
        std::fs::create_dir(aoc_core::CACHE_DIR)?;
        std::fs::write(format!("{}/.keep", aoc_core::CACHE_DIR), "")?;
        return Ok(());
    }

    if matches.subcommand_matches("ls").is_some() {
        for e in es.iter() {
            println!("{}", e);
        }
        return Ok(());
    }

    let run_cmd = matches.subcommand_matches("run");
    if run_cmd.is_none() {
        return Err(Error::msg("Not a valid subcommand"));
    }

    let run_cmd = run_cmd.unwrap();
    let session_id: &String = run_cmd.get_one("id").unwrap();
    let year: u16 = *run_cmd.get_one("YEAR").unwrap();
    let day: u8 = *run_cmd.get_one("DAY").unwrap();

    let mut selected_ex: Option<&Box<dyn AdventOfCodeRunnable>> = None;
    for ex in es.iter() {
        if ex.matches(day, year) {
            selected_ex = Some(ex);
        }
    }

    if selected_ex.is_none() {
        println!(
            "Exercise of Dec {}, {} is not implemented. Exiting...",
            day, year
        );
        return Ok(());
    }

    let selected_ex = selected_ex.unwrap();
    let result = match run_cmd.get_flag("second") {
        true => {
            let input = selected_ex.get_input(session_id, true)?;
            selected_ex.run2(&input)?
        }
        false => {
            let input = selected_ex.get_input(session_id, false)?;
            selected_ex.run(&input)?
        }
    };

    println!("Result: {}", result);
    Ok(())
}
