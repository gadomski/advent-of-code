use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/13.txt");
    println!("Part 1: {}", first_crash(input)?);
    Ok(())
}

fn first_crash(input: &str) -> Result<String, Error> {
    let map: Map = input.parse()?;
    unimplemented!()
}

#[derive(Debug)]
struct Map {
    track: HashMap<Location, Piece>,
    carts: Vec<Cart>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Piece {
    Vertical,
    Horizontal,
    RightCurve,
    LeftCurve,
    Intersection,
}

#[derive(Debug)]
struct Cart {
    location: Location,
    next_turn_direction: Turn,
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
enum Error {}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Map, Error> {
        let mut track = HashMap::new();
        let mut carts = Vec::new();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    _ => unimplemented!(),
                }
            }
        }
        Ok(Map {
            track: track,
            carts: carts,
        })
    }
}

#[test]
fn part_1() {
    let input = r"/---\        
|   |  /----\
| /-+--+-\  |
| | |  X |  |
\-+-/  \-+--/
  \------/   ";
    assert_eq!("7,3", first_crash(input).unwrap());
}
