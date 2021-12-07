use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::print_result;
use anyhow::{Context, Error, Result};
use std::collections::HashMap;

pub fn parse_state(path: &str) -> Result<(HashMap<usize, usize>, usize)> {
    let br = BufReader::new(File::open(path)?);
    let mut in_lines = br.lines();
    let input = in_lines
        .next()
        .context("Expected at least one line")??
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut first_fuel_cost = 0;
    for x in input {
        let old = *counts.entry(x).or_insert(0);
        counts.insert(x, old + 1);
        first_fuel_cost += x;
    }
    Ok((counts, first_fuel_cost))
}

pub fn part1() -> Result<()> {
    let (mut counts, first_fuel_cost) = parse_state("inputs/day07.txt")?;
    let mut left = 0;
    let mut right: usize = counts.values().sum::<usize>() as usize;
    let max_pos = counts.keys().max().context("Expected at least one entry")?;
    let mut cost = first_fuel_cost;
    let mut min_cost = cost;

    for pos in 0..=*max_pos {
        let pos_count = *counts.entry(pos).or_insert(0);
        right -= pos_count;
        left += pos_count;
        cost += left;
        cost -= right;

        if cost < min_cost {
            min_cost = cost;
        }
    }

    print_result!(7, 1, min_cost);
}

pub fn part2() -> Result<()> {
    let (counts, _) = parse_state("inputs/day07.txt")?;
    let max_pos = counts.keys().max().context("Expected at least one entry")?;
    let mut cost;
    let mut min_cost = usize::MAX;

    for pos in 0..=*max_pos {
        let mut left_cost = 0;
        for l_pos in 0..pos {
            let pos_count = *counts.get(&l_pos).unwrap_or(&0);
            let dist = pos - l_pos;
            left_cost += dist * (dist + 1) * pos_count / 2;
        }

        let mut right_cost = 0;
        for r_pos in pos + 1..=*max_pos {
            let pos_count = *counts.get(&r_pos).unwrap_or(&0);
            let dist = r_pos - pos;
            right_cost += dist * (dist + 1) * pos_count / 2;
        }
        cost = left_cost + right_cost;

        if cost < min_cost {
            min_cost = cost;
        }
    }
    print_result!(7, 2, min_cost);
}
