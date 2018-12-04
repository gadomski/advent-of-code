extern crate chrono;
#[macro_use]
extern crate failure;

use chrono::{DateTime, Utc};
use failure::Error;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", id_times_minute(input)?);
    Ok(())
}

fn id_times_minute(input: &str) -> Result<u64, Error> {
    let events = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, _>>()?;
    unimplemented!()
}

struct Event {
    datetime: DateTime<Utc>,
    r#type: EventType,
}

enum EventType {
    BeginShift { id: u64 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid event: {}", _0)]
struct InvalidEvent(String);

impl FromStr for Event {
    type Err = InvalidEvent;
    fn from_str(s: &str) -> Result<Event, InvalidEvent> {
        unimplemented!()
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
