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
struct Tracks {}

#[derive(Debug)]
enum Error {}

impl FromStr for Tracks {
    type Err = Error;
    fn from_str(s: &str) -> Result<Tracks, Error> {
        Ok(Tracks {})
    }
}

impl fmt::Display for Tracks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tracks!")
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
