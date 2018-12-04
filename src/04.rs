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
        .collect::<Result<Vec<Event>, _>>()?;
    events.sort();
    unimplemented!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    datetime: DateTime<Utc>,
    r#type: EventType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    BeginShift { id: u64 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid event: {}", _0)]
struct InvalidEvent(String);

#[derive(Debug, Fail)]
#[fail(display = "invalid event type: {}", _0)]
struct InvalidEventType(String);

impl FromStr for Event {
    type Err = Error;
    fn from_str(s: &str) -> Result<Event, Error> {
        let regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)$")?;
        let captures = regex.captures(s).ok_or(InvalidEvent(s.to_string()))?;
        Ok(Event {
            datetime: Utc.datetime_from_str(&captures[1], "%Y-%m-%d %H:%M")?,
            r#type: captures[2].parse()?,
        })
    }
}

impl FromStr for EventType {
    type Err = Error;
    fn from_str(s: &str) -> Result<EventType, Error> {
        if s == "falls asleep" {
            Ok(EventType::FallAsleep)
        } else if s == "wakes up" {
            Ok(EventType::WakeUp)
        } else {
            let regex = Regex::new(r"^Guard #(\d+) begins shift$")?;
            let captures = regex.captures(s).ok_or(InvalidEventType(s.to_string()))?;
            Ok(EventType::BeginShift {
                id: captures[1].parse()?,
            })
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
