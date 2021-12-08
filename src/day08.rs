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
        //contains the character sets representing digit i at position i
        let mut digit_representations2: Vec<HashSet<char>> = vec![HashSet::new(); 10];
        for x in &self.digits {
            let x = x.clone();
            match x.len() {
                2 => digit_representations2[1] = x,
                3 => digit_representations2[7] = x,
                4 => digit_representations2[4] = x,
                7 => digit_representations2[8] = x,
                _ => {}
            }
        }

        //3 is the only one with 5 segments and a two segment intersection with 1
        for x in &self.digits {
            if x.len() == 5 {
                if x.intersection(&digit_representations2[1]).count() == 2 {
                    digit_representations2[3] = x.clone();
                }
            }
        }

        // 0, 6 or 9 are the digits with 6 segments
        for x in &self.digits {
            if x.len() == 6 {
                if x.intersection(&digit_representations2[3]).count() == 5 {
                    digit_representations2[9] = x.clone();
                } else if x.intersection(&digit_representations2[1]).count() == 2 {
                    digit_representations2[0] = x.clone();
                } else {
                    digit_representations2[6] = x.clone();
                }
            }
        }

        //5 and 2
        for x in &self.digits {
            if x.len() == 5 {
                if x.intersection(&digit_representations2[1]).count() != 2 {
                    //2 or 5
                    if x.intersection(&digit_representations2[6]).count() == 5 {
                        digit_representations2[5] = x.clone();
                    } else {
                        digit_representations2[2] = x.clone();
                    }
                }
            }
        }

        let mut result = 0;
        let base: usize = 10;
        for (out_index, x) in self.output.iter().rev().enumerate() {
            for (digit, y) in digit_representations2.iter().enumerate() {
                if *y == *x {
                    result += base.pow(out_index as u32) * digit;
                    break;
                }
            }
        }
        return result;
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
