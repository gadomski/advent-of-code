#[macro_use]
extern crate failure;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input/03.txt");
    println!("Part 1: {}", overlap_count(input));
}

fn overlap_count(input: &str) -> usize {
    let claims: Vec<Claim> = input.lines().map(|line| line.parse().unwrap()).collect();
    let fabric = Fabric::new(&claims);
    fabric.0.values().filter(|ids| ids.len() > 1).count()
}

#[derive(Debug)]
struct Claim {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

#[derive(Debug)]
struct Fabric(HashMap<(u64, u64), Vec<u64>>);

#[derive(Debug, Fail)]
#[fail(display = "invalid claim format: {}", _0)]
struct InvalidClaimFormat(String);

impl Claim {
    fn x_coordinates(&self) -> Range<u64> {
        self.x..(self.x + self.width)
    }

    fn y_coordinates(&self) -> Range<u64> {
        self.y..(self.y + self.height)
    }
}

impl FromStr for Claim {
    type Err = InvalidClaimFormat;
    fn from_str(s: &str) -> Result<Claim, InvalidClaimFormat> {
        let regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let captures = regex.captures(s).ok_or(InvalidClaimFormat(s.to_string()))?;
        Ok(Claim {
            id: captures[1].to_string().parse().unwrap(),
            x: captures[2].to_string().parse().unwrap(),
            y: captures[3].to_string().parse().unwrap(),
            width: captures[4].to_string().parse().unwrap(),
            height: captures[5].to_string().parse().unwrap(),
        })
    }
}

impl Fabric {
    fn new(claims: &[Claim]) -> Fabric {
        let mut map = HashMap::new();
        for claim in claims {
            for x in claim.x_coordinates() {
                for y in claim.y_coordinates() {
                    let mut entry = map.entry((x, y)).or_insert_with(Vec::new);
                    entry.push(claim.id);
                }
            }
        }
        Fabric(map)
    }
}

#[test]
fn part_1() {
    let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
    assert_eq!(4, overlap_count(input));
}
