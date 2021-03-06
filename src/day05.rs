use crate::util::read_to_vec;
use anyhow::{Error, Result};
use std::cmp::Eq;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

impl FromStr for Edge {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EdgeType::*;
        let coords: Result<Vec<Point>, Self::Err> = s
            .split(" -> ")
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<Point>, _>>();
        let coords = coords?;
        Ok(if coords[0].x == coords[1].x {
            if coords[0].y <= coords[1].y {
                Edge {
                    start: coords[0],
                    end: coords[1],
                    typ: Vertical,
                }
            } else {
                Edge {
                    start: coords[1],
                    end: coords[0],
                    typ: Vertical,
                }
            }
        } else if coords[0].y == coords[1].y {
            if coords[0].x <= coords[1].x {
                Edge {
                    start: coords[0],
                    end: coords[1],
                    typ: Horizontal,
                }
            } else {
                Edge {
                    start: coords[1],
                    end: coords[0],
                    typ: Horizontal,
                }
            }
        } else if (coords[0].x - coords[1].x).abs() == (coords[0].y - coords[1].y).abs() {
            if coords[0].x <= coords[1].x {
                Edge {
                    start: coords[0],
                    end: coords[1],
                    typ: Diagonal,
                }
            } else {
                Edge {
                    start: coords[1],
                    end: coords[0],
                    typ: Diagonal,
                }
            }
        } else {
            return Err(Error::msg(""));
        })
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s
            .split(",")
            .map(|x| x.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Copy, Debug)]
pub enum EdgeType {
    Vertical,
    Horizontal,
    Diagonal,
}
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
    pub typ: EdgeType,
}

impl Edge {
    fn contains(&self, p: &Point) -> bool {
        use EdgeType::*;
        let start = self.start;
        let end = self.end;
        match self.typ {
            Vertical => p.x == start.x && start.y <= p.y && end.y >= p.y,
            Horizontal => p.y == start.y && start.x <= p.x && end.x >= p.x,
            Diagonal => {
                let step = (p.y - start.y).abs();
                if step > end.x - start.x {
                    return false;
                }
                let step_x = 1; //due to sorting
                let mut step_y = 1;
                if start.y > end.y {
                    step_y = -1;
                }
                let x_cand = start.x + step * step_x;
                let y_cand = start.y + step * step_y;
                x_cand == p.x && y_cand == p.y
            }
        }
    }
    pub fn intersection(&self, other: &Self, allow_diag: bool) -> Result<HashSet<Point>> {
        use EdgeType::*;
        let mut result = HashSet::new();
        let start = self.start;
        let end = self.end;
        let start2 = other.start;
        let end2 = other.end;
        match (self.typ, other.typ) {
            (Vertical, Vertical) => {
                if start.x == start2.x {
                    for y in max(start.y, start2.y)..=min(end.y, end2.y) {
                        result.insert(Point { x: start.x, y: y });
                    }
                }
            }
            (Vertical, Horizontal) => {
                let p = Point {
                    x: start.x,
                    y: start2.y,
                };
                if self.contains(&p) && other.contains(&p) {
                    result.insert(p);
                }
            }
            (Vertical, Diagonal) => {
                if allow_diag {
                    let (m, t) = other.compute_diag_parameters()?;
                    let y_cand = m * start.x + t;
                    let p = Point {
                        x: start.x,
                        y: y_cand,
                    };
                    if self.contains(&p) && other.contains(&p) {
                        result.insert(p);
                    }
                }
            }
            (Horizontal, Horizontal) => {
                if start.y == start2.y {
                    for x in max(start.x, start2.x)..=min(end.x, end2.x) {
                        result.insert(Point { x: x, y: start.y });
                    }
                }
            }
            (Horizontal, Diagonal) => {
                if allow_diag {
                    let (m, t) = other.compute_diag_parameters()?;
                    let x_cand = (start.y - t) / m;
                    let p = Point {
                        x: x_cand,
                        y: start.y,
                    };
                    if self.contains(&p) && other.contains(&p) {
                        result.insert(p);
                    }
                }
            }
            (Diagonal, Diagonal) => {
                if allow_diag {
                    if min(start2.x, end2.x) <= max(start.x, end.x)
                        && min(end.x, start.x) <= max(start2.x, end2.x)
                        && min(start2.y, end2.y) <= max(start.y, end.y)
                        && min(end.y, start.y) <= max(start2.y, end2.y)
                    {
                        /*
                        m_1 x+t_1=m_2 x+t_2 =>(m_1-m_2)x=t_2 - t_1=> x= (t_2-t_1)/(m_1-m_2)
                        */
                        let (m1, t1) = self.compute_diag_parameters()?;
                        let (m2, t2) = other.compute_diag_parameters()?;
                        if m1 != m2 {
                            let x = (t2 - t1) / (m1 - m2);
                            let y = m1 * x + t1;
                            let p = Point { x, y };
                            if self.contains(&p) && other.contains(&p) {
                                result.insert(p);
                            }
                        } else if t1 == t2 {
                            for x in max(start.x, start2.x)..=min(end.x, end2.x) {
                                let y = m1 * x + t1;
                                let p = Point { x, y };
                                if self.contains(&p) && other.contains(&p) {
                                    result.insert(p);
                                }
                            }
                        }
                    }
                }
            }
            _ => return other.intersection(self, allow_diag),
        }
        return Ok(result);
    }
    fn compute_diag_parameters(&self) -> Result<(i32, i32)> {
        match self.typ {
            EdgeType::Diagonal => {
                let mut m = 1;
                if self.start.y > self.end.y {
                    m = -1;
                }
                let t = self.start.y - m * self.start.x;
                Ok((m, t))
            }
            _ => Err(Error::msg("Non-diagonal has no parameters!")),
        }
    }
}

pub fn unique_intersections(edges: Vec<Edge>, include_diagonals: bool) -> Result<usize> {
    let instant = Instant::now();
    let mut points: HashSet<Point> = HashSet::new();

    for i in 0..edges.len() {
        for j in (i + 1)..edges.len() {
            let intersections = edges[i].intersection(&edges[j], include_diagonals);

            points.extend(intersections?);
        }
    }
    println!("finished in {} ms", instant.elapsed().as_millis());
    return Ok(points.len());
}

pub fn part1(path: &str) -> Result<usize> {
    let edges: Vec<Edge> = read_to_vec(path)?;
    //print_result!(5, 1, unique_intersections(edges, false)?);
    unique_intersections(edges, false)
}

pub fn part2(path: &str) -> Result<usize> {
    let edges: Vec<Edge> = read_to_vec(path)?;
    unique_intersections(edges, true)
}
