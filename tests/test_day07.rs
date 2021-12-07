use aoc2021_rust::standard_tests;

//standard_tests!(day07, 7, 37, 333755, 168, 94017638);
standard_tests!(
    day07,
    7,
    test_part1 = 37,
    part1 = 333755,
    test_part2 = 168,
    part2 = 94017638
);

/*use anyhow::Result;
use aoc2021_rust::day07::{part1, part2};

#[test]
fn test_part1() -> Result<()> {
    assert_eq!(part1("inputs/day07_test.txt")?, 37);
    assert_eq!(part1("inputs/day07.txt")?, 333755);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    assert_eq!(part2("inputs/day07_test.txt")?, 168);
    assert_eq!(part2("inputs/day07.txt")?, 94017638);
    Ok(())
}*/
