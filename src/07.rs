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

fn time_required(input: &str, workers: usize, seconds: u64) -> Result<u64, Error> {
    let team = Team::from_input(input, workers, seconds)?;
    Ok(team.time_required())
}

#[derive(Debug)]
struct Team {}

#[derive(Debug, Fail)]
#[fail(display = "invalid line: {}", _0)]
struct InvalidLine(String);

#[derive(Debug, Fail)]
#[fail(display = "circular requirements: {:?}", _0)]
struct Circular(HashMap<String, Vec<String>>);

impl Team {
    fn from_input(input: &str, workers: usize, seconds: u64) -> Result<Team, Error> {
        let mut requirements = HashMap::new();
        let regex = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")?;
        for line in input.lines() {
            let captures = regex.captures(line).ok_or(InvalidLine(line.to_string()))?;
            let parent = &captures[1];
            let child = &captures[2];
            requirements
                .entry(parent.to_string())
                .or_insert_with(Vec::new)
                .push(child.to_string())
        }
        let mut available: Vec<String> = requirements
            .keys()
            .filter(|step| {
                !requirements
                    .values()
                    .any(|children| children.contains(step))
            }).map(|s| s.clone())
            .collect();
        if available.is_empty() {
            return Err(Circular(requirements.clone()).into());
        }
        let mut done = String::new();
        while !available.is_empty() {
            available.sort();
            let step = available.remove(0);
            done.push_str(&step);
            if let Some(children) = requirements.remove(&step) {
                for child in children {
                    if !requirements
                        .values()
                        .any(|children| children.contains(&child))
                    {
                        available.push(child);
                    }
                }
            }
        }
        Ok(Team {})
    }

    fn time_required(&self) -> u64 {
        unimplemented!()
    }

    fn order(&self) -> &str {
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
