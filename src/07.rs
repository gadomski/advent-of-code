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

fn time_required(input: &str, workers: usize, base_seconds: i64) -> Result<i64, Error> {
    let sleigh = Sleigh::build(input, workers, base_seconds)?;
    Ok(sleigh.time_required)
}

#[derive(Debug)]
struct Sleigh {
    steps: String,
    time_required: i64,
}

#[derive(Debug)]
struct Builder {
    workers: Vec<Worker>,
}

#[derive(Clone, Debug)]
enum Worker {
    Active {
        step: char,
        elapsed: i64,
        time_required: i64,
    },
    Inactive,
}

impl Sleigh {
    fn build(input: &str, workers: usize, base_seconds: i64) -> Result<Sleigh, Error> {
        let mut builder = Builder {
            workers: vec![Worker::new(); workers],
        };
        while !builder.is_done() {
            builder.tic();
        }
        Ok(builder.into())
    }
}

impl Builder {
    fn is_done(&self) -> bool {
        unimplemented!()
    }

    fn tic(&self) {
        unimplemented!()
    }
}

impl From<Builder> for Sleigh {
    fn from(builder: Builder) -> Sleigh {
        unimplemented!()
    }
}

impl Worker {
    fn new() -> Worker {
        Worker::Inactive
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
