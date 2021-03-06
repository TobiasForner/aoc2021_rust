#![feature(generic_associated_types)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
    #[clap(short, long, default_value = "25")]
    day: u16,
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let day = opts.day;
    macro_rules! run_day {
        ($mod_name:ident) => {
            return execute_and_time($mod_name::part1, $mod_name::part2, day)
        };
    }

    match day {
        1 => run_day!(day01),
        2 => run_day!(day02),
        3 => run_day!(day03),
        4 => run_day!(day04),
        5 => run_day!(day05),
        6 => run_day!(day06),
        7 => run_day!(day07),
        8 => run_day!(day08),
        9 => run_day!(day09),
        10 => run_day!(day10),
        11 => run_day!(day11),
        12 => run_day!(day12),
        13 => run_day!(day13),
        14 => run_day!(day14),
        15 => run_day!(day15),
        16 => run_day!(day16),
        17 => run_day!(day17),
        18 => run_day!(day18),
        19 => run_day!(day19),
        20 => run_day!(day20),
        21 => run_day!(day21),
        22 => run_day!(day22),
        23 => run_day!(day23),
        24 => run_day!(day24),
        25 => run_day!(day25),
        _ => println!("So far there are no solutions for day: {}", day),
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
