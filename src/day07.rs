use crate::util::parse_first_to_vec;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn parse_state(path: &str) -> Result<HashMap<usize, usize>> {
    let (input, _) = parse_first_to_vec(path, ",")?;
    let counts = input.into_iter().counts();
    Ok(counts)
}

pub fn min_total_dist<F: Fn(usize) -> usize>(
    counts: HashMap<usize, usize>,
    dist: F,
) -> Result<usize> {
    let total_dist = |x: usize| -> usize {
        counts
            .iter()
            .map(|t| t.1 * dist(max(x, *t.0) - min(x, *t.0)))
            .sum()
    };
    let start = *counts.keys().min().context("No max")?;
    let end = *counts.keys().max().context("No min")?;
    let res = (start..=end)
        .map(|x| total_dist(x))
        .min()
        .context("Expected at least one entry")?;

    Ok(res)
}

pub fn part1(path: &str) -> Result<usize> {
    let counts = parse_state(path)?;
    min_total_dist(counts, |x| x)
}

pub fn part2(path: &str) -> Result<usize> {
    let counts = parse_state(path)?;
    min_total_dist(counts, |x| (x * (x + 1)) / 2)
}
