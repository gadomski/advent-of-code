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
    let sleigh = Sleigh::build(input, 1, 0)?;
    Ok(sleigh.steps)
}

fn time_required(input: &str, workers: usize, base_seconds: u64) -> Result<u64, Error> {
    let sleigh = Sleigh::build(input, workers, base_seconds)?;
    Ok(sleigh.time_required)
}

fn requirements(input: &str) -> Result<HashMap<char, Vec<char>>, Error> {
    let mut requirements = HashMap::new();
    let regex = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")?;
    for line in input.lines() {
        let captures = regex
            .captures(line)
            .ok_or(InvalidRequirement(line.to_string()))?;
        let step = captures[1].chars().next().unwrap();
        let child = captures[2].chars().next().unwrap();
        requirements
            .entry(step)
            .or_insert_with(Vec::new)
            .push(child);
    }
    Ok(requirements)
}

#[derive(Debug)]
struct Sleigh {
    steps: String,
    time_required: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Step {
    name: char,
    children: Vec<char>,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid requirement: {}", _0)]
struct InvalidRequirement(String);

impl Sleigh {
    fn build(input: &str, workers: usize, base_seconds: u64) -> Result<Sleigh, Error> {
        let steps = String::new();
        let requirements = requirements(input)?;
        let second = 0;
        Ok(Sleigh {
            steps: steps,
            time_required: second,
        })
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
