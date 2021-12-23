use anyhow::{Error, Result};
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
type Cord = i32;
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: Cord,
    y: Cord,
    z: Cord,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Position {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let cords: Result<Vec<Cord>, ParseIntError> = s.split(',').map(|x| x.parse()).collect();
        let cords = cords?;
        Ok(Position {
            x: cords[0],
            y: cords[1],
            z: cords[2],
        })
    }
}

impl Position {
    fn dist(&self, other: Self) -> u32 {
        let tmp = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f32;
        tmp.sqrt().round() as u32
    }

    fn dist_pos(&self, other: Self) -> Self {
        Position {
            x: (self.x - other.x),
            y: (self.y - other.y),
            z: (self.z - other.z),
        }
    }
    fn rotate_x(&self) -> Self {
        Position {
            x: self.x,
            y: self.z,
            z: -1 * self.y,
        }
    }

    fn rotate_y(&self) -> Self {
        Position {
            x: -1 * self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rotate_z(&self) -> Self {
        Position {
            x: self.y,
            y: -1 * self.x,
            z: self.z,
        }
    }
}

fn parse_input(path: &str) -> Result<Vec<Vec<Position>>> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let lines = br.lines();
    let mut scan_results: Vec<Vec<Position>> = vec![];
    const CONTAINS: &str = "scanner";
    let mut pos: usize = 0;
    for line in lines {
        let line = line?;
        if line.contains(CONTAINS) {
            scan_results.push(vec![]);
            pos = scan_results.len() - 1;
        } else if !line.is_empty() {
            scan_results[pos].push(line.parse()?);
        }
    }
    Ok(scan_results)
}

fn merge_scanners(first: usize, second: usize, scans: &mut Vec<Vec<Position>>) -> (bool, Position) {
    //try all rotations of second
    let mut current = scans[second].clone();
    //z points to front
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //x points to front
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //z points to back
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //x points to back
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }
    //y points to front
    current = current.iter().map(|x| x.rotate_x()).collect();
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }
    //y points to back
    current = current.iter().map(|x| x.rotate_y().rotate_y()).collect();
    let (found, pos) = check_all_z_rotations(&mut current, scans, first, second);
    if found {
        return (true, pos);
    }

    //check all point pairs of both for same distance
    //if match is at least of size 12 then merge the vecs
    (false, Position { x: 0, y: 0, z: 0 })
}

fn check_all_z_rotations(
    current: &mut Vec<Position>,
    scans: &mut Vec<Vec<Position>>,
    first: usize,
    second: usize,
) -> (bool, Position) {
    for _ in 0..=3 {
        *current = current.iter().map(|x| x.rotate_z()).collect();

        //println!("{:?}", current);
        let (success, positions, p) = check_match(&mut scans[first], &current);
        if success {
            scans[first].extend(positions);
            scans[first].sort();
            scans[first].dedup();
            scans.remove(second);
            return (true, p);
        }
    }
    (false, Position { x: 0, y: 0, z: 0 })
}

fn merge_scanners_hints(
    first: usize,
    second: usize,
    scans: &mut Vec<Vec<Position>>,
    pos1: usize,
    pos2: usize,
) -> (bool, Position) {
    //try all rotations of second
    let mut current = scans[second].clone();
    //z points to front
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //x points to front
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //z points to back
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }
    current = current.iter().map(|x| x.rotate_y()).collect();
    //x points to back
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }
    //y points to front
    current = current.iter().map(|x| x.rotate_x()).collect();
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }
    //y points to back
    current = current.iter().map(|x| x.rotate_y().rotate_y()).collect();
    let (found, pos) = check_all_z_rotations_hints(&mut current, scans, first, second, pos1, pos2);
    if found {
        return (true, pos);
    }

    //check all point pairs of both for same distance
    //if match is at least of size 12 then merge the vecs
    (false, Position { x: 0, y: 0, z: 0 })
}

fn check_all_z_rotations_hints(
    current: &mut Vec<Position>,
    scans: &mut Vec<Vec<Position>>,
    first: usize,
    second: usize,
    pos1: usize,
    pos2: usize,
) -> (bool, Position) {
    for _ in 0..=3 {
        *current = current.iter().map(|x| x.rotate_z()).collect();

        //println!("{:?}", current);
        let (success, positions, p) =
            check_match_with_hints(&mut scans[first], &current, pos1, pos2);
        if success {
            scans[first].extend(positions);
            scans[first].sort();
            scans[first].dedup();
            scans.remove(second);
            return (true, p);
        }
    }
    (false, Position { x: 0, y: 0, z: 0 })
}

