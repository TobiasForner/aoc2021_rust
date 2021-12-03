use aoc2021_rust::day03::{oxygen_gen_rating, scrubber_rating};
use aoc2021_rust::util::read_to_vec;

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
