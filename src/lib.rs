pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod util;

use anyhow::{Error, Result};

pub fn run_all() -> Result<(), Error> {
    day01::part1()?;
    day01::part2()?;
    day02::part1()?;
    day02::part2()?;
    day03::part1()?;
    day03::part2()?;
    Ok(())
}
