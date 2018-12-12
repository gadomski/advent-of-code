use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/12.txt");
    println!("Part 1: {}", sum_of_pots_with_plants(input, 20)?);
    Ok(())
}

fn sum_of_pots_with_plants(input: &str, generations: usize) -> Result<i64, Error> {
    let game: Game = input.parse()?;
    unimplemented!()
}

#[derive(Debug)]
struct Game {
    state: State,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct State {}

#[derive(Debug)]
struct Rule {}

#[derive(Debug)]
enum Error {
    MissingState(String),
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Game, Error> {
        let mut lines = s.lines();
        let state: State = lines
            .next()
            .ok_or(Error::MissingState(s.to_string()))
            .and_then(|line| line.parse())?;
        unimplemented!()
    }
}

impl FromStr for State {
    type Err = Error;
    fn from_str(s: &str) -> Result<State, Error> {
        unimplemented!()
    }
}

#[test]
fn part_1() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
    assert_eq!(325, sum_of_pots_with_plants(input, 20).unwrap());
}
