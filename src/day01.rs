use crate::{print_result, util::read_to_vec};
use anyhow::Result;
use itertools::Itertools;

pub fn part1() -> Result<()> {
    let mut count: u32 = 0;
    let ints = read_to_vec("./inputs/day01.txt")?;
    let mut last: u32 = u32::MAX;
    for my_int in ints {
        if my_int > last {
            count += 1;
        }
        last = my_int;
    }
    print_result!(1, 1, count);
}

pub fn part2() -> Result<()> {
    let mut count: u32 = 0;
    let mut last: u32 = u32::MAX;
    let it = read_to_vec::<u32>("./inputs/day01.txt")?
        .into_iter()
        .tuple_windows();
    for (x1, x2, x3) in it {
        let value: u32 = x1 + x2 + x3;
        if value > last {
            count += 1;
        }
        last = value;
    }
    print_result!(1, 2, count);
}
