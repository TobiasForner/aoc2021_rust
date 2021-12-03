use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

use std::fs::File;
use std::str::FromStr;

pub fn read_to_vec<T: FromStr>(path: &str) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    parse_lines(File::open(path)?)
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
