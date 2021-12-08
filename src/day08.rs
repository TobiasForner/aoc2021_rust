use crate::util::read_to_vec;
use anyhow::Result;
use std::collections::HashSet;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
struct InputPair {
    digits: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl FromStr for InputPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<InputPair> {
        let v: Vec<&str> = s.split(" | ").collect();
        let digits: Vec<HashSet<char>> = v[0].split(" ").map(|x| x.chars().collect()).collect();
        let output: Vec<HashSet<char>> = v[1].split(" ").map(|x| x.chars().collect()).collect();
        Ok(InputPair { digits, output })
    }
}

impl InputPair {
    fn count_simple_numbers(&self) -> u32 {
        let mut count = 0;
        for x in &self.output {
            if x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7 {
                count += 1;
            }
        }
        count
    }

    fn digit_sum(&self) -> usize {
        //figure out digit representation, this contains the indices into self.digits that have been figured out, or 10000 as default value
        let mut digit_representations: Vec<usize> = vec![10000; 10];
        for (index, x) in self.digits.iter().enumerate() {
            match x.len() {
                2 => digit_representations[1] = index,
                3 => digit_representations[7] = index,
                4 => digit_representations[4] = index,
                7 => digit_representations[8] = index,
                _ => {}
            }
        }

        //3 is the only one with 5 segments and a two segment intersection with 1
        for (index, x) in self.digits.iter().enumerate() {
            if x.len() == 5 {
                if x.intersection(&self.digits[digit_representations[1]])
                    .map(|x| *x)
                    .collect::<Vec<char>>()
                    .len()
                    == 2
                {
                    digit_representations[3] = index;
                }
            }
        }

        // 0, 6 or 9 are the digits with 6 segments
        for (index, x) in self.digits.iter().enumerate() {
            if x.len() == 6 {
                if x.intersection(&self.digits[digit_representations[3]])
                    .map(|x| *x)
                    .collect::<Vec<char>>()
                    .len()
                    == 5
                {
                    digit_representations[9] = index;
                } else if x
                    .intersection(&self.digits[digit_representations[1]])
                    .map(|x| *x)
                    .collect::<Vec<char>>()
                    .len()
                    == 2
                {
                    digit_representations[0] = index;
                } else {
                    digit_representations[6] = index;
                }
            }
        }

        //5 and 2
        for (index, x) in self.digits.iter().enumerate() {
            if x.len() == 5 {
                if x.intersection(&self.digits[digit_representations[1]])
                    .map(|x| *x)
                    .collect::<Vec<char>>()
                    .len()
                    != 2
                {
                    //2 or 5
                    if x.intersection(&self.digits[digit_representations[6]])
                        .map(|x| *x)
                        .collect::<Vec<char>>()
                        .len()
                        == 5
                    {
                        digit_representations[5] = index;
                    } else {
                        digit_representations[2] = index;
                    }
                }
            }
        }

        let mut result = 0;
        let base: usize = 10;
        for (out_index, x) in self.output.iter().rev().enumerate() {
            for (digit, y) in digit_representations.iter().enumerate() {
                if self.digits[*y] == *x {
                    result += base.pow(out_index as u32) * digit;
                    break;
                }
            }
        }
        result
    }
}

pub fn part1(path: &str) -> Result<u32> {
    let inputs: Vec<InputPair> = read_to_vec(path)?;
    let result = inputs.iter().map(|x| x.count_simple_numbers()).sum();
    Ok(result)
}

pub fn part2(path: &str) -> Result<usize> {
    let inputs: Vec<InputPair> = read_to_vec(path)?;
    let result = inputs.iter().map(|x| x.digit_sum()).sum();
    Ok(result)
}
