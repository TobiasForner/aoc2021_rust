use crate::{print_result, util::read_to_vec};
use anyhow::Result;

pub fn part1() -> Result<()> {
    let count = read_to_vec::<u32>("./inputs/day01.txt")?
        .windows(2)
        .filter(|t| t[1] > t[0])
        .count();
    print_result!(1, 1, count);
}

pub fn part2() -> Result<()> {
    let count = read_to_vec::<u32>("./inputs/day01.txt")?
        .windows(4)
        .filter(|t| t[3] > t[0])
        .count();
    print_result!(1, 2, count);
}
