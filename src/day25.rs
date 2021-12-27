use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq)]
struct SeaCucumber {
    x: usize,
    y: usize,
}

fn parse_input(
    path: &str,
) -> Result<(
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    usize,
    usize,
)> {
    let mut east_positions = HashSet::new();
    let mut south_positions = HashSet::new();
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let lines = br.lines();
    let mut y_max = 0;
    let mut x_max = 0;
    for (y, row) in lines.enumerate() {
        y_max += 1;
        let row = row?;
        x_max = row.len();
        for (x, c) in row.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == '>' {
                east_positions.insert((x, y));
            } else {
                south_positions.insert((x, y));
            };
        }
    }
    Ok((east_positions, south_positions, y_max, x_max))
}

#[allow(dead_code)]
fn print_state(
    east_positions: &HashSet<(usize, usize)>,
    south_positions: &HashSet<(usize, usize)>,
    y_max: usize,
    x_max: usize,
) {
    println!("------------------------");
    for y in 0..=y_max {
        let mut line: String = String::new();
        for x in 0..=x_max {
            if east_positions.contains(&(x, y)) {
                line += ">";
            } else if south_positions.contains(&(x, y)) {
                line += "V";
            } else {
                line += "."
            };
        }
        println!("{}", line);
    }
}

pub fn part1(path: &str) -> Result<u32> {
    let (mut east_positions, mut south_positions, y_max, x_max) = parse_input(path)?;
    let mut change = true;
    let mut count = 0;
    while change {
        count += 1;
        //println!("Step {}", count);
        change = false;
        let mut new_east_positions = HashSet::new();
        //east-facing
        for pos in &east_positions {
            let pos = *pos;
            let new_x_pos = if pos.0 + 1 == x_max { 0 } else { pos.0 + 1 };
            let new_pos = (new_x_pos, pos.1);
            if !east_positions.contains(&new_pos) && !south_positions.contains(&new_pos) {
                new_east_positions.insert(new_pos);
                change = true;
            } else {
                new_east_positions.insert(pos);
            }
        }
        east_positions = new_east_positions;

        let mut new_south_positions = HashSet::new();
        //east-facing
        for pos in &south_positions {
            let pos = *pos;
            let new_y_pos = if pos.1 + 1 == y_max { 0 } else { pos.1 + 1 };
            let new_pos = (pos.0, new_y_pos);
            if !east_positions.contains(&new_pos) && !south_positions.contains(&new_pos) {
                new_south_positions.insert(new_pos);
                change = true;
            } else {
                new_south_positions.insert(pos);
            }
        }
        south_positions = new_south_positions;
        //print_state(&east_positions, &south_positions, y_max, x_max);
    }

    Ok(count)
}

pub fn part2(_path: &str) -> Result<u32> {
    Ok(0)
}
