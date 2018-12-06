#[macro_use]
extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/06.txt");
    println!("Part 1: {}", size_of_largest_area(input)?);
    Ok(())
}

fn size_of_largest_area(input: &str) -> Result<u64, Error> {
    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, Error>>()?;
    unimplemented!()
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid point format: {}", _0)]
struct InvalidPointFormat(String);

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point, Error> {
        let regex = Regex::new(r"^(\d+), (\d+)$")?;
        let captures = regex.captures(s).ok_or(InvalidPointFormat(s.to_string()))?;
        Ok(Point {
            x: captures[1].parse()?,
            y: captures[2].parse()?,
        })
    }
}

#[test]
fn part_1() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(17, size_of_largest_area(input).unwrap());
}
