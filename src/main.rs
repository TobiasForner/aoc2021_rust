use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Read};
use std::path::Path;

fn parse_int_iter<R: Read>(io: R) -> Result<Iterator<item = u32>, Error> {
    let br = BufReader::new(io);
    br.lines().map(|line| line.and_then(|v| v.parse()))
}

fn day01_part2() {}

fn main() {
    let mut count: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/day01.txt") {
        let mut last: u32 = u32::MAX;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let my_int: u32 = ip.parse().unwrap();
                if my_int > last {
                    count += 1;
                }
                last = my_int;

                //println!("{}", ip);
            }
        }
    }
    println!("{}", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
