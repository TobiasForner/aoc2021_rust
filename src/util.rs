use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Lines, Read};

use std::fs::File;
use std::iter::Iterator;
use std::str::FromStr;

pub fn read_to_vec<T: FromStrCtx>(path: &str) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    parse_lines(File::open(path)?, &mut ())
}

pub fn read_to_vec_ctx<Ctx, T: FromStrCtx<Ctx>>(path: &str, c: &mut Ctx) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    parse_lines(File::open(path)?, c)
}

pub fn parse_first_to_vec<T: FromStr>(
    path: &str,
    sep: &str,
) -> Result<(Vec<T>, Lines<BufReader<File>>)>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(File::open(path)?);
    let mut in_lines = br.lines();
    let x = in_lines
        .next()
        .context("Expected at least one line")??
        .split(sep)
        .map(|x| x.parse())
        .collect::<Result<Vec<T>, _>>()?;
    Ok((x, in_lines))
}

fn parse_lines<Ctx, T: FromStrCtx<Ctx>, R: Read>(io: R, c: &mut Ctx) -> Result<Vec<T>>
where
    anyhow::Error: From<T::Err>,
{
    let br = BufReader::new(io);
    br.lines()
        .map(|line| Ok(T::from_str_ctx(&line?, c)?))
        .collect()
}

#[macro_export]
macro_rules! parse_input {
    ($path: expr, $vec1:expr, $type1:ident , $sep1: expr $(,$vec:expr, $type:ident, $sep: expr)*) => {{
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let io = File::open($path)?;
        let br = BufReader::new(io);
        let mut lines = br.lines();
        loop{
            if let Some(x)=lines.next(){
                let x=x?;
                if x==$sep1{
                    break;
                }
                $vec1.push($type1::from_str(&x)?);
            }else{break;}
        }
        $(
            loop{
                if let Some(x)=lines.next(){
                    let x=x?;
                    if x==$sep{
                        break;
                    }
                    $vec.push($type::from_str(&x)?);
                }else{break;}
            }
        )*
    }};
}

#[macro_export]
macro_rules! print_result {
    ($day: expr, $part: expr, $result: expr) => {
        println!("Day {} Part {}: {}", $day, $part, $result);
        return Ok(());
    };
}

/*pub enum ParseType {
    CharParse,
    MultilineCharParse(String),
    LineParseSingle,
    LineParseMultiple(String),
}

#[macro_export]
macro_rules! parse_input_v2 {
    ($path: expr, $($parse_type: expr, $typ: ty,)+) => {{
        use crate::util::ParseType::*;
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let io = File::open($path)?;
        let br = BufReader::new(io);
        let mut lines = br.lines();
        let mut res: ($(match $parse_type{
            CharParse=> Vec<$typ>
            MultilineCharParse=>Vec<Vec<$typ>>
        LineParseSingle=>$typ
    LineParseMultiple=>Vec<$typ>},)+);
        let mut count =0;
        $(let res1=match $parse_type {
            CharParse => {
                let line = lines.next().unwrap()?;
                let chars: Vec<$typ> = line.chars().collect();
                chars
            }
            MultilineCharParse(sep) => {
                let mut vec = vec![];
                const RADIX:u32=10;
                loop{
                    if let Some(x)=lines.next(){
                        let x=x?;
                        if x==sep{
                            break;
                        }
                        vec.push(x.chars()
                        .map(|y| y.to_digit(RADIX).unwrap() as $typ).collect::<Vec<&typ>>());
                    }else{break;}
                }
                vec
            }
            LineParseSingle => {
                let line = lines.next().unwrap()?;
                let res = line.parse::<$typ>();
                res
            }
            LineParseMultiple(sep) => {
                let mut vec = vec![];
                loop{
                    if let Some(x)=lines.next(){
                        let x=x?;
                        if x==sep{
                            break;
                        }
                        vec.push($typ::from_str(&x)?);
                    }else{break;}
                }
                vec
            }
        }
        res[count]=res1;
    count+=1;)+
    res
    };}
}*/

#[macro_export]
macro_rules! standard_tests {
    ($mod_name: ident, $day: expr, test_part1=$test_res_part1: expr$(, part1=$res_part1: expr)?$(, test_part2=$test_res_part2: expr$(, part2=$res_part2: expr)?)?) => {
        use anyhow::Result;
        use aoc2021_rust::$mod_name::{part1, part2};

        #[test]
        fn test_part1() -> Result<()> {
            assert_eq!(
                part1(&format!("inputs/day{:0>2}_test.txt", $day))?,
                $test_res_part1
            );
            $(assert_eq!(part1(&format!("inputs/day{:0>2}.txt", $day))?, $res_part1);)?
            Ok(())
        }

        $(#[test]
        fn test_part2() -> Result<()> {
            assert_eq!(
                part2(&format!("inputs/day{:0>2}_test.txt", $day))?,
                $test_res_part2
            );
            $(assert_eq!(part2(&format!("inputs/day{:0>2}.txt", $day))?, $res_part2);)?
            Ok(())
        })?
    };
}

pub trait FromStrCtx<Ctx = ()>: Sized {
    type Err;
    fn from_str_ctx(s: &str, c: &mut Ctx) -> Result<Self, Self::Err>;
}

impl<T: FromStr> FromStrCtx for T {
    type Err = T::Err;
    fn from_str_ctx(s: &str, _: &mut ()) -> Result<Self, Self::Err> {
        T::from_str(s)
    }
}
