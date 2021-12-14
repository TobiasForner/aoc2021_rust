use crate::parse_input;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<u32> = s
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<Vec<u32>, _>>()?;
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let parts: Vec<&str> = parts[2].split("=").collect();
        Ok(if parts[0] == "x" {
            Fold::X(parts[1].parse()?)
        } else {
            Fold::Y(parts[1].parse()?)
        })
    }
}

fn apply_fold(f: &Fold, points: HashSet<Point>) -> HashSet<Point> {
    use Fold::*;
    let mut result = HashSet::new();
    match f {
        X(x) => {
            for p in points {
                if p.x < *x {
                    result.insert(p);
                } else if p.x > *x {
                    let new_p = Point {
                        x: 2 * x - p.x,
                        y: p.y,
                    };
                    result.insert(new_p);
                }
            }
        }
        Y(y) => {
            for p in points {
                if p.y < *y {
                    result.insert(p);
                } else if p.y > *y {
                    let new_p = Point {
                        x: p.x,
                        y: 2 * y - p.y,
                    };
                    result.insert(new_p);
                }
            }
        }
    }
    result
}

pub fn part1(path: &str) -> Result<u64> {
    let mut points = vec![];
    let mut folds = vec![];

    parse_input!(path, points, Point, "", folds, Fold, "");
    let mut points2: HashSet<Point> = HashSet::new();
    for p in points {
        points2.insert(p);
    }
    let points_after_fold = apply_fold(&folds[0], points2);
    let res = points_after_fold.len();
    Ok(res as u64)
}

pub fn part2(path: &str) -> Result<u64> {
    let mut points = vec![];
    let mut folds = vec![];

    //parse_input!(path, points, Point, "", folds, Fold, "");
    parse_input!(path, points, Point, "", folds, Fold, "");
    let mut points2: HashSet<Point> = HashSet::new();
    for p in points {
        points2.insert(p);
    }
    for f in folds {
        points2 = apply_fold(&f, points2);
    }
    let max_x = points2.iter().map(|p| p.x).max().context("")? as usize;
    let max_y = points2.iter().map(|p| p.y).max().context("")? as usize;
    let mut res = vec![vec!['.'; max_x + 1]; max_y + 1];
    for p in points2 {
        res[p.y as usize][p.x as usize] = '#';
    }
    for line in res {
        let s: String = line.iter().collect();
        println!("{}", s);
    }
    Ok(0)
}
