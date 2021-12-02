use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_lines<T: FromStr, R: Read>(io: R) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(io);
    br.lines().map(|line| Ok(line?.parse()?)).collect()
}
