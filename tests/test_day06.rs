use anyhow::Result;
use aoc2021_rust::day06::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    assert_eq!(part1("inputs/day06_test.txt")?, 5934);
    assert_eq!(part1("inputs/day06.txt")?, 379414);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    assert_eq!(part2("inputs/day06_test.txt")?, 26984457539);
    assert_eq!(part2("inputs/day06.txt")?, 1705008653296);
    Ok(())
}
