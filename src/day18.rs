use crate::util::read_to_vec;
use anyhow::{Error, Result};
use std::fmt::{self, Debug, Formatter};
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone)]
enum PairNumber {
    Nested(Box<PairNumber>, Box<PairNumber>),
    Single(u32),
}

impl FromStr for PairNumber {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let chars: Vec<char> = s.chars().collect();
        fn build_from_pos<'a>(pos: usize, chars: &Vec<char>) -> (usize, PairNumber) {
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

impl Debug for PairNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use PairNumber::*;
        match self {
            Single(v) => {
                write!(f, "{}", v)
            }
            Nested(l, r) => {
                write!(f, "[{:?}, {:?}]", l, r)
            }
        }
    }
}

impl Add for PairNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut unreduced = PairNumber::Nested(Box::new(self), Box::new(other));
        unreduced.reduce();
        unreduced
    }
}

impl PairNumber {
    fn reduce(&mut self) {
        loop {
            let (found, _, _, _, _) = self.reduce_explode(0);
            if found {
                continue;
            }
            if !self.reduce_split() {
                break;
            }
        }
    }

    fn reduce_explode(&mut self, level: u32) -> (bool, bool, bool, u32, u32) {
        use PairNumber::*;
        //(found, left_done, right_done, left_v, right_v)
        match self {
            Nested(x, y) => match (&**x, &**y) {
                (Single(v1), Single(v2)) => {
                    if level >= 4 {
                        let v1 = *v1;
                        let v2 = *v2;
                        *self = Single(0);
                        (true, false, false, v1, v2)
                    } else {
                        (false, false, false, 0, 0)
                    }
                }
                (Single(_), Nested(_, _)) => {
                    let (found, left_done, right_done, left, right) = y.reduce_explode(level + 1);
                    if found && !left_done {
                        x.add_right(left);
                        (found, true, right_done, left, right)
                    } else {
                        (found, left_done, right_done, left, right)
                    }
                }
                (Nested(_, _), Single(_)) => {
                    let (found, left_done, right_done, left, right) = x.reduce_explode(level + 1);

                    if found && !right_done {
                        y.add_left(right);
                        (found, left_done, true, left, right)
                    } else {
                        (found, left_done, right_done, left, right)
                    }
                }
                (Nested(_, _), Nested(_, _)) => {
                    let (found, left_done, right_done, left, right) = x.reduce_explode(level + 1);
                    if found {
                        if !right_done {
                            y.add_left(right);
                        }
                        (found, left_done, true, left, right)
                    } else {
                        let (found, left_done, right_done, left, right) =
                            y.reduce_explode(level + 1);
                        if found && !left_done {
                            x.add_right(left);
                            (found, true, right_done, left, right)
                        } else {
                            (found, left_done, right_done, left, right)
                        }
                    }
                }
            },
            Single(_) => (false, false, false, 0, 0),
        }
    }

    fn reduce_split(&mut self) -> bool {
        use PairNumber::*;
        match self {
            Nested(x, y) => x.reduce_split() || y.reduce_split(),
            Single(v) => {
                if *v >= 10 {
                    let half = (*v as f32) / 2 as f32;
                    *self = Nested(
                        Box::new(Single(half.floor() as u32)),
                        Box::new(Single(half.ceil() as u32)),
                    );
                    true
                } else {
                    false
                }
            }
        }
    }

    fn add_right(&mut self, val: u32) {
        use PairNumber::*;
        match self {
            Nested(_, y) => y.add_right(val),
            Single(v) => *v += val,
        }
    }

    fn add_left(&mut self, val: u32) {
        use PairNumber::*;
        match self {
            Nested(x, _) => x.add_left(val),
            Single(v) => *v += val,
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

fn largest_sum(path: &str) -> Result<u32> {
    let nums: Vec<PairNumber> = read_to_vec(path)?;
    let mut max = 0;
    nums.iter()
        .flat_map(|num| std::iter::repeat(num).zip(nums.iter()))
        .for_each(|(n1, n2)| {
            max = max.max(n1.clone().add(n2.clone()).magnitude());
            max = max.max(n2.clone().add(n1.clone()).magnitude());
        });
    Ok(max)
}
pub fn part2(path: &str) -> Result<u32> {
    let x = largest_sum(path)?;
    Ok(x)
}
