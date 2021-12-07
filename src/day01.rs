use crate::util::read_to_vec;
use anyhow::Result;

pub fn part1(path: &str) -> Result<usize> {
    let count = read_to_vec::<u32>(path)?
        .windows(2)
        .filter(|t| t[1] > t[0])
        .count();
    Ok(count)
}

pub fn part2(path: &str) -> Result<usize> {
    let count = read_to_vec::<u32>(path)?
        .windows(4)
        .filter(|t| t[3] > t[0])
        .count();
    Ok(count)
}
