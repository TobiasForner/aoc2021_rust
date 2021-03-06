use crate::util::read_to_vec;
use anyhow::{Error, Result};
use std::str::FromStr;

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        if let Some(c) = s.strip_prefix("forward ") {
            Ok(Forward(c.parse()?))
        } else if let Some(c) = s.strip_prefix("up ") {
            Ok(Up(c.parse()?))
        } else if let Some(c) = s.strip_prefix("down ") {
            Ok(Down(c.parse()?))
        } else {
            Err(Error::msg("string is empty"))
        }
    }
}
enum Direction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

pub fn part1(path: &str) -> Result<i64> {
    let mut depth = 0;
    let mut horizontal = 0;
    for dir in read_to_vec(path)? {
        use Direction::*;
        match dir {
            Forward(x) => horizontal += x,
            Up(x) => depth -= x,
            Down(x) => depth += x,
        }
    }
    Ok(depth * horizontal)
}

pub fn part2(path: &str) -> Result<i64> {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for dir in read_to_vec(path)? {
        use Direction::*;
        match dir {
            Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Up(x) => aim -= x,
            Down(x) => aim += x,
        }
    }
    Ok(depth * horizontal)
}
