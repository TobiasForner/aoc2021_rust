use anyhow::Result;
use aoc2021_rust::day03::{oxygen_gen_rating, part1, part2, scrubber_rating};
use aoc2021_rust::util::read_to_vec;

#[test]
fn test_part1() -> Result<()> {
    assert_eq!(part1("inputs/day03_test.txt")?, 198);
    assert_eq!(part1("inputs/day03.txt")?, 3901196);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    assert_eq!(part2("inputs/day03_test.txt")?, 230);
    assert_eq!(part2("inputs/day03.txt")?, 4412188);
    Ok(())
}

#[test]
fn test_oxygen() {
    if let Ok(input_vec) = read_to_vec("./inputs/day03_test.txt") {
        let res = oxygen_gen_rating(input_vec);
        assert_eq!(res, 23);
    }
}
#[test]
fn test_scrubber() {
    if let Ok(input_vec) = read_to_vec("./inputs/day03_test.txt") {
        let res = scrubber_rating(input_vec);
        assert_eq!(res, 10);
    }
}
