#[macro_use]
extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/07.txt");
    println!("Part 1: {}", correct_order(input)?);
    println!("Part 2: {}", time_required(input, 5, 60)?);
    Ok(())
}

fn correct_order(input: &str) -> Result<String, Error> {
    let team = Team::from_input(input, 1, 0)?;
    Ok(team.order().to_string())
}

fn time_required(input: &str, workers: usize, seconds: i64) -> Result<i64, Error> {
    let team = Team::from_input(input, workers, seconds)?;
    Ok(team.time_required())
}

#[derive(Debug)]
struct Team {
    requirements: HashMap<char, Vec<char>>,
    available: Vec<char>,
    second: i64,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid line: {}", _0)]
struct InvalidLine(String);

#[derive(Debug, Fail)]
#[fail(display = "circular requirements: {:?}", _0)]
struct Circular(HashMap<char, Vec<char>>);

impl Team {
    fn from_input(input: &str, workers: usize, seconds: i64) -> Result<Team, Error> {
        let mut requirements = HashMap::new();
        let regex = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")?;
        for line in input.lines() {
            let captures = regex.captures(line).ok_or(InvalidLine(line.to_string()))?;
            let parent = captures[1].chars().next().unwrap();
            let child = captures[2].chars().next().unwrap();
            requirements
                .entry(parent)
                .or_insert_with(Vec::new)
                .push(child)
        }
        let mut team = Team::new(requirements, workers, seconds)?;
        while !team.is_done() {
            team.tic();
        }
        Ok(team)
    }

    fn new(
        requirements: HashMap<char, Vec<char>>,
        workers: usize,
        seconds: i64,
    ) -> Result<Team, Circular> {
        let available: Vec<char> = requirements
            .keys()
            .filter(|step| {
                !requirements
                    .values()
                    .any(|children| children.contains(step))
            }).map(|&c| c)
            .collect();
        if available.is_empty() {
            Err(Circular(requirements))
        } else {
            Ok(Team {
                requirements: requirements,
                available: available,
                second: -1,
            })
        }
    }

    fn time_required(&self) -> i64 {
        unimplemented!()
    }

    fn order(&self) -> &str {
        unimplemented!()
    }

    fn is_done(&self) -> bool {
        unimplemented!()
    }

    fn tic(&mut self) {
        unimplemented!()
    }
}

#[test]
fn part_1() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!("CABDFE", correct_order(input).unwrap());
}

#[test]
fn part_2() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!(15, time_required(input, 2, 0).unwrap());
}
