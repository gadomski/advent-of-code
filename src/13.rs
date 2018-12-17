use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/13.txt");
    println!("Part 1: {}", first_crash(input)?);
    Ok(())
}

fn first_crash(input: &str) -> Result<String, Error> {
    let mut tracks: Tracks = input.parse()?;
    while !tracks.has_crash() {
        if let Err(err) = tracks.tick() {
            match err {
                Error::Crash(location) => return Ok(format!("{},{}", location.x, location.y)),
                _ => return Err(err),
            }
        }
    }
    unreachable!()
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cart {
    orientation: Orientation,
}

#[derive(Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Error {
    Crash(Location),
    InvalidTrack(char),
}

impl Tracks {
    fn has_crash(&self) -> bool {
        let mut cart_locations = HashSet::new();
        self.tracks
            .iter()
            .filter_map(|(location, track)| track.cart.as_ref().map(|_| location))
            .any(|location| !cart_locations.insert(location))
    }

    fn tick(&mut self) -> Result<(), Error> {
        for location in self.locations_with_carts() {
            panic!("need to advance {:?}", location);
        }
        Ok(())
    }

    fn locations_with_carts(&mut self) -> Vec<Location> {
        self.tracks
            .iter()
            .filter_map(|(&location, track)| {
                if track.cart.is_some() {
                    Some(location)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl FromStr for Tracks {
    type Err = Error;
    fn from_str(s: &str) -> Result<Tracks, Error> {
        let mut tracks = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if let Some(track) = Track::from_char(c)? {
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
    fn from_char(c: char) -> Result<Option<Track>, Error> {
        use TrackType::{Backslash, Horizontal, Intersection, Slash, Vertical};
        let mut cart: Option<Cart> = None;
        let track_type = match c {
            '|' => Vertical,
            '-' => Horizontal,
            '/' => Slash,
            '\\' => Backslash,
            '+' => Intersection,
            ' ' => return Ok(None),
            '>' | '<' => {
                cart = Some(Cart::new(c)?);
                Horizontal
            }
            '^' | 'v' => {
                cart = Some(Cart::new(c)?);
                Vertical
            }
            _ => return Err(Error::InvalidTrack(c)),
        };
        Ok(Some(Track {
            track_type: track_type,
            cart: cart,
        }))
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Orientation::{Down, Left, Right, Up};
        use TrackType::{Backslash, Horizontal, Intersection, Slash, Vertical};
        if let Some(cart) = self.cart.as_ref() {
            write!(
                f,
                "{}",
                match cart.orientation {
                    Up => '^',
                    Right => '>',
                    Down => 'v',
                    Left => '<',
                }
            )
        } else {
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
}

impl Cart {
    fn new(c: char) -> Result<Cart, Error> {
        use Orientation::{Down, Left, Right, Up};
        let orientation = match c {
            '>' => Right,
            'v' => Down,
            '<' => Left,
            '^' => Up,
            _ => return Err(Error::InvalidTrack(c)),
        };
        Ok(Cart {
            orientation: orientation,
        })
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
