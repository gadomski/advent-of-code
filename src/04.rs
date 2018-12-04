extern crate chrono;
#[macro_use]
extern crate failure;
extern crate regex;

use chrono::{DateTime, TimeZone, Timelike, Utc};
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", strategy_1(input)?);
    println!("Part 2: {}", strategy_2(input)?);
    Ok(())
}

fn strategy_1(input: &str) -> Result<u64, Error> {
    let state_machine = StateMachine::from_input(input)?;
    let id = state_machine.sleepiest_guard().ok_or(NoSleeps)?;
    let minute = state_machine.sleepiest_minute(id).ok_or(NoSleeps)?;
    Ok(id * u64::from(minute))
}

fn strategy_2(input: &str) -> Result<u64, Error> {
    let state_machine = StateMachine::from_input(input)?;
    state_machine.strategy_2()
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
    state: State,
    sleeps: HashMap<u64, HashMap<u32, u64>>,
}

#[derive(Debug)]
enum State {
    Awake,
    Asleep(DateTime<Utc>),
}

impl StateMachine {
    fn from_input(input: &str) -> Result<StateMachine, Error> {
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
        Ok(state_machine)
    }

    fn new(event: Event) -> Result<StateMachine, StateError> {
        match event.kind {
            EventKind::BeginShift(id) => Ok(StateMachine {
                guard_id: id,
                state: State::Awake,
                sleeps: HashMap::new(),
            }),
            _ => Err(StateError::Initialize(event)),
        }
    }

    fn handle(&mut self, event: Event) -> Result<(), StateError> {
        match event.kind {
            EventKind::BeginShift(id) => match self.state {
                State::Awake => self.guard_id = id,
                State::Asleep(_) => {
                    return Err(StateError::EndShiftWhileAsleep(self.guard_id, event))
                }
            },
            EventKind::FallAsleep => match self.state {
                State::Awake => self.state = State::Asleep(event.datetime),
                State::Asleep(_) => return Err(StateError::DoubleSleep(event)),
            },
            EventKind::WakeUp => match self.state {
                State::Awake => return Err(StateError::DoubleAwake(event)),
                State::Asleep(start) => {
                    if (event.datetime - start).num_minutes() >= 60 {
                        return Err(StateError::SleepTooLong(start, event.datetime));
                    }
                    let sleeps = self
                        .sleeps
                        .entry(self.guard_id)
                        .or_insert_with(HashMap::new);
                    for minute in start.minute()..event.datetime.minute() {
                        let entry = sleeps.entry(minute).or_insert(0);
                        *entry += 1;
                    }
                    self.state = State::Awake;
                }
            },
        }
        Ok(())
    }

    fn sleepiest_guard(&self) -> Option<u64> {
        self.sleeps
            .iter()
            .max_by_key(|(_, sleeps)| sleeps.values().sum::<u64>())
            .map(|(&k, _)| k)
    }

    fn sleepiest_minute(&self, guard_id: u64) -> Option<u32> {
        self.sleeps
            .get(&guard_id)
            .and_then(|ref minutes| minutes.iter().max_by_key(|(_, &n)| n).map(|(&k, _)| k))
    }

    fn strategy_2(&self) -> Result<u64, Error> {
        let guard_id = self
            .sleeps
            .iter()
            .max_by_key(|(_, v)| v.values().max())
            .map(|(k, _)| k)
            .ok_or(NoSleeps)?;
        let minute = self
            .sleeps
            .get(guard_id)
            .ok_or(NoSleeps)?
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(&k, _)| u64::from(k))
            .ok_or(NoSleeps)?;
        Ok(minute * guard_id)
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
#[fail(display = "no sleeps provided in input")]
struct NoSleeps;

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

    #[fail(display = "too long asleep: {} to {}", _0, _1)]
    SleepTooLong(DateTime<Utc>, DateTime<Utc>),
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
    assert_eq!(240, strategy_1(input).unwrap());
}

#[test]
fn part_2() {
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
    assert_eq!(4455, strategy_2(input).unwrap());
}
