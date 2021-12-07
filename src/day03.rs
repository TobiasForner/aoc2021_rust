use crate::util::read_to_vec;
use anyhow::Result;
use std::iter::Sum;
use std::str::FromStr;

impl FromStr for IntString {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let bits = s
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(IntString { bits })
    }
}

impl Sum for IntString {
    fn sum<I: Iterator<Item = IntString>>(mut iter: I) -> Self {
        if let Some(mut first) = iter.next() {
            for other in iter {
                for (index, x) in other.bits.iter().enumerate() {
                    first.bits[index] += x;
                }
            }
            first
        } else {
            IntString { bits: vec![] }
        }
    }
}

impl From<IntString> for usize {
    fn from(this: IntString) -> usize {
        let mut res = 0;
        for x in this.bits {
            res <<= 1;
            res += x;
        }
        res
    }
}
#[derive(Clone)]
pub struct IntString {
    bits: Vec<usize>,
}

pub fn part1(path: &str) -> Result<i32> {
    let input_vec: Vec<IntString> = read_to_vec(path)?;
    let vec_len = input_vec.len();
    let one_counts: IntString = input_vec.into_iter().sum();
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in one_counts.bits {
        gamma <<= 1;
        epsilon <<= 1;
        if i > vec_len / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    Ok(gamma * epsilon)
}

pub fn part2(path: &str) -> Result<usize> {
    let input_vec: Vec<IntString> = read_to_vec(path)?;
    let input_vec2 = input_vec.clone();
    let ox_gen_rating = oxygen_gen_rating(input_vec);
    let scrub_rating = scrubber_rating(input_vec2);
    Ok(ox_gen_rating * scrub_rating)
}

pub fn oxygen_gen_rating(vec: Vec<IntString>) -> usize {
    compute_rating(vec, 0, |x, y| x >= y)
}

pub fn scrubber_rating(vec: Vec<IntString>) -> usize {
    compute_rating(vec, 0, |x, y| x < y)
}

fn compute_rating<C: FnMut(usize, usize) -> bool>(
    mut vec: Vec<IntString>,
    pos: usize,
    mut cmp: C,
) -> usize {
    let vec_len = vec.len();
    if vec_len == 1 {
        vec.remove(0).into()
    } else {
        let (one_at_pos, zero_at_pos): (Vec<_>, _) =
            vec.into_iter().partition(|x| x.bits[pos] == 1);
        if cmp(one_at_pos.len(), zero_at_pos.len()) {
            compute_rating(one_at_pos, pos + 1, cmp)
        } else {
            compute_rating(zero_at_pos, pos + 1, cmp)
        }
    }
}
