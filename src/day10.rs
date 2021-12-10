use crate::util::read_to_vec;
use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

use std::collections::HashMap;

struct BracketLine {
    brackets: Vec<char>,
}

impl FromStr for BracketLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let brackets = s.chars().collect();
        Ok(BracketLine { brackets })
    }
}

#[derive(Debug)]
enum PartialParse {
    Corrupted(char),
    Incomplete(Vec<char>),
}

impl PartialParse {
    fn into_score(mut self) -> u64 {
        use PartialParse::*;
        match self {
            Corrupted(x) => *HashMap::from([('}', 1197), (']', 57), (')', 3), ('>', 25137)])
                .get(&x)
                .unwrap_or(&0),
            Incomplete(ref mut stack) => {
                let mut result = 0;
                let points = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
                while !stack.is_empty() {
                    if let Some(x) = stack.pop() {
                        if let Some(p) = points.get(&x) {
                            result = result * 5 + *p;
                        } else {
                            panic!("Illegal char {}", x)
                        }
                    } else {
                        panic!("Should not happen");
                    }
                }
                result
            }
        }
    }

    fn is_corrupted(&self) -> bool {
        matches!(self, &PartialParse::Corrupted(_))
    }
}

impl BracketLine {
    fn partial_parse(&self) -> PartialParse {
        use PartialParse::*;
        let reverse = HashMap::from([('}', '{'), (']', '['), (')', '('), ('>', '<')]);
        let mut stack: Vec<char> = vec![];
        for c in &self.brackets {
            let c = *c;
            if reverse.values().contains(&c) {
                stack.push(c);
            } else if reverse.contains_key(&c) {
                match stack.pop() {
                    Some(x) => {
                        if x != *reverse.get(&c).unwrap_or(&'n') {
                            return Corrupted(c);
                        }
                    }
                    None => {
                        panic!("Should not happen!")
                    }
                }
            }
        }
        return Incomplete(stack);
    }
}

pub fn part1(path: &str) -> Result<u64> {
    let bracket_lines: Vec<BracketLine> = read_to_vec(path)?;
    let partial_parses = bracket_lines.iter().map(|x| x.partial_parse());
    let result: u64 = partial_parses
        .filter(|x| x.is_corrupted())
        .map(|x| x.into_score())
        .sum();
    Ok(result)
}

pub fn part2(path: &str) -> Result<u64> {
    let bracket_lines: Vec<BracketLine> = read_to_vec(path)?;
    let partial_parses = bracket_lines.iter().map(|x| x.partial_parse());
    let mut scores: Vec<u64> = partial_parses
        .filter(|x| !x.is_corrupted())
        .map(|x| x.into_score())
        .collect();
    scores.sort();
    let result = scores[scores.len() / 2];
    Ok(result)
}
