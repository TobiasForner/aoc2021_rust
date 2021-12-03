mod day01;
mod day02;
mod day03;
mod util;

use anyhow::{Error, Result};

use clap::Parser;

/// Rust solutions to AoC 2021
#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    /// day of aoc 2021 to execute
    #[clap(short, long, default_value = "3")]
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
        _ => {
            println!("So far there are no solutions for day: {}", opts.day);
        }
    }
    Ok(())
}
