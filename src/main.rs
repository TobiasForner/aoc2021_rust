mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

use anyhow::{Error, Result};
use std::fmt::Display;
use std::time::Instant;

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
    macro_rules! run_day {
        ($mod_name:ident, $day:expr) => {
            return execute_and_time($mod_name::part1, $mod_name::part2, $day)
        };
    }
    let opts: Opts = Opts::parse();
    let day = opts.day;

    match day {
        1 => run_day!(day01, day),
        2 => run_day!(day02, day),
        3 => run_day!(day03, day),
        4 => run_day!(day04, day),
        5 => run_day!(day05, day),
        6 => run_day!(day06, day),
        7 => run_day!(day07, day),
        _ => println!("So far there are no solutions for day: {}", opts.day),
    }
    Ok(())
}

fn execute_and_time<D1: Display, D2: Display, F1, F2>(part1: F1, part2: F2, day: u16) -> Result<()>
where
    F1: Fn(&str) -> Result<D1>,
    F2: Fn(&str) -> Result<D2>,
{
    let input_file = &format!("inputs/day{:0>2}.txt", day);
    let mut instant = Instant::now();
    println!(
        "Day {} Part 1: {} ({}ms)",
        day,
        part1(input_file)?,
        instant.elapsed().as_millis()
    );
    instant = Instant::now();
    println!(
        "Day {} Part 2: {} ({}ms)",
        day,
        part2(input_file)?,
        instant.elapsed().as_millis()
    );
    Ok(())
}
