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

fn requirements(input: &str) -> Result<HashMap<u8, Vec<u8>>, Error> {
    let mut requirements = HashMap::new();
    let regex = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")?;
    for line in input.lines() {
        let captures = regex
            .captures(line)
            .ok_or(InvalidRequirement(line.to_string()))?;
        let step = captures[1].bytes().next().unwrap();
        let child = captures[2].bytes().next().unwrap();
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

#[derive(Clone, Debug)]
struct Worker {
    step: u8,
    children: Vec<u8>,
    elapsed: u64,
    time_required: u64,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid requirement: {}", _0)]
struct InvalidRequirement(String);

#[derive(Debug, Fail)]
#[fail(display = "the worker overworked: {:?}", _0)]
struct Overwork(Worker);

impl Sleigh {
    fn build(input: &str, workers: usize, base_seconds: u64) -> Result<Sleigh, Error> {
        let steps = String::new();
        let requirements = requirements(input)?;
        let second = 0;
        let mut workers: Vec<Option<Worker>> = vec![None; workers];
        let mut available: Vec<_> = requirements
            .iter()
            .filter_map(|(step, children)| {
                if requirements.values().all(|v| !v.contains(step)) {
                    Some((*step, children.clone()))
                } else {
                    None
                }
            }).collect();
        available.sort();
        loop {
            for worker in workers.iter_mut().filter(|w| w.is_none()) {
                if !available.is_empty() {
                    let (step, children) = available.remove(0);
                    *worker = Some(Worker {
                        step: step,
                        children: children,
                        elapsed: 0,
                        time_required: base_seconds + u64::from(step) + 1 - u64::from(b'A'),
                    });
                }
            }
            for worker in workers.iter_mut().filter_map(|w| w.as_mut()) {
                worker.tic()?;
            }
        }
        Ok(Sleigh {
            steps: steps,
            time_required: second,
        })
    }
}

impl Worker {
    fn tic(&mut self) -> Result<(), Overwork> {
        self.elapsed += 1;
        if self.elapsed <= self.time_required {
            Ok(())
        } else {
            Err(Overwork(self.clone()))
        }
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
