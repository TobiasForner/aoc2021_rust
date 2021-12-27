use anyhow::{Error, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse_input(path: &str) -> Result<Vec<Block>> {
    let mut blocks = vec![];
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let lines = br.lines();
    let mut current = Block::new();
    for line in lines {
        let line = line?;
        let ch = line.chars().next().unwrap();
        if ch == 'i' {
            if !current.ops.is_empty() {
                blocks.push(current);
            }
            current = Block::new();
            continue;
        }
        current.add_op(line.parse()?);
    }
    blocks.push(current);
    Ok(blocks)
}

#[derive(Debug)]
struct Block {
    ops: Vec<Operation>,
}

impl Block {
    fn eval(&self, z: i64, w: i64) -> i64 {
        let mut vars = vec![0; 4];
        vars[0] = w;
        vars[3] = z;
        for op in &self.ops {
            op.apply(&mut vars);
        }
        vars[3]
    }
    fn new() -> Self {
        Block { ops: vec![] }
    }
    fn add_op(&mut self, op: Operation) {
        self.ops.push(op);
    }
}

#[derive(Debug)]
struct Operation {
    first: usize,
    second: Arg,
    typ: OpType,
}

#[derive(Copy, Clone, Debug)]
enum OpType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use OpType::*;
        let parts: Vec<&str> = s.split(" ").collect();
        let parse_var = |x| match x {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!("Invalid var"),
        };
        let first = parse_var(parts[1]);
        let second: Arg = parts[2].parse()?;
        let typ = match parts[0] {
            "inp" => panic!("input operation should not be parsed!"),
            "add" => Add,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "eql" => Eql,
            _ => panic!("Invalid op {}", parts[0]),
        };
        Ok(Operation { first, second, typ })
    }
}

#[derive(Debug)]
enum Arg {
    Var(usize),
    Lit(i64),
}

impl Operation {
    fn apply(&self, vars: &mut Vec<i64>) {
        use Arg::*;
        use OpType::*;
        let a = self.first;
        let second = match self.second {
            Var(c) => vars[c],
            Lit(x) => x,
        };
        match self.typ {
            Add => {
                vars[a] += second;
            }
            Mul => {
                vars[a] *= second;
            }
            Div => {
                vars[a] /= second;
            }
            Mod => {
                vars[a] = vars[a] % second;
            }
            Eql => {
                if vars[a] == second {
                    vars[a] = 1;
                } else {
                    vars[a] = 0;
                }
            }
        }
    }
}

impl FromStr for Arg {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use Arg::*;
        let res = match s {
            "w" => Var(0),
            "x" => Var(1),
            "y" => Var(2),
            "z" => Var(3),
            _ => Lit(s.parse()?),
        };
        Ok(res)
    }
}

fn has_solution(z_values: &HashSet<i64>, blocks: &Vec<Block>, pos: usize) -> bool {
    let mut current_states = z_values.clone();
    for p in pos..=13 {
        let mut new_states = HashSet::new();
        for w in 1..=9 {
            for z in &current_states {
                let new_z = blocks[p].eval(*z, w);

                new_states.insert(new_z);
            }
            /*if has_solution(&new_states, blocks, pos + 1) {
                println!("{} works", w);
                return true;
            }*/
        }
        current_states = new_states.clone();
    }
    return current_states.contains(&0);
}

#[allow(dead_code)]
fn is_accepted(inp: Vec<i64>, blocks: Vec<Block>) -> bool {
    let mut z = 0;
    for (index, i) in inp.iter().enumerate() {
        z = blocks[index].eval(z, *i);
    }
    z == 0
}

pub fn part1(path: &str) -> Result<i64> {
    let blocks = parse_input(path)?;
    let mut current_states = HashSet::new();
    current_states.insert(0);
    let mut res = 0;
    'pos_loop: for pos in 0..=13 {
        for w in (1..=9).rev() {
            let mut new_states = HashSet::new();
            for z in current_states.clone() {
                let state = blocks[pos].eval(z, w);
                new_states.insert(state);
            }
            if pos >= 13 {
                if !new_states.contains(&0) {
                    continue;
                } else {
                    res += w;
                    return Ok(res);
                }
            } else if has_solution(&new_states, &blocks, pos + 1) {
                println!("There is a solution with {} at pos {}", w, pos);
                current_states = new_states;
                res += w * 10i64.pow(13 - pos as u32);
                continue 'pos_loop;
            } else if w == 1 {
                panic!("No solution from pos {}", pos);
            }
        }
    }
    Ok(res)
}

