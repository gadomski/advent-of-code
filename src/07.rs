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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Step {
    name: char,
    children: Vec<char>,
}

#[derive(Clone, Debug)]
enum Worker {
    Active {
        step: Step,
        elapsed: u64,
        time_required: u64,
    },
    Inactive,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid requirement: {}", _0)]
struct InvalidRequirement(String);

#[derive(Debug, Fail)]
#[fail(display = "worker overworked: {:?}", _0)]
struct Overwork(Worker);

impl Sleigh {
    fn build(input: &str, workers: usize, base_seconds: u64) -> Result<Sleigh, Error> {
        let steps = String::new();
        let requirements = requirements(input)?;
        let mut workers = vec![Worker::inactive(); workers];
        let second = 0;
        let mut available: Vec<&Step> = requirements
            .keys()
            .filter_map(|&name| {
                if requirements.values().all(|step| !step.has_child(name)) {
                    Some(&requirements[&name])
                } else {
                    None
                }
            }).collect();
        available.sort();
        for worker in &mut workers {
            if !available.is_empty() {
                *worker = Worker::active(available.remove(0).clone(), base_seconds);
            }
        }
        while workers.iter().any(|worker| worker.is_active()) {
            for worker in &mut workers {
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
    fn inactive() -> Worker {
        Worker::Inactive
    }

    fn is_active(&self) -> bool {
        match *self {
            Worker::Inactive => false,
            Worker::Active { .. } => true,
        }
    }

    fn active(step: Step, base_seconds: u64) -> Worker {
        Worker::Active {
            elapsed: 0,
            time_required: base_seconds + step.name as u64 - u64::from(b'A') + 1,
            step: step,
        }
    }

    fn tic(&mut self) -> Result<(), Overwork> {
        match *self {
            Worker::Active {
                ref mut elapsed, ..
            } => {
                *elapsed += 1;
            }
            Worker::Inactive => {}
        }
        match *self {
            Worker::Active {
                elapsed,
                time_required,
                ..
            } => if elapsed > time_required {
                Err(Overwork(self.clone()))
            } else {
                Ok(())
            },
            Worker::Inactive => return Ok(()),
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

    fn has_child(&self, child: char) -> bool {
        self.children.iter().any(|&c| c == child)
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
