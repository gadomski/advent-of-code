use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/13.txt");
    println!("Part 1: {}", first_crash(input)?);
    Ok(())
}

fn first_crash(input: &str) -> Result<String, Error> {
    let tracks: Tracks = input.parse()?;
    println!("{}", tracks);
    unimplemented!()
}

#[derive(Debug)]
struct Tracks {
    tracks: HashMap<Location, Track>,
}

#[derive(Debug)]
struct Track {
    track_type: TrackType,
    cart: Option<Cart>,
}

#[derive(Debug)]
enum TrackType {
    Vertical,
    Horizontal,
    Slash,
    Backslash,
    Intersection,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cart {}

#[derive(Debug)]
enum Error {}

impl FromStr for Tracks {
    type Err = Error;
    fn from_str(s: &str) -> Result<Tracks, Error> {
        let mut tracks = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if let Some(track) = Track::from_char(c) {
                    tracks.insert(Location::new(col, row), track);
                }
            }
        }
        Ok(Tracks { tracks: tracks })
    }
}

impl fmt::Display for Tracks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.tracks.keys().map(|location| location.y).max().unwrap() {
            for x in 0..=self.tracks.keys().map(|location| location.x).max().unwrap() {
                if let Some(track) = self.tracks.get(&Location::new(x, y)) {
                    write!(f, "{}", track)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Track {
    fn from_char(c: char) -> Option<Track> {
        use TrackType::{Backslash, Horizontal, Intersection, Slash, Vertical};
        let mut cart: Option<Cart> = None;
        let track_type = match c {
            '|' => Vertical,
            '-' => Horizontal,
            '/' => Slash,
            '\\' => Backslash,
            '+' => Intersection,
            ' ' => return None,
            _ => return None,
        };
        Some(Track {
            track_type: track_type,
            cart: cart,
        })
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TrackType::{Backslash, Horizontal, Intersection, Slash, Vertical};
        write!(
            f,
            "{}",
            match self.track_type {
                Vertical => '|',
                Horizontal => '-',
                Slash => '/',
                Backslash => '\\',
                Intersection => '+',
            }
        )
    }
}

impl Location {
    fn new(x: usize, y: usize) -> Location {
        Location { x: x, y: y }
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
