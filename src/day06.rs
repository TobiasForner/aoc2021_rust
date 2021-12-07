use crate::util::parse_first_to_vec;
use anyhow::Result;

pub fn part1(path: &str) -> Result<i64> {
    let mut state = parse_state(path)?;
    //print_result!(6, 1, simulate(&mut state, 80)?);
    simulate(&mut state, 80)
}

pub fn part2(path: &str) -> Result<i64> {
    let mut state = parse_state(path)?;
    simulate(&mut state, 256)
}

pub fn parse_state(path: &str) -> Result<Vec<i64>> {
    let (input, _) = parse_first_to_vec::<usize>(path, ",")?;
    let mut counts = vec![0; 9];
    for x in input {
        counts[x] += 1;
    }
    Ok(counts)
}

pub fn simulate(state: &mut Vec<i64>, turns: u32) -> Result<i64> {
    for _ in 0..turns {
        let zeros = state[0];
        for pos in 1..=8 {
            state[pos - 1] = state[pos];
        }
        state[6] += zeros;
        state[8] = zeros;
    }
    Ok(state.iter().sum())
}
