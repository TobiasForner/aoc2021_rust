use crate::util::read_to_vec;
use anyhow::Result;

const UPPER_BOUND: usize = 10;

fn parse_input(path: &str) -> Result<Vec<Vec<u32>>> {
    let in_lines: Vec<String> = read_to_vec(path)?;
    let parsed = in_lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Ok(parsed)
}

fn surrounding(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if x > 0 {
        result.push((x - 1, y));
        if y > 0 {
            result.push((x - 1, y - 1));
        }
        if y < UPPER_BOUND - 1 {
            result.push((x - 1, y + 1));
        }
    }

    if y < UPPER_BOUND - 1 {
        result.push((x, y + 1));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if x < UPPER_BOUND - 1 {
        result.push((x + 1, y));
        if y > 0 {
            result.push((x + 1, y - 1));
        }
        if y < UPPER_BOUND - 1 {
            result.push((x + 1, y + 1));
        }
    }

    result
}

fn simulate(rounds: u32, state: &mut Vec<Vec<u32>>) -> u64 {
    let mut result = 0;
    let mut last_flashed: Vec<Vec<u32>> = vec![vec![0; 10]; 10];
    for step in 1..=rounds {
        result += simulate_step(state, &mut last_flashed, step);
    }
    result
}

fn simulate_step(state: &mut Vec<Vec<u32>>, last_flashed: &mut Vec<Vec<u32>>, step: u32) -> u64 {
    let mut result = 0;
    for row in &mut *state {
        for pos in row {
            *pos += 1;
        }
    }
    let mut change = true;
    while change {
        change = false;
        for y in 0..UPPER_BOUND {
            for x in 0..UPPER_BOUND {
                if state[y][x] > 9 {
                    result += 1;
                    last_flashed[y][x] = step;
                    state[y][x] = 0;
                    change = true;
                    for (x_sur, y_sur) in surrounding(x, y) {
                        if last_flashed[y_sur][x_sur] < step {
                            state[y_sur][x_sur] += 1;
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn part1(path: &str) -> Result<u64> {
    let mut input = parse_input(path)?;
    Ok(simulate(100, &mut input))
}

pub fn part2(path: &str) -> Result<u32> {
    let mut state = parse_input(path)?;
    let mut last_flashed: Vec<Vec<u32>> = vec![vec![0; 10]; 10];
    let mut step = 0;
    loop {
        step += 1;
        let res = simulate_step(&mut state, &mut last_flashed, step);
        if res == 100 {
            return Ok(step);
        }
    }
}
