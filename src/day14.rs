use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct InsertionRule {
    left: (char, char),
    right: char,
}

impl FromStr for InsertionRule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let left_chars: Vec<char> = parts[0].chars().collect();
        let left = (left_chars[0], left_chars[1]);
        let right = parts[1]
            .chars()
            .next()
            .context("There should be a character on the right")?;
        Ok(InsertionRule { left, right })
    }
}

fn parse_input(path: &str) -> Result<(Vec<char>, Vec<InsertionRule>)> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let template: Vec<char> = lines
        .next()
        .context("There should be template")??
        .chars()
        .collect();
    lines.next();
    let rules: Result<Vec<InsertionRule>> = lines.map(|x| InsertionRule::from_str(&x?)).collect();
    Ok((template, rules?))
}

fn run_steps_v2(max_steps: u32, path: &str) -> Result<usize> {
    let (template, rules) = parse_input(path)?;
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    for w in template.windows(2) {
        *counts.entry((w[0], w[1])).or_insert(0) += 1;
    }
    for _ in 1..=max_steps {
        let mut new_counts: HashMap<(char, char), usize> = HashMap::new();
        for ((x, y), count) in &counts {
            for rule in &rules {
                let left = rule.left;
                if left.0 == *x && left.1 == *y {
                    *new_counts.entry((*x, rule.right)).or_insert(0) += count;
                    *new_counts.entry((rule.right, *y)).or_insert(0) += count;
                }
            }
        }
        counts = new_counts;
    }

    //compute char counts
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for ((x, y), count) in &counts {
        *char_counts.entry(*x).or_insert(0) += count;
        *char_counts.entry(*y).or_insert(0) += count;
    }
    *char_counts.entry(template[0]).or_insert(0) += 1;
    *char_counts.entry(template[template.len() - 1]).or_insert(0) += 1;
    let max = *char_counts
        .values()
        .max()
        .context("There should be a max count")?
        / 2;
    let min = *char_counts
        .values()
        .min()
        .context("There should be a min count")?
        / 2;
    //println!("{}, {}", min, max);
    Ok(max - min)
}

#[allow(dead_code)]
fn run_steps(max_steps: u32, path: &str) -> Result<usize> {
    let (mut template, rules) = parse_input(path)?;
    for _ in 1..=max_steps {
        //let mut new_template = template.clone();
        let mut number_inserted = 0;
        let mut to_add: Vec<(usize, char)> = Vec::with_capacity(100);
        for (index, w) in template.windows(2).enumerate() {
            for rule in &rules {
                let left = rule.left;
                if left.0 == w[0] && left.1 == w[1] {
                    to_add.push((index + 1 + number_inserted, rule.right));
                    number_inserted += 1;
                    break;
                }
            }
        }
        for (index, c) in to_add {
            template.insert(index, c);
        }
        println!("{:?}", template);
    }
    let counts = template.into_iter().counts();
    let counts: Vec<&usize> = counts.values().collect();
    let max = **counts.iter().max().context("There should be a max count")?;
    let min = **counts.iter().min().context("There should be a min count")?;
    Ok(max - min)
}

pub fn part1(path: &str) -> Result<usize> {
    run_steps_v2(10, path)
}

pub fn part2(path: &str) -> Result<usize> {
    run_steps_v2(40, path)
}
