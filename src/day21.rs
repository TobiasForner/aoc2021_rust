use anyhow::Result;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Result<(usize, usize)> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let p1: usize = lines
        .next()
        .unwrap()?
        .replace("Player 1 starting position: ", "")
        .parse()?;
    let p2: usize = lines
        .next()
        .unwrap()?
        .replace("Player 2 starting position: ", "")
        .parse()?;

    Ok((p1, p2))
}

struct DeterministicD100 {
    count: usize,
}

impl DeterministicD100 {
    fn roll(&mut self) -> usize {
        let res = self.count;
        if res == 100 {
            self.count = 1;
        } else {
            self.count += 1;
        }
        res
    }
}

pub fn part1(path: &str) -> Result<usize> {
    let (mut p1, mut p2) = parse_input(path)?;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut dice = DeterministicD100 { count: 1 };
    let mut roll_count: usize = 0;
    let res;
    loop {
        let steps = (dice.roll() + dice.roll() + dice.roll()) % 10;
        roll_count += 3;
        p1 = p1 + steps;
        if p1 > 10 {
            p1 -= 10;
        }
        score1 += p1;
        if score1 >= 1000 {
            res = score2 * roll_count;
            break;
        }

        let steps = (dice.roll() + dice.roll() + dice.roll()) % 10;
        roll_count += 3;
        p2 = p2 + steps;
        if p2 > 10 {
            p2 -= 10;
        }
        score2 += p2;
        if score2 >= 1000 {
            res = score1 * roll_count;
            break;
        }
    }
    Ok(res)
}

pub fn part2(path: &str) -> Result<usize> {
    let (p1, p2) = parse_input(path)?;
    // maps score1, score 2, pos1, pos2 to number of universes that p1, p2 win in
    let mut lookup: HashMap<(usize, usize, usize, usize), (usize, usize)> = HashMap::new();
    let steps_counts: Vec<(usize, usize)> =
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    for score2 in 0..=20 {
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                lookup.insert((20, score2, pos1, pos2), (27, 0));
            }
        }
    }
    for score1 in (0..=19).rev() {
        for score2 in (0..=20).rev() {
            for pos1 in 1..=10 {
                for pos2 in 1..=10 {
                    let mut wins1 = 0;
                    let mut wins2 = 0;
                    for (steps, count) in &steps_counts {
                        let mut new_pos = pos1 + steps;
                        if new_pos > 10 {
                            new_pos -= 10;
                        }
                        let new_s1 = score1 + new_pos;
                        if new_s1 >= 21 {
                            wins1 += count;
                        } else {
                            for (steps2, count2) in &steps_counts {
                                let mut new_pos2 = pos2 + steps2;
                                if new_pos2 > 10 {
                                    new_pos2 -= 10;
                                }
                                let new_s2 = score2 + new_pos2;
                                if new_s2 >= 21 {
                                    wins2 += count * count2;
                                } else {
                                    let key = &(new_s1, new_s2, new_pos, new_pos2);
                                    if !lookup.contains_key(key) {
                                        println!("not found: {:?}", *key);
                                    }
                                    let (w1, w2) = lookup[key];
                                    wins1 += count * count2 * w1;
                                    wins2 += count * count2 * w2;
                                }
                            }
                        }
                    }
                    lookup.insert((score1, score2, pos1, pos2), (wins1, wins2));
                }
            }
        }
    }
    let (c1, c2) = lookup[&(0, 0, p1, p2)];
    Ok(c1.max(c2))
}
