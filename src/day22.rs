use anyhow::{Error, Result};

use crate::util::read_to_vec;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Hash, Debug)]
struct Cube {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    op: Operation,
}

#[derive(Clone, Copy, Hash, Debug)]
enum Operation {
    ON,
    OFF,
}

impl FromStr for Cube {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(' ').collect();
        let mut op = Operation::ON;
        if parts[0] == "off" {
            op = Operation::OFF;
        }
        let coords: Result<Vec<i64>, ParseIntError> = parts[1]
            .replace("x=", "")
            .replace(",y=", "..")
            .replace(",z=", "..")
            .split("..")
            .map(|x| x.parse())
            .collect();
        let coords = coords?;
        let x = (coords[0], coords[1]);
        let y = (coords[2], coords[3]);
        let z = (coords[4], coords[5]);
        Ok(Cube { x, y, z, op })
    }
}

impl Cube {
    fn limit(&mut self, lower: i64, upper: i64) {
        self.x = (self.x.0.max(lower), self.x.1.min(upper));
        self.y = (self.y.0.max(lower), self.y.1.min(upper));
        self.z = (self.z.0.max(lower), self.z.1.min(upper));
    }

    ///returns subcubes that are in self, but not in other
    fn difference_cubes(&self, other: &Cube) -> Vec<Cube> {
        let mut res = vec![];
        //top
        if self.z.1 > other.z.1 {
            res.push(Cube {
                x: self.x,
                y: self.y,
                z: ((other.z.1 + 1).max(self.z.0), self.z.1),
                op: self.op,
            });
        }
        let middle_layer_z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));
        if middle_layer_z.0 <= middle_layer_z.1 {
            //middle layer top
            if self.y.1 > other.y.1 {
                res.push(Cube {
                    x: self.x,
                    y: ((other.y.1 + 1).max(self.y.0), self.y.1),
                    z: middle_layer_z,
                    op: self.op,
                });
            }
            let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
            //middle layer left
            if self.x.0 < other.x.0 {
                if y.0 <= y.1 {
                    res.push(Cube {
                        x: (self.x.0, (other.x.0 - 1).min(self.x.1)),
                        y: y,
                        z: middle_layer_z,
                        op: self.op,
                    });
                }
            }
            //middle layer right
            if self.x.1 > other.x.1 {
                if y.0 <= y.1 {
                    res.push(Cube {
                        x: ((other.x.1 + 1).max(self.x.0), self.x.1),
                        y: y,
                        z: middle_layer_z,
                        op: self.op,
                    });
                }
            }
            //middle layer bottom
            if self.y.0 < other.y.0 {
                res.push(Cube {
                    x: self.x,
                    y: (self.y.0, (other.y.0 - 1).min(self.y.1)),
                    z: middle_layer_z,
                    op: self.op,
                });
            }
        }
        //bottom
        if self.z.0 < other.z.0 {
            res.push(Cube {
                x: self.x,
                y: self.y,
                z: (self.z.0, (other.z.0 - 1).min(self.z.1)),
                op: self.op,
            });
        }
        res
    }

    fn size(&self) -> usize {
        if self.x.0 > self.x.1 || self.y.0 > self.y.1 || self.z.0 > self.z.1 {
            return 0;
        }
        ((self.x.1 - self.x.0 + 1) as usize)
            * ((self.y.1 - self.y.0 + 1) as usize)
            * ((self.z.1 - self.z.0 + 1) as usize)
    }
}

pub fn part1(path: &str) -> Result<usize> {
    /*use Operation::*;
    let cubes = read_to_vec::<Cube>(path)?;
    let mut active: HashSet<(i32, i32, i32)> = HashSet::new();
    for mut cube in cubes {
        cube.limit(-50, 50);
        match cube.op {
            ON => {
                for x in cube.x.0..=cube.x.1 {
                    for y in cube.y.0..=cube.y.1 {
                        for z in cube.z.0..=cube.z.1 {
                            active.insert((x, y, z));
                        }
                    }
                }
            }
            OFF => {
                for x in cube.x.0..=cube.x.1 {
                    for y in cube.y.0..=cube.y.1 {
                        for z in cube.z.0..=cube.z.1 {
                            active.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }
    Ok(active.len())*/
    part1_v2(path)
}

pub fn part1_v2(path: &str) -> Result<usize> {
    use Operation::*;
    let cubes = read_to_vec::<Cube>(path)?;
    let mut active_cubes: Vec<Cube> = vec![];
    for mut cube in cubes {
        cube.limit(-50, 50);
        if cube.size() == 0 {
            continue;
        }
        let mut new_cubes: Vec<Cube> = vec![];
        for c in active_cubes {
            new_cubes.append(&mut c.difference_cubes(&cube))
        }

        match cube.op {
            ON => new_cubes.push(cube),
            OFF => {}
        }
        active_cubes = new_cubes;
    }
    let res = active_cubes.iter().map(|x| x.size()).sum();

    Ok(res)
}

pub fn part2(path: &str) -> Result<usize> {
    use Operation::*;
    let cubes = read_to_vec::<Cube>(path)?;
    let mut active_cubes: Vec<Cube> = vec![];
    for cube in cubes {
        let mut new_cubes: Vec<Cube> = vec![];
        for c in active_cubes {
            new_cubes.append(&mut c.difference_cubes(&cube))
        }

        match cube.op {
            ON => new_cubes.push(cube),
            OFF => {}
        }
        active_cubes = new_cubes;
    }
    let res = active_cubes.iter().map(|x| x.size()).sum();

    Ok(res)
}

#[test]
fn test_difference_cubes() {
    use Operation::*;
    let c1 = Cube {
        x: (-10, 10),
        y: (-10, 10),
        z: (-10, 10),
        op: ON,
    };

    let c2 = Cube {
        x: (-5, 5),
        y: (-5, 5),
        z: (-5, 5),
        op: ON,
    };

    let dif = c1.difference_cubes(&c2);
    println!("{:?}", dif);
}
