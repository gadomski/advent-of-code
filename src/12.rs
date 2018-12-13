use std::collections::HashSet;
use std::fmt;
use std::slice::Iter;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/12.txt");
    println!("Part 1: {}", sum_of_pots_with_plants(input, 20)?);
    println!("Part 2: {}", sum_of_pots_with_plants(input, 50000000000)?);
    Ok(())
}

fn sum_of_pots_with_plants(input: &str, generations: usize) -> Result<i64, Error> {
    let mut game: Game = input.parse()?;
    for _ in 0..generations {
        game.advance();
    }
    Ok(game.sum_of_pots_with_plants())
}

#[derive(Debug)]
struct Game {
    generation: u64,
    state: State,
    rules: Rules,
}

#[derive(Debug)]
struct State {
    pots_with_plants: HashSet<i64>,
    stable: bool,
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
    input_len: i64,
}

#[derive(Clone, Debug)]
struct Rule {
    input: Vec<bool>,
    output: bool,
}

#[derive(Debug)]
enum Error {
    InvalidState(String),
    MissingState(String),
    NoInput(String),
    NoNewline(String),
    NoOutput(String),
    NoRules,
    EvenInputLength(Rule),
    UnequalRuleInputLength(Vec<Rule>),
}

impl Game {
    fn advance(&mut self) {
        self.state.advance(&self.rules);
        self.generation += 1;
    }

    fn sum_of_pots_with_plants(&self) -> i64 {
        self.state.pots_with_plants.iter().sum()
    }
}

impl State {
    fn new(pots_with_plants: HashSet<i64>) -> State {
        State {
            pots_with_plants: pots_with_plants,
            stable: false,
        }
    }

    fn min(&self) -> i64 {
        self.pots_with_plants.iter().map(|&n| n).min().unwrap_or(0)
    }

    fn max(&self) -> i64 {
        self.pots_with_plants.iter().map(|&n| n).max().unwrap_or(0)
    }

    fn min_with_buffer(&self, rules: &Rules) -> i64 {
        self.min() - rules.input_len() / 2
    }

    fn max_with_buffer(&self, rules: &Rules) -> i64 {
        self.max() + rules.input_len() / 2
    }

    fn advance(&mut self, rules: &Rules) {
        if self.stable {
            return;
        }
        let mut pots_with_plants = HashSet::new();
        for pot in self.min_with_buffer(rules)..=self.max_with_buffer(rules) {
            for rule in rules.iter() {
                if let Some(output) = rule.match_(pot, &self.pots_with_plants) {
                    if output {
                        pots_with_plants.insert(pot);
                    }
                }
            }
        }
        if pots_with_plants == self.pots_with_plants {
            self.stable = true;
        } else {
            self.pots_with_plants = pots_with_plants;
        }
    }
}

impl Rules {
    fn new(rules: Vec<Rule>) -> Result<Rules, Error> {
        let input_len = if let Some(first_rule) = rules.get(0) {
            let input_len = first_rule.input_len();
            if input_len % 2 == 0 {
                return Err(Error::EvenInputLength(first_rule.clone()));
            }
            if rules.iter().any(|rule| rule.input_len() != input_len) {
                return Err(Error::UnequalRuleInputLength(rules.clone()));
            }
            input_len
        } else {
            return Err(Error::NoRules);
        };
        Ok(Rules {
            rules: rules,
            input_len: input_len as i64,
        })
    }

    fn iter(&self) -> Iter<Rule> {
        self.rules.iter()
    }

    fn input_len(&self) -> i64 {
        self.input_len
    }
}

impl Rule {
    fn input_len(&self) -> usize {
        self.input.len()
    }

    fn match_(&self, pot: i64, pots_with_plants: &HashSet<i64>) -> Option<bool> {
        if self
            .input
            .iter()
            .enumerate()
            .map(|(i, expected)| (i as i64 - self.input.len() as i64 / 2, expected))
            .all(|(delta, &expected)| pots_with_plants.contains(&(pot + delta)) == expected)
        {
            Some(self.output)
        } else {
            None
        }
    }
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Game, Error> {
        let mut lines = s.lines();
        let state: State = lines
            .next()
            .ok_or(Error::MissingState(s.to_string()))
            .and_then(|line| line.parse())?;
        lines.next().ok_or(Error::NoNewline(s.to_string()))?;
        let rules = lines
            .map(|line| line.parse::<Rule>())
            .collect::<Result<Vec<Rule>, Error>>()
            .and_then(Rules::new)?;
        Ok(Game {
            generation: 0,
            state: state,
            rules: rules,
        })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min = self.state.min();
        let max = self.state.max();
        write!(f, "{} ({}..={}): ", self.generation, min, max)?;
        for n in min..=max {
            if self.state.pots_with_plants.contains(&n) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

impl FromStr for State {
    type Err = Error;
    fn from_str(s: &str) -> Result<State, Error> {
        let s = s
            .split(' ')
            .nth(2)
            .ok_or(Error::InvalidState(s.to_string()))?;
        let mut pots_with_plants = HashSet::new();
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                pots_with_plants.insert(i as i64);
            }
        }
        Ok(State::new(pots_with_plants))
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &b in &self.input {
            if b {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, " => {}", if self.output { "#" } else { "." })
    }
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Rule, Error> {
        let mut iter = s.split(" => ");
        let input = iter
            .next()
            .ok_or(Error::NoInput(s.to_string()))
            .map(|input| input.chars().map(|c| c == '#').collect::<Vec<bool>>())?;
        let output = iter
            .next()
            .ok_or(Error::NoOutput(s.to_string()))
            .map(|s| s == "#")?;
        Ok(Rule {
            input: input,
            output: output,
        })
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
