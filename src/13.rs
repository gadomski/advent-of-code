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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    direction: Direction,
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Error {
    UnknownPiece(char, Location),
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Map, Error> {
        let mut track = HashMap::new();
        let mut carts = Vec::new();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let location = Location { x: col, y: row };
                let piece = match c {
                    '|' => Some(Piece::Vertical),
                    '-' => Some(Piece::Horizontal),
                    '\\' => Some(Piece::LeftCurve),
                    '/' => Some(Piece::RightCurve),
                    '+' => Some(Piece::Intersection),
                    ' ' => None,
                    '>' => {
                        carts.push(Cart::new(location, Direction::Right));
                        Some(Piece::Horizontal)
                    }
                    'v' => {
                        carts.push(Cart::new(location, Direction::Down));
                        Some(Piece::Vertical)
                    }
                    '<' => {
                        carts.push(Cart::new(location, Direction::Left));
                        Some(Piece::Horizontal)
                    }
                    '^' => {
                        carts.push(Cart::new(location, Direction::Up));
                        Some(Piece::Vertical)
                    }
                    _ => return Err(Error::UnknownPiece(c, location)),
                };
                if let Some(piece) = piece {
                    track.insert(location, piece);
                }
            }
        }
        Ok(Map {
            track: track,
            carts: carts,
        })
    }
}

impl Cart {
    fn new(location: Location, direction: Direction) -> Cart {
        Cart {
            location: location,
            direction: direction,
            next_turn_direction: Turn::Left,
        }
    }
}

#[test]
fn part_1() {
    let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
    assert_eq!("7,3", first_crash(input).unwrap());
}
