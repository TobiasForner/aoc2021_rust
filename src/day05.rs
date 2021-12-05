use crate::{print_result, util::read_to_vec};
use anyhow::{Error, Result};
use std::cmp::Eq;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

impl FromStr for Edge {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Edge::*;
        let coords: Result<Vec<Point>, Self::Err> = s
            .split(" -> ")
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<Point>, _>>();
        let coords = coords?;
        if coords[0].x == coords[1].x {
            if coords[0].y <= coords[1].y {
                return Ok(Vertical(coords[0], coords[1]));
            } else {
                return Ok(Vertical(coords[1], coords[0]));
            }
        } else if coords[0].y == coords[1].y {
            if coords[0].x <= coords[1].x {
                return Ok(Horizontal(coords[0], coords[1]));
            } else {
                return Ok(Horizontal(coords[1], coords[0]));
            }
        } else if (coords[0].x - coords[0].x).abs() == (coords[0].x - coords[0].x).abs() {
            return Ok(Diagonal(coords[0], coords[1]));
        } else {
            return Err(Error::msg(""));
        }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Result<Vec<i32>, Self::Err> = s
            .split(",")
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<i32>, _>>();
        let coords = coords?;
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Copy)]
pub enum Edge {
    Vertical(Point, Point),
    Horizontal(Point, Point),
    Diagonal(Point, Point),
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Edge::*;
        match self {
            Vertical(start, end) => write!(f, "Vertical({}, {})", start, end),
            Horizontal(start, end) => write!(f, "Vertical({}, {})", start, end),
            Diagonal(start, end) => write!(f, "Vertical({}, {})", start, end),
        }
    }
}

impl Edge {
    pub fn intersection(&self, other: Self, allow_diag: bool) -> HashSet<Point> {
        let mut result = HashSet::new();
        if let Self::Vertical(start, end) = self {
            if let Self::Vertical(start2, end2) = other {
                if start.x == start2.x {
                    for y in max(start.y, start2.y)..=min(end.y, end2.y) {
                        result.insert(Point { x: start.x, y: y });
                    }
                }
            }
            if let Self::Horizontal(start2, end2) = other {
                if end.y >= start2.y
                    && start.y <= start2.y
                    && start.x >= start2.x
                    && start.x <= end2.x
                {
                    result.insert(Point {
                        x: start.x,
                        y: start2.y,
                    });
                }
            } else if allow_diag {
                if let Self::Diagonal(start2, end2) = other {
                    if min(start2.y, end2.y) <= end.y
                        && start.y <= max(start2.y, end2.y)
                        && min(start2.x, end2.x) <= start.x
                        && start.x <= max(start2.x, end2.x)
                    {
                        let step = (start.x - start2.x).abs();
                        let mut step_y = 1;
                        if start2.y > end2.y {
                            step_y = -1;
                        }
                        let y_cand = start2.y + step * step_y;
                        if start.y <= y_cand && y_cand <= end.y {
                            result.insert(Point {
                                x: start.x,
                                y: y_cand,
                            });
                        }
                    }
                }
            }
        } else if let Self::Horizontal(start, end) = self {
            if let Self::Horizontal(start2, end2) = other {
                if start.y == start2.y {
                    for x in max(start.x, start2.x)..=min(end.x, end2.x) {
                        result.insert(Point { x: x, y: start.y });
                    }
                }
            }
            if let Self::Vertical(_, _) = other {
                return other.intersection(*self, allow_diag);
            } else if allow_diag {
                if let Self::Diagonal(start2, end2) = other {
                    if min(start2.x, end2.x) <= end.x
                        && start.x <= max(start2.x, end2.x)
                        && min(start2.y, end2.y) <= start.y
                        && start.y <= max(start2.y, end2.y)
                    {
                        let step = (start.y - start2.y).abs();
                        let mut step_x = 1;
                        if start2.x > end2.x {
                            step_x = -1;
                        }
                        let x_cand = start2.x + step * step_x;
                        if start.x <= x_cand && x_cand <= end.x {
                            result.insert(Point {
                                x: x_cand,
                                y: start.y,
                            });
                        }
                    }
                }
            }
        } else if let Self::Diagonal(_, _) = self {
            if let Self::Diagonal(_, _) = other {
                if allow_diag {
                    return self
                        .points()
                        .intersection(&other.points())
                        .copied()
                        .collect();
                } else {
                    return result;
                }
            }
            return other.intersection(*self, allow_diag);
        }
        return result;
    }

    pub fn points(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        if let Self::Vertical(start, end) = self {
            for y in start.y..=end.y {
                result.insert(Point { x: start.x, y: y });
            }
        }
        if let Self::Horizontal(start, end) = self {
            for x in start.x..=end.x {
                result.insert(Point { x: x, y: start.y });
            }
        }
        if let Self::Diagonal(start, end) = self {
            let mut step_x = 1;
            let mut step_y = 1;
            let num = (start.x - end.x).abs();
            if start.x > end.x {
                step_x = -1;
            }
            if start.y > end.y {
                step_y = -1;
            }
            for step in 0..=num {
                result.insert(Point {
                    x: start.x + step * step_x,
                    y: start.y + step * step_y,
                });
            }
        }
        return result;
    }
}

pub fn unique_intersections(edges: Vec<Edge>, include_diagonals: bool) -> usize {
    let mut points: HashSet<Point> = HashSet::new();

    for i in 0..edges.len() {
        for j in (i + 1)..edges.len() {
            let intersections = edges[i].intersection(edges[j], include_diagonals);

            points.extend(intersections);
        }
    }
    return points.len();
}

pub fn part1() -> Result<(), Error> {
    let edges: Vec<Edge> = read_to_vec("inputs/day05.txt")?;
    print_result!(5, 1, unique_intersections(edges, false));
}

pub fn part2() -> Result<(), Error> {
    let edges: Vec<Edge> = read_to_vec("inputs/day05.txt")?;
    print_result!(5, 2, unique_intersections(edges, true));
}
