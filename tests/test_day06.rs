use anyhow::Result;
use aoc2021_rust::day06::{parse_state, simulate};

#[test]
fn test_part1() -> Result<()> {
    let mut input = parse_state("inputs/day06_test.txt")?;
    assert_eq!(simulate(&mut input, 18)?, 26);
    let mut input = parse_state("inputs/day06_test.txt")?;
    assert_eq!(simulate(&mut input, 80)?, 5934);

    let mut input = parse_state("inputs/day06.txt")?;
    assert_eq!(simulate(&mut input, 80)?, 379414);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let mut input = parse_state("inputs/day06_test.txt")?;
    assert_eq!(simulate(&mut input, 256)?, 26984457539);

    let mut input = parse_state("inputs/day06.txt")?;
    assert_eq!(simulate(&mut input, 256)?, 1705008653296);
    Ok(())
}
