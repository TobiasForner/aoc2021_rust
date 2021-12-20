use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn parse_input(path: &str) -> Result<(i32, i32, i32, i32)> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let mut line = lines.next().unwrap()?;
    line = line.replace("target area: x=", "").replace(", y=", "..");
    let bounds: Vec<i32> = line
        .split("..")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;
    Ok((bounds[0], bounds[1], bounds[2], bounds[3]))
}

pub fn part1(path: &str) -> Result<i32> {
    Ok(0)
}

pub fn part2(path: &str) -> Result<i32> {
    let result = 0;

    Ok(result)
}
