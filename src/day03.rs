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

pub fn part2() -> Result<()> {
    let input_vec: Vec<String> = read_to_vec("./inputs/day03.txt")?;
    let ox_gen_rating = oxygen_gen_rating(input_vec, 0);
    //parsed again due to move
    let input_vec: Vec<String> = read_to_vec("./inputs/day03.txt")?;
    let scrub_rating = scrubber_rating(input_vec, 0);
    print_result!(3, 2, ox_gen_rating * scrub_rating);
}

fn oxygen_gen_rating(vec: Vec<String>, pos: usize) -> u32 {
    let vec_len = vec.len();
    if vec_len == 1 {
        return u32::from_str_radix(&vec[0][..], 2).unwrap();
    }
    let (one_at_pos, zero_at_pos): (_, Vec<String>) = vec
        .into_iter()
        .partition(|x| x.chars().collect::<Vec<char>>()[pos] == '1');
    if one_at_pos.len() >= zero_at_pos.len() {
        return oxygen_gen_rating(one_at_pos, pos + 1);
    } else {
        return oxygen_gen_rating(zero_at_pos, pos + 1);
    }
}

fn scrubber_rating(vec: Vec<String>, pos: usize) -> u32 {
    let vec_len = vec.len();
    if vec_len == 1 {
        return u32::from_str_radix(&vec[0][..], 2).unwrap();
    }
    let (one_at_pos, zero_at_pos): (_, Vec<String>) = vec
        .into_iter()
        .partition(|x| x.chars().collect::<Vec<char>>()[pos] == '1');
    if one_at_pos.len() < zero_at_pos.len() {
        return scrubber_rating(one_at_pos, pos + 1);
    } else {
        return scrubber_rating(zero_at_pos, pos + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oxygen() {
        if let Ok(input_vec) = read_to_vec("./inputs/day03_test.txt") {
            let res = oxygen_gen_rating(input_vec, 0);
            assert_eq!(res, 23);
        }
    }
    #[test]
    fn test_scrubber() {
        if let Ok(input_vec) = read_to_vec("./inputs/day03_test.txt") {
            let res = scrubber_rating(input_vec, 0);
            assert_eq!(res, 10);
        }
    }
}
