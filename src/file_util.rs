use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

use std::str::FromStr;

pub fn parse_lines<T: FromStr, R: Read>(io: R) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(io);
    br.lines().map(|line| Ok(line?.parse()?)).collect()
}
