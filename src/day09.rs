use crate::util::read_to_vec;
use anyhow::{Context, Result};
use std::str::FromStr;

pub struct HeightRow {
    depths: Vec<u32>,
}

impl FromStr for HeightRow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        const RADIX: u32 = 10;
        let depths = s
            .chars()
            .map(|x| x.to_digit(RADIX).context("All chars should be digits"))
            .collect::<Result<Vec<u32>>>()?;
        Ok(HeightRow { depths })
    }
}

pub fn part1(path: &str) -> Result<u32> {
    let height_rows: Vec<HeightRow> = read_to_vec(path)?;
    let mut result = 0;
    for (y, row) in height_rows.iter().enumerate() {
        for (x, depth) in row.depths.iter().enumerate() {
            if is_low_point(x, y, &height_rows) {
                result += depth + 1;
            }
        }
    }
    Ok(result)
}

fn is_low_point(x: usize, y: usize, height_rows: &Vec<HeightRow>) -> bool {
    let row = &height_rows[y];
    let depth = row.depths[x];
    let left_larger = x <= 0 || row.depths[x - 1] > depth;
    let right_larger = x >= row.depths.len() - 1 || row.depths[x + 1] > depth;
    let top_larger = y <= 0 || height_rows[y - 1].depths[x] > depth;
    let bottom_larger = y >= height_rows.len() - 1 || height_rows[y + 1].depths[x] > depth;
    left_larger && right_larger && top_larger && bottom_larger
}

pub fn part2(path: &str) -> Result<u32> {
    let height_rows: Vec<HeightRow> = read_to_vec(path)?;
    let mut basin_mapping = vec![vec![-1; height_rows[0].depths.len()]; height_rows.len()];
    let mut basin_sizes = vec![0; 10];
    let mut basin_count: i32 = 0;
    for (y, row) in height_rows.iter().enumerate() {
        for (x, _) in row.depths.iter().enumerate() {
            if is_low_point(x, y, &height_rows) {
                basin_mapping[y][x] = basin_count;
                if basin_sizes.len() == basin_count as usize {
                    basin_sizes.append(&mut vec![0; basin_sizes.len()]);
                }
                basin_sizes[basin_count as usize] += 1;
                basin_count += 1;
            }
        }
    }
    let mut change: bool = true;
    while change {
        change = false;
        for (y, row) in height_rows.iter().enumerate() {
            for (x, depth) in row.depths.iter().enumerate() {
                if basin_mapping[y][x] != -1 || *depth == 9 {
                    continue;
                }
                if add_to_basin(*depth, x - 1, y, &height_rows, &basin_mapping) {
                    basin_mapping[y][x] = basin_mapping[y][x - 1];
                    basin_sizes[basin_mapping[y][x] as usize] += 1;
                    change = true;
                } else if add_to_basin(*depth, x + 1, y, &height_rows, &basin_mapping) {
                    basin_mapping[y][x] = basin_mapping[y][x + 1];
                    basin_sizes[basin_mapping[y][x] as usize] += 1;
                    change = true;
                } else if add_to_basin(*depth, x, y - 1, &height_rows, &basin_mapping) {
                    basin_mapping[y][x] = basin_mapping[y - 1][x];
                    basin_sizes[basin_mapping[y][x] as usize] += 1;
                    change = true;
                } else if add_to_basin(*depth, x, y + 1, &height_rows, &basin_mapping) {
                    basin_mapping[y][x] = basin_mapping[y + 1][x];
                    basin_sizes[basin_mapping[y][x] as usize] += 1;
                    change = true;
                }
            }
        }
    }
    basin_sizes.sort();
    if basin_sizes.len() < 3 {
        panic!("There need to be a t least 3 basins!");
    }
    let res = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];
    Ok(res as u32)
}

fn add_to_basin(
    depth: u32,
    x: usize,
    y: usize,
    height_rows: &Vec<HeightRow>,
    basin_mapping: &Vec<Vec<i32>>,
) -> bool {
    y < height_rows.len()
        && x < height_rows[0].depths.len()
        && basin_mapping[y][x] != -1
        && depth < height_rows[y].depths[x]
}
