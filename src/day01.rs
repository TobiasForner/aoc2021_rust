use crate::file_util::parse_lines;
use anyhow::Result;
use itertools::Itertools;
use std::fs::File;

pub fn part1() -> Result<()> {
    let mut count: u32 = 0;
    let ints = parse_lines(File::open("./inputs/day01.txt")?)?;
    let mut last: u32 = u32::MAX;
    for my_int in ints {
        if my_int > last {
            count += 1;
        }
        last = my_int;
    }
    println!("Part 1: {}", count);
    Ok(())
}

pub fn part2() -> Result<()> {
    let mut count: u32 = 0;
    let mut last: u32 = u32::MAX;
    let it = parse_lines::<u32, _>(File::open("./inputs/day01.txt")?)?
        .into_iter()
        .tuple_windows();
    for (x1, x2, x3) in it {
        let value: u32 = x1 + x2 + x3;
        if value > last {
            count += 1;
        }
        last = value;
    }
    println!("Part 2: {}", count);
    Ok(())
}
