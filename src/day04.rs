use crate::print_result;
use anyhow::{Context, Error};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Result<(Vec<u32>, Vec<Board>), Error> {
    let br = BufReader::new(File::open(path)?);
    let mut in_lines = br.lines();
    let num_seq = in_lines
        .next()
        .context("Expected at least one line")??
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<u32>, _>>()?;
    in_lines.next().context("Expected at least two lines")??;
    let mut boards = vec![];
    let mut current = Board::default();
    for line in in_lines {
        let line = line?;
        if line.is_empty() {
            boards.push(current);
            current = Board::default();
            continue;
        }
        current.add_line(
            line.split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| Some(x.parse()).transpose())
                .collect::<Result<Vec<Option<u32>>, _>>()?,
        );
    }
    Ok((num_seq, boards))
}

#[derive(Default, Clone)]
struct Board {
    entries: Vec<Vec<Option<u32>>>,
    won: bool,
}

impl Board {
    fn add_line(&mut self, line: Vec<Option<u32>>) {
        self.entries.push(line);
    }

    fn won(&mut self) -> bool {
        'col_loop: for col in 0..self.entries.len() {
            for row in 0..self.entries.len() {
                if self.entries[row][col].is_some() {
                    continue 'col_loop;
                }
            }
            self.won = true;
            return true;
        }
        self.won = self.entries.iter().any(|l| l.iter().all(|x| x.is_none()));
        self.won
    }

    fn remove_num(&mut self, num: u32) {
        for row in &mut self.entries {
            for entry in row {
                if entry == &Some(num) {
                    entry.take();
                }
            }
        }
        self.won();
    }

    fn score(&self) -> u32 {
        self.entries
            .iter()
            .flat_map(|r| r.iter().filter_map(|x| *x))
            .sum()
    }
}

pub fn part1() -> Result<(), Error> {
    let (num_vec, mut boards) = parse_input("inputs/day04.txt")?;
    let mut result = 0;
    'outer: for num in num_vec {
        for board in &mut boards {
            board.remove_num(num);
            if board.won {
                result = num * board.score();
                break 'outer;
            }
        }
    }

    print_result!(4, 1, result);
}

pub fn part2() -> Result<(), Error> {
    let (num_vec, boards) = parse_input("inputs/day04.txt")?;
    let mut result = 0;
    let mut queue: VecDeque<Board> = boards.into_iter().collect();
    let mut queue2: VecDeque<Board> = VecDeque::new();

    for num in num_vec {
        while let Some(mut board) = queue.pop_front() {
            board.remove_num(num);
            if board.won {
                result = num * board.score();
            } else {
                queue2.push_back(board)
            }
        }
        std::mem::swap(&mut queue, &mut queue2)
    }

    print_result!(4, 2, result);
}
