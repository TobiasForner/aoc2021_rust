use crate::{print_result, util::read_to_vec};
use anyhow::Result;

pub fn part1() -> Result<()> {
    let input_vec: Vec<String> = read_to_vec("./inputs/day03.txt")?;
    let vec_len = input_vec.len();
    let mut one_counts = vec![0; input_vec[0].chars().count()];
    for s in input_vec {
        for (index, bit) in s.chars().enumerate() {
            if bit == '1' {
                one_counts[index] += 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in one_counts {
        if i > vec_len / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }
    let epsilon = isize::from_str_radix(&epsilon[..], 2).unwrap();
    let gamma = isize::from_str_radix(&gamma[..], 2).unwrap();
    print_result!(3, 1, gamma * epsilon);
}