fn check_match(
    first: &mut Vec<Position>,
    second: &Vec<Position>,
) -> (bool, Vec<Position>, Position) {
    let first_copy = first.clone();
    let second_copy = second.clone();
    for p1 in first {
        let dists: HashSet<Position> = first_copy.iter().map(|p| p1.dist_pos(*p)).collect();
        for p2 in &*second {
            let dists2: HashSet<Position> = second_copy.iter().map(|p| p2.dist_pos(*p)).collect();
            let tmp = dists.intersection(&dists2).count();
            if tmp >= 12 {
                let rel_position = p1.dist_pos(*p2);
                let dist_pos = Position {
                    x: p1.x - p2.x,
                    y: p1.y - p2.y,
                    z: p1.z - p2.z,
                };
                let to_add = second_copy
                    .iter()
                    .map(|x| Position {
                        x: x.x + rel_position.x,
                        y: x.y + rel_position.y,
                        z: x.z + rel_position.z,
                    })
                    .collect();
                return (true, to_add, dist_pos);
            }
        }
    }
    (false, vec![], Position { x: 0, y: 0, z: 0 })
}

fn check_match_with_hints(
    first: &mut Vec<Position>,
    second: &Vec<Position>,
    pos1: usize,
    pos2: usize,
) -> (bool, Vec<Position>, Position) {
    let p1 = first[pos1];
    let p2 = second[pos2];
    let dists: HashSet<Position> = first.iter().map(|p| p1.dist_pos(*p)).collect();

    let dists2: HashSet<Position> = second.iter().map(|p| p2.dist_pos(*p)).collect();
    let tmp = dists.intersection(&dists2).count();
    if tmp >= 12 {
        let rel_position = p1.dist_pos(p2);
        let dist_pos = Position {
            x: p1.x - p2.x,
            y: p1.y - p2.y,
            z: p1.z - p2.z,
        };
        let to_add = second
            .iter()
            .map(|x| Position {
                x: x.x + rel_position.x,
                y: x.y + rel_position.y,
                z: x.z + rel_position.z,
            })
            .collect();
        return (true, to_add, dist_pos);
    }

    (false, vec![], Position { x: 0, y: 0, z: 0 })
}

pub fn part1(path: &str) -> Result<usize> {
    let mut scans = parse_input(path)?;

    let mut scanner_positions: Vec<Position> = vec![Position { x: 0, y: 0, z: 0 }];
    'outer: loop {
        println!("{}", scans.len());
        for second in 1..scans.len() {
            for pos1 in 0..scans[0].len() {
                let p1 = scans[0][pos1];
                let dists: HashSet<u32> = scans[0].iter().map(|p| p1.dist(*p)).collect();
                for pos2 in 0..scans[second].len() {
                    let p2 = scans[second][pos2];
                    let dists2: HashSet<u32> = scans[second].iter().map(|p| p2.dist(*p)).collect();
                    if dists.intersection(&dists2).count() >= 12 {
                        //potential corresponding pattern
                        let (found, pos) = merge_scanners_hints(0, second, &mut scans, pos1, pos2);
                        if found == true {
                            scanner_positions.push(pos);
                            continue 'outer;
                        }
                    }
                }
            }
            let (found, pos) = merge_scanners(0, second, &mut scans);
            if found == true {
                scanner_positions.push(pos);
                continue 'outer;
            }
        }
        break;
    }
    /*for first in 0..scans.len() {
        for second in first + 1..scans.len() {
            for p1 in &scans[first] {
                let dists: HashSet<u32> = scans[first].iter().map(|p| p1.dist(*p)).collect();
                for p2 in &scans[second] {
                    let dists2: HashSet<u32> = scans[second].iter().map(|p| p2.dist(*p)).collect();
                    if dists.intersection(&dists2).count() >= 12 {
                        //potential corresponding pattern
                        //println!("{}, {}", first, second);
                        println!("found");
                        res -= 1;
                    }
                }
            }
        }
    }
    let mut scanner_positions: Vec<Position> = vec![Position { x: 0, y: 0, z: 0 }];
    'outer: loop {
        println!("{}", scans.len());
        for second in 1..scans.len() {
            let (found, pos) = merge_scanners(0, second, &mut scans);
            if found == true {
                scanner_positions.push(pos);
                continue 'outer;
            }
        }
        break;
    }*/
    let mut max_dist = 0;
    for i in 0..scanner_positions.len() {
        for j in i + 1..scanner_positions.len() {
            let p1 = scanner_positions[i];
            let p2 = scanner_positions[j];
            let md = (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs();
            if md > max_dist {
                max_dist = md;
            }
        }
    }
    //println!("{:?}", scanner_positions);
    println!("max manhattan dist: {}", max_dist);
    let res = scans.iter().map(|x| x.len()).sum();
    Ok(res)
}

pub fn part2(path: &str) -> Result<i32> {
    let result = 0;

    Ok(result)
}
