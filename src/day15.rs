use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Result<Vec<Vec<usize>>> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let lines = br.lines();
    const RADIX: u32 = 10;
    let result: Vec<Vec<usize>> = lines
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(RADIX).unwrap() as usize)
                .collect()
        })
        .collect();
    Ok(result)
}

fn solve(risk_levels: Vec<Vec<usize>>) -> Result<usize> {
    let mut total_risk_levels = vec![vec![usize::MAX; risk_levels[0].len()]; risk_levels.len()];
    total_risk_levels[0][0] = 0;
    let mut change = true;
    while change {
        change = false;
        for y in 0..risk_levels.len() {
            for x in 0..risk_levels[0].len() {
                if x == 0 && y == 0 {
                    continue;
                }
                //update own value
                if x > 0 {
                    if total_risk_levels[y][x - 1] < usize::MAX {
                        let new_risk = total_risk_levels[y][x - 1] + risk_levels[y][x];
                        if new_risk < total_risk_levels[y][x] {
                            total_risk_levels[y][x] = new_risk;
                            change = true;
                        }
                    }
                }
                if y > 0 {
                    if total_risk_levels[y - 1][x] < usize::MAX {
                        let new_risk = total_risk_levels[y - 1][x] + risk_levels[y][x];
                        if new_risk < total_risk_levels[y][x] {
                            total_risk_levels[y][x] = new_risk;
                            change = true;
                        }
                    }
                }

                if x < risk_levels[0].len() - 1 {
                    if total_risk_levels[y][x + 1] < usize::MAX {
                        let new_risk = total_risk_levels[y][x + 1] + risk_levels[y][x];
                        if new_risk < total_risk_levels[y][x] {
                            total_risk_levels[y][x] = new_risk;
                            change = true;
                        }
                    }
                }
                if y < risk_levels.len() - 1 {
                    if total_risk_levels[y + 1][x] < usize::MAX {
                        let new_risk = total_risk_levels[y + 1][x] + risk_levels[y][x];
                        if new_risk < total_risk_levels[y][x] {
                            total_risk_levels[y][x] = new_risk;
                            change = true;
                        }
                    }
                }
            }
        }
    }
    Ok(total_risk_levels[total_risk_levels.len() - 1][total_risk_levels[0].len() - 1])
}

pub fn part1(path: &str) -> Result<usize> {
    let risk_levels = parse_input(path)?;
    solve(risk_levels)
}

pub fn part2(path: &str) -> Result<usize> {
    let risk_levels = parse_input(path)?;
    let x_len = risk_levels[0].len();
    let y_len = risk_levels.len();
    let total_x_len = 5 * risk_levels[0].len();
    let total_y_len = 5 * risk_levels.len();

    let mut large_risk_levels =
        vec![vec![usize::MAX; 5 * risk_levels[0].len()]; 5 * risk_levels.len()];
    for y in 0..total_y_len {
        for x in 0..total_x_len {
            if x < x_len && y < y_len {
                large_risk_levels[y][x] = risk_levels[y][x];
            } else if x >= x_len {
                let mut new_risk_level = large_risk_levels[y][x - x_len] + 1;
                if new_risk_level > 9 {
                    new_risk_level = 1;
                }
                large_risk_levels[y][x] = new_risk_level;
            } else {
                let mut new_risk_level = large_risk_levels[y - y_len][x] + 1;
                if new_risk_level > 9 {
                    new_risk_level = 1;
                }
                large_risk_levels[y][x] = new_risk_level;
            }
        }
    }

    solve(large_risk_levels)
}
