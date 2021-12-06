use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::print_result;
use anyhow::{Error, Result};

pub fn part1() -> Result<()> {
    let mut state = parse_state("inputs/day06.txt")?;
    print_result!(6, 1, simulate(&mut state, 80)?);
}

pub fn part2() -> Result<()> {
    let mut state = parse_state("inputs/day06.txt")?;
    print_result!(6, 1, simulate(&mut state, 256)?);
}

pub fn parse_state(path: &str) -> Result<Vec<i64>> {
    let br = BufReader::new(File::open(path)?);
    let mut in_lines = br.lines();
    let input = in_lines
        .next()
        .ok_or_else(|| Error::msg("Expected at least one line"))??
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    let mut counts = vec![0; 9];
    for x in input {
        counts[x] += 1;
    }
    Ok(counts)
}

pub fn simulate(state: &mut Vec<i64>, turns: u32) -> Result<i64> {
    for _ in 0..turns {
        let zeros = state[0];
        for pos in 1..=8 {
            state[pos - 1] = state[pos];
        }
        state[6] += zeros;
        state[8] = zeros;
    }
    return Ok(state.iter().sum());
}
