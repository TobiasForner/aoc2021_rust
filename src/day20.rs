use anyhow::Result;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Result<(Vec<bool>, HashSet<(i32, i32)>, i32, i32)> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let enhancement: Vec<bool> = lines.next().unwrap()?.chars().map(|x| x == '#').collect();
    lines.next();
    let mut light_pixels: HashSet<(i32, i32)> = HashSet::new();
    let mut y_max = 0;
    let mut x_max = 0;
    for (y, line) in lines.enumerate() {
        if y > y_max {
            y_max = y;
        }

        for (x, char) in line?.chars().enumerate() {
            if char == '#' {
                light_pixels.insert((x as i32, y as i32));
            }
            if x > x_max {
                x_max = x;
            }
        }
    }
    Ok((enhancement, light_pixels, y_max as i32, x_max as i32))
}

pub fn part1(path: &str) -> Result<usize> {
    enhance(path, 2)
}

fn enhance(path: &str, repetitions: u32) -> Result<usize> {
    let (enhancement, mut light_pixels, mut y_max, mut x_max) = parse_input(path)?;
    let mut y_min = 0;
    let mut x_min = 0;
    let offsets = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut outside_light = false;
    for count in 1..=repetitions {
        y_max += 1;
        y_min -= 1;
        x_max += 1;
        x_min -= 1;
        let mut new_light: HashSet<(i32, i32)> = HashSet::new();
        if count > 1 {
            if !outside_light && enhancement[0] {
                outside_light = true;
            } else if outside_light && !enhancement[enhancement.len() - 1] {
                outside_light = false;
            }
        }
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let mut index = 0;
                for (pos, o) in offsets.iter().enumerate() {
                    let c_x = x + o.0;
                    let c_y = y + o.1;
                    let outside = c_x <= x_min || c_x >= x_max || c_y <= y_min || c_y >= y_max;
                    if light_pixels.contains(&(c_x, c_y)) && outside {
                        panic!("invalid state");
                    }
                    if light_pixels.contains(&(c_x, c_y)) || (outside & outside_light) {
                        index += 1 << (8 - pos);
                    }
                }
                if enhancement[index as usize] {
                    new_light.insert((x, y));
                }
            }
        }
        light_pixels = new_light;
    }
    Ok(light_pixels.len())
}

pub fn part2(path: &str) -> Result<usize> {
    enhance(path, 50)
}
