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
    let mut one_counts = 0;
    for s in &vec {
        let s_chars: Vec<char> = s.chars().collect();
        if s_chars[pos] == '1' {
            one_counts += 1;
        }
    }
    let mut filter_char = '0';
    let boundary: f32 = vec_len as f32 / 2 as f32;
    if one_counts as f32 >= boundary {
        filter_char = '1';
    }
    let filtered_vec: Vec<String> = vec
        .into_iter()
        .filter(|x| x.chars().collect::<Vec<char>>()[pos] == filter_char)
        .collect();
    return oxygen_gen_rating(filtered_vec, pos + 1);
}

fn scrubber_rating(vec: Vec<String>, pos: usize) -> u32 {
    let vec_len = vec.len();
    if vec_len == 1 {
        return u32::from_str_radix(&vec[0][..], 2).unwrap();
    }
    let mut one_counts = 0;
    for s in &vec {
        let s_chars: Vec<char> = s.chars().collect();
        if s_chars[pos] == '1' {
            one_counts += 1;
        }
    }
    let mut filter_char = '0';
    let boundary: f32 = vec_len as f32 / 2 as f32;
    if boundary > one_counts as f32 {
        filter_char = '1';
    }
    let filtered_vec: Vec<String> = vec
        .into_iter()
        .filter(|x| x.chars().collect::<Vec<char>>()[pos] == filter_char)
        .collect();
    return scrubber_rating(filtered_vec, pos + 1);
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
