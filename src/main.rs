mod day01;
mod day02;
mod file_util;

use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    day01::part1()?;
    day01::part2()?;
    day02::part1()?;
    day02::part2()?;
    Ok(())
}
