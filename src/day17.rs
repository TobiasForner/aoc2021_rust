use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn parse_input(path: &str) -> Result<(i32, i32, i32, i32)> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let mut line = lines.next().unwrap()?;
    line = line.replace("target area: x=", "").replace(", y=", "..");
    let bounds: Vec<i32> = line
        .split("..")
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;
    Ok((bounds[0], bounds[1], bounds[2], bounds[3]))
}

pub fn part1(path: &str) -> Result<i32> {
    let (x_start, x_end, y_start, y_end) = parse_input(path)?;
    let mut min_x_v = 0;
    loop {
        if min_x_v * (min_x_v + 1) / 2 >= x_start {
            break;
        }
        min_x_v += 1;
    }
    let mut max_y_v = 0;
    loop {
        let upper_y_v = if min_x_v * (min_x_v + 1) / 2 > x_end {
            x_end / 2
        } else {
            y_start.abs() + 1
        };
        for y_v in max_y_v..=upper_y_v {
            if hits_target(min_x_v, y_v, x_start, x_end, y_start, y_end) {
                max_y_v = y_v;
            }
        }
        min_x_v += 1;
        if 2 * max_y_v > min_x_v {
            break;
        }
    }
    Ok(max_y_v * (max_y_v + 1) / 2)
}

pub fn part2(path: &str) -> Result<i32> {
    let mut result = 0;
    let (x_start, x_end, y_start, y_end) = parse_input(path)?;
    let mut min_x_v = 0;
    loop {
        if min_x_v * (min_x_v + 1) / 2 >= x_start {
            break;
        }
        min_x_v += 1;
    }
    loop {
        let upper_y_v = if min_x_v * (min_x_v + 1) / 2 > x_end {
            x_end / 2
        } else {
            y_start.abs() + 1
        };
        for y_v in y_start..=upper_y_v {
            if hits_target(min_x_v, y_v, x_start, x_end, y_start, y_end) {
                result += 1;
            }
        }
        min_x_v += 1;
        if min_x_v > x_end {
            break;
        }
    }
    Ok(result)
}

fn hits_target(x_vel: i32, y_vel: i32, x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut current_v = (x_vel, y_vel);
    loop {
        if current_v.0 == 0 && (x_pos < x_start || x_pos > x_end) {
            return false;
        }
        if x_pos > x_end || y_pos < y_start {
            return false;
        }
        x_pos += current_v.0;
        y_pos += current_v.1;
        if x_start <= x_pos && x_pos <= x_end && y_start <= y_pos && y_pos <= y_end {
            return true;
        }
        if current_v.0 > 0 {
            current_v.0 -= 1;
        }
        current_v.1 -= 1;
    }
}
