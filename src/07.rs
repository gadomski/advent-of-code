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
    seconds: i64,
    workers: Vec<Option<Worker>>,
    done: String,
}

#[derive(Clone, Debug)]
struct Worker {
    step: char,
    elapsed: i64,
    required: i64,
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
                seconds: seconds,
                second: -1,
                workers: vec![None; workers],
                done: String::new(),
            })
        }
    }

    fn time_required(&self) -> i64 {
        unimplemented!()
    }

    fn order(&self) -> &str {
        self.done.as_str()
    }

    fn is_done(&self) -> bool {
        // FIXME this isn't true with the finite-time model
        self.available.is_empty()
    }

    fn tic(&mut self) {
        self.available.sort();
        for worker in self.workers.iter_mut().filter(|worker| worker.is_none()) {
            if !self.available.is_empty() {
                *worker = Some(Worker::new(self.available.remove(0), self.seconds));
            }
        }
        for worker in self.workers.iter_mut().filter_map(|worker| worker.as_mut()) {
            worker.tic();
        }
        self.second += 1;
        println!("{:?}", self);
    }
}

impl Worker {
    fn new(step: char, seconds: i64) -> Worker {
        Worker {
            step: step,
            elapsed: 0,
            required: seconds + step as i64 - i64::from(b'A') + 1,
        }
    }

    fn tic(&mut self) {
        self.elapsed += 1;
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