pub fn part2(path: &str) -> Result<i64> {
    /*for w in 1..=9 {
        let inp = vec![9, 7, 4, 9, 2, 9, 9, 9, 5, 9, 9, 9, 3, w];
        let blocks = parse_input(path)?;
        println!("{:?} :{}", inp.clone(), is_accepted(inp, blocks));
    }*/

    let blocks = parse_input(path)?;
    let mut current_states = HashSet::new();
    current_states.insert(0);
    let mut res = 0;
    'pos_loop: for pos in 0..=13 {
        for w in 1..=9 {
            let mut new_states = HashSet::new();
            for z in &current_states {
                let state = blocks[pos].eval(*z, w);
                new_states.insert(state);
            }
            if pos >= 13 {
                if !new_states.contains(&0) {
                    continue;
                } else {
                    res += w;
                    return Ok(res);
                }
            } else if has_solution(&new_states, &blocks, pos + 1) {
                println!("There is a solution with {} at pos {}", w, pos);
                current_states = new_states;
                res += w * 10i64.pow(13 - pos as u32);
                continue 'pos_loop;
            } else if w == 9 {
                panic!("No solution from pos {}", pos);
            }
        }
    }
    Ok(res)
}

/*pub fn part1_3(path: &str) -> Result<i64> {
    let blocks = parse_input(path)?;
    let vars = vec![0; 4];
    let mut current_states = HashSet::new();
    current_states.insert(vars);
    let mut count = 0;
    let mut blocks = blocks.iter();
    blocks.next();
    for block in blocks {
        count += 1;
        println!("block {}", count);
        let mut new_states = HashSet::new();
        for w in 1..=9 {
            for mut vars in current_states.clone() {
                let z = vars[3];
                if count == 14 && (z < 14 || z > 21 || w == 1) {
                    //found through prints below;
                    continue;
                }
                vars[0] = w;
                let mut state = block.eval(vars);
                if count == 14 && state[3] == 0 {
                    println!("end. z={}, w={}", z, w);
                }
                let new_z = state[3];
                if count == 13 && (new_z < 14 || new_z > 21) {
                    //found through prints below;
                    continue;
                } else if count == 13 {
                    println!("block 13. input z={}, input w={}", z, w);
                }
                state[0] = 0;
                state[1] = 0;
                state[2] = 0;
                new_states.insert(state);
            }
        }
        current_states = new_states;
    }
    let z_value = current_states.iter().map(|x| x[3]).min().unwrap();
    println!("z vals: {:?}", z_value);

    Ok(0)
}

fn block(a: i64, b: i64, w: i64, z: i64, z_div: i64) -> i64 {
    let mut z = z / z_div;
    z = if z % 26 + a == w { z } else { 26 * z + w + b };
    z
}

pub fn part1_old(_path: &str) -> Result<i64> {
    let blocks = vec![
        (11, 14, 1),
        (13, 8, 1),
        (11, 4, 1),
        (10, 10, 1),
        (-3, 14, 26),
        (-4, 10, 26),
        (12, 4, 1),
        (-8, 14, 26),
        (-3, 1, 26),
        (-12, 6, 26),
        (14, 0, 1),
        (-6, 9, 26),
        (11, 13, 1),
        (-12, 12, 26),
    ];
    /*let (a, b, z_div) = blocks.first().unwrap();
    for w in 1..=9 {
        println!("w:{}, z:{}", w, block(*a, *b, w, 0, *z_div));
    }*/
    let mut z_values = HashSet::new();
    z_values.insert(0);
    let mut count = 0;
    for (a, b, z_div) in blocks {
        count += 1;
        println!("block {}", count);
        let mut results = HashSet::new();
        for w in 1..=9 {
            for z in &z_values {
                let res = block(a, b, w, *z, z_div);
                if count == 14 && res == 0 {
                    println!("end w: {}", w);
                }
                results.insert(res);
                //println!("w:{}, z:{}", w, res);
            }
        }
        z_values = results;
    }
    println!("{}", z_values.iter().min().unwrap());
    //Ok(block(*a, *b, 9, 0, *z_div))
    Ok(0)
}*/
