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

fn requirements(input: &str) -> Result<HashMap<char, Step>, Error> {
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
            .or_insert_with(|| Step::new(step))
            .add_child(child);
    }
    Ok(requirements)
}

#[derive(Debug)]
struct Sleigh {
    steps: String,
    time_required: u64,
}

#[derive(Debug)]
struct Builder {
    base_seconds: u64,
    steps: String,
    requirements: HashMap<char, Step>,
    workers: Vec<Worker>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Step {
    name: char,
    children: Vec<char>,
}

#[derive(Clone, Debug)]
enum Worker {
    Active {
        step: char,
        elapsed: u64,
        time_required: u64,
    },
    Inactive,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid requirement: {}", _0)]
struct InvalidRequirement(String);

impl Sleigh {
    fn build(input: &str, workers: usize, base_seconds: u64) -> Result<Sleigh, Error> {
        let mut builder = Builder {
            base_seconds: base_seconds,
            steps: String::new(),
            requirements: requirements(input)?,
            workers: vec![Worker::new(); workers],
        };
        let mut second = 0;
        builder.tic();
        while !builder.is_done() {
            builder.tic();
            second += 1;
            println!("{}: {:?}", second, builder);
        }
        Ok(Sleigh {
            steps: builder.steps,
            time_required: second,
        })
    }
}

impl Builder {
    fn is_done(&self) -> bool {
        unimplemented!()
    }

    fn tic(&mut self) {
        for worker in self.workers.iter_mut().filter(|worker| !worker.is_active()) {}
    }
}

impl Worker {
    fn new() -> Worker {
        Worker::Inactive
    }

    fn is_active(&self) -> bool {
        match *self {
            Worker::Inactive => false,
            Worker::Active { .. } => true,
        }
    }
}

impl Step {
    fn new(name: char) -> Step {
        Step {
            name: name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: char) {
        self.children.push(child);
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
