use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Lines, Read};

use std::fs::File;
use std::iter::Iterator;
use std::str::FromStr;

pub fn read_to_vec<T: FromStr>(path: &str) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    parse_lines(File::open(path)?)
}

pub fn parse_first_to_vec<T: FromStr>(
    path: &str,
    sep: &str,
) -> Result<(Vec<T>, Lines<BufReader<File>>)>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(File::open(path)?);
    let mut in_lines = br.lines();
    let x = in_lines
        .next()
        .context("Expected at least one line")??
        .split(sep)
        .map(|x| x.parse())
        .collect::<Result<Vec<T>, _>>()?;
    Ok((x, in_lines))
}

fn parse_lines<T: FromStr, R: Read>(io: R) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(io);
    br.lines().map(|line| Ok(line?.parse()?)).collect()
}

#[macro_export]
macro_rules! print_result {
    ($day: expr, $part: expr, $result: expr) => {
        println!("Day {} Part {}: {}", $day, $part, $result);
        return Ok(());
    };
}
