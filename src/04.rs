extern crate chrono;
#[macro_use]
extern crate failure;
extern crate regex;

use chrono::{DateTime, TimeZone, Utc};
use failure::Error;
use regex::Regex;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", id_times_minute(input)?);
    Ok(())
}

fn id_times_minute(input: &str) -> Result<u64, Error> {
    let mut events = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, Error>>()?;
    if events.is_empty() {
        return Err(NoEvents.into());
    }
    events.sort();
    let mut state_machine = StateMachine::new(events.remove(0))?;
    for event in events {
        state_machine.handle(event)?;
    }
    unimplemented!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    datetime: DateTime<Utc>,
    kind: EventKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    BeginShift(u64),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct StateMachine {
    guard_id: u64,
    awake: bool,
}

impl StateMachine {
    fn new(event: Event) -> Result<StateMachine, StateError> {
        match event.kind {
            EventKind::BeginShift(id) => Ok(StateMachine {
                guard_id: id,
                awake: true,
            }),
            _ => Err(StateError::Initialize(event)),
        }
    }

    fn handle(&mut self, event: Event) -> Result<(), StateError> {
        match event.kind {
            EventKind::BeginShift(id) => {
                if self.awake {
                    self.guard_id = id;
                } else {
                    return Err(StateError::EndShiftWhileAsleep(self.guard_id, event));
                }
            }
            EventKind::FallAsleep => {
                if self.awake {
                    self.awake = false;
                } else {
                    return Err(StateError::DoubleSleep(event));
                }
            }
            EventKind::WakeUp => {
                if self.awake {
                    return Err(StateError::DoubleAwake(event));
                } else {
                    self.awake = true;
                    // TODO track sleeps
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Fail)]
#[fail(display = "invalid event format: {}", _0)]
struct ParseEvent(String);

#[derive(Debug, Fail)]
#[fail(display = "invalid event kind format: {}", _0)]
struct ParseEventKind(String);

#[derive(Debug, Fail)]
#[fail(display = "no events provided in input")]
struct NoEvents;

#[derive(Debug, Fail)]
enum StateError {
    #[fail(
        display = "must initialize a state with a `BeginShift`, not: {:?}",
        _0
    )]
    Initialize(Event),

    #[fail(
        display = "shift ended for guard {} while still asleep: {:?}",
        _0,
        _1
    )]
    EndShiftWhileAsleep(u64, Event),

    #[fail(display = "can't fall asleep when already asleep: {:?}", _0)]
    DoubleSleep(Event),

    #[fail(display = "can't wake up when already awake: {:?}", _0)]
    DoubleAwake(Event),
}

impl FromStr for Event {
    type Err = Error;
    fn from_str(s: &str) -> Result<Event, Error> {
        let regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)$")?;
        let captures = regex.captures(s).ok_or(ParseEvent(s.to_string()))?;
        Ok(Event {
            datetime: Utc.datetime_from_str(&captures[1], "%Y-%m-%d %H:%M")?,
            kind: captures[2].parse()?,
        })
    }
}

impl FromStr for EventKind {
    type Err = Error;
    fn from_str(s: &str) -> Result<EventKind, Error> {
        if s == "falls asleep" {
            Ok(EventKind::FallAsleep)
        } else if s == "wakes up" {
            Ok(EventKind::WakeUp)
        } else {
            let regex = Regex::new(r"^Guard #(\d+) begins shift$")?;
            let captures = regex.captures(s).ok_or(ParseEventKind(s.to_string()))?;
            Ok(EventKind::BeginShift(captures[1].parse()?))
        }
    }
}

#[test]
fn part_1() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
    assert_eq!(240, id_times_minute(input).unwrap());
}
