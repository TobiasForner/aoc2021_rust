mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

use anyhow::{Error, Result};

use clap::Parser;

/// Rust solutions to AoC 2021
#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    /// day of aoc 2021 to execute
    #[clap(short, long, default_value = "7")]
    day: u16,
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    match opts.day {
        1 => {
            day01::part1()?;
            day01::part2()?;
        }
        2 => {
            day02::part1()?;
            day02::part2()?;
        }
        3 => {
            day03::part1()?;
            day03::part2()?;
        }
        4 => {
            day04::part1()?;
            day04::part2()?;
        }
        5 => {
            day05::part1()?;
            day05::part2()?;
        }
        6 => {
            day06::part1()?;
            day06::part2()?;
        }
        7 => {
            day07::part1()?;
            day07::part2()?;
        }
        _ => {
            println!("So far there are no solutions for day: {}", opts.day);
        }
    }
    Ok(())
}
