use crate::util::read_to_vec;
use anyhow::{Error, Result};
use std::cmp;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
enum PairNumber {
    Nested(Box<PairNumber>, Box<PairNumber>),
    Single(u32),
}

impl FromStr for PairNumber {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let chars: Vec<char> = s.chars().collect();
        fn build_from_pos<'a>(pos: usize, chars: &Vec<char>) -> (usize, PairNumber) {
            //println!("pos {}", pos);
            use PairNumber::*;
            if chars[pos + 1] == '[' {
                let (left_end, left) = build_from_pos(pos + 1, chars);
                if chars[left_end + 2] == '[' {
                    let (right_end, right) = build_from_pos(left_end + 2, chars);
                    (right_end + 1, Nested(Box::new(left), Box::new(right)))
                } else {
                    let right = chars[left_end + 2].to_digit(10).unwrap();
                    (
                        left_end + 3,
                        Nested(Box::new(left), Box::new(Single(right))),
                    )
                }
            } else if chars[pos + 1] == ']' || chars[pos + 1] == ',' {
                panic!("Invalid char at start of char sequence!");
            } else {
                //left is a number
                let left = chars[pos + 1].to_digit(10).unwrap();
                if chars[pos + 3] == '[' {
                    let (right_end, right) = build_from_pos(pos + 3, chars);
                    (
                        right_end + 1,
                        Nested(Box::new(Single(left)), Box::new(right)),
                    )
                } else {
                    let right = chars[pos + 3].to_digit(10).unwrap();
                    (
                        pos + 4,
                        Nested(Box::new(Single(left)), Box::new(Single(right))),
                    )
                }
            }
        }

        let (_, nested_number_pairs) = build_from_pos(0, &chars);
        Ok(nested_number_pairs)
    }
}

impl Add for PairNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let unreduced = PairNumber::Nested(Box::new(self), Box::new(other));
        unreduced.reduce()
    }
}

impl PairNumber {
    fn reduce(self) -> Self {
        let mut res = self;
        loop {
            let (found, _, _, _, _, new_res) = res.reduce_explode(0);
            res = new_res;
            if found {
                continue;
            }
            let (found, new_res) = res.reduce_split();
            res = new_res;
            if !found {
                break;
            }
        }
        //println!("{:?}", res);
        res
    }

    fn reduce_explode(self, level: u32) -> (bool, bool, bool, u32, u32, PairNumber) {
        use PairNumber::*;
        //(found, left_done, right_done, left, right, self)
        match self {
            Nested(x, y) => {
                let x = *x;
                let y = *y;
                match (&x, &y) {
                    (Single(v1), Single(v2)) => {
                        if level >= 4 {
                            (true, false, false, *v1, *v2, Single(0))
                        } else {
                            (
                                false,
                                false,
                                false,
                                0,
                                0,
                                Nested(Box::new(Single(*v1)), Box::new(Single(*v2))),
                            )
                        }
                    }
                    (Single(v1), Nested(_, _)) => {
                        let (found, left_done, right_done, left, right, new_right) =
                            y.reduce_explode(level + 1);
                        if found && !left_done {
                            (
                                found,
                                true,
                                right_done,
                                left,
                                right,
                                Nested(Box::new(Single(v1 + left)), Box::new(new_right)),
                            )
                        } else {
                            (
                                found,
                                left_done,
                                right_done,
                                left,
                                right,
                                Nested(Box::new(Single(*v1)), Box::new(new_right)),
                            )
                        }
                    }
                    (Nested(_, _), Single(v2)) => {
                        let (found, left_done, right_done, left, right, new_left) =
                            x.reduce_explode(level + 1);
                        if found && !right_done {
                            (
                                found,
                                left_done,
                                true,
                                left,
                                right,
                                Nested(Box::new(new_left), Box::new(Single(v2 + right))),
                            )
                        } else {
                            (
                                found,
                                left_done,
                                right_done,
                                left,
                                right,
                                Nested(Box::new(new_left), Box::new(Single(*v2))),
                            )
                        }
                    }
                    (Nested(_, _), Nested(_, _)) => {
                        let (found, left_done, right_done, left, right, mut new_left) =
                            x.reduce_explode(level + 1);
                        let mut new_right = y;
                        if found {
                            if !right_done {
                                new_right = new_right.add_left(right);
                            }
                            (
                                found,
                                left_done,
                                true,
                                left,
                                right,
                                Nested(Box::new(new_left), Box::new(new_right)),
                            )
                        } else {
                            let (found, left_done, right_done, left, right, new_right) =
                                new_right.reduce_explode(level + 1);
                            if !left_done {
                                new_left = new_left.add_right(left);
                            }
                            (
                                found,
                                true,
                                right_done,
                                left,
                                right,
                                Nested(Box::new(new_left), Box::new(new_right)),
                            )
                        }
                    }
                }
            }
            Single(_) => (false, false, false, 0, 0, self),
        }
    }

    fn reduce_split(self) -> (bool, Self) {
        use PairNumber::*;
        match self {
            Nested(x, y) => {
                let (found, new_left) = x.reduce_split();
                if found {
                    (found, Nested(Box::new(new_left), y))
                } else {
                    let (found, new_right) = y.reduce_split();
                    (found, Nested(Box::new(new_left), Box::new(new_right)))
                }
            }
            Single(v) => {
                if v >= 10 {
                    let half = (v as f32) / 2 as f32;
                    (
                        true,
                        Nested(
                            Box::new(Single(half.floor() as u32)),
                            Box::new(Single(half.ceil() as u32)),
                        ),
                    )
                } else {
                    (false, Single(v))
                }
            }
        }
    }

    fn add_right(self, val: u32) -> Self {
        use PairNumber::*;
        match self {
            Nested(x, y) => Nested(x, Box::new(y.add_right(val))),
            Single(v) => Single(v + val),
        }
    }

    fn add_left(self, val: u32) -> Self {
        use PairNumber::*;
        match self {
            Nested(x, y) => Nested(Box::new(x.add_left(val)), y),
            Single(v) => Single(v + val),
        }
    }

    fn magnitude(&self) -> u32 {
        use PairNumber::*;
        match self {
            Nested(x, y) => 3 * x.magnitude() + 2 * y.magnitude(),
            Single(v) => *v,
        }
    }
}

pub fn part1(path: &str) -> Result<u32> {
    let nested_number_pairs: Vec<PairNumber> = read_to_vec(path)?;
    let res = nested_number_pairs
        .into_iter()
        .reduce(|x, y| x + y)
        .unwrap()
        .magnitude();

    Ok(res)
}

fn largest_sum(path: &str, reverse: bool) -> Result<u32> {
    let nested_number_pairs: Vec<PairNumber> = read_to_vec(path)?;
    let max_len = nested_number_pairs.len();
    let mut best = 0;
    for first in 0..max_len {
        for second in first..max_len - 1 {
            let mut nested_number_pairs: Vec<PairNumber> = read_to_vec(path)?;
            let x = nested_number_pairs.remove(first);
            let y = nested_number_pairs.remove(second);
            let res = if reverse {
                (y + x).magnitude()
            } else {
                (x + y).magnitude()
            };
            if res > best {
                best = res;
            }
        }
    }
    Ok(best)
}
pub fn part2(path: &str) -> Result<u32> {
    let x = largest_sum(path, false)?;
    let y = largest_sum(path, true)?;

    Ok(cmp::max(x, y))
}
