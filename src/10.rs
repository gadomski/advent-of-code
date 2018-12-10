extern crate failure;
extern crate regex;
#[macro_use]
extern crate text_io;

use failure::Error;
use regex::Regex;
use std::env;
use std::i64;
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    let argument = env::args().nth(1).expect("Must provide one argument");
    let input = match argument.as_str() {
        "example" => example_input(),
        "part-1" | "part-2" => include_str!("../input/10.txt"),
        _ => panic!("invalid argument: {}", argument),
    };
    let mut points = input
        .lines()
        .map(|line| {
            line.parse::<Point>()
                .expect(&format!("invalid line: {}", line))
        })
        .collect::<Vec<Point>>();
    let mut second = 0;
    loop {
        println!("After {} seconds:", second);
        let (minx, miny, maxx, maxy) = minmax(&points);
        let keep_going = if maxy - miny > 100 {
            println!("Too many rows, skipping print and auto-continuing.");
            true
        } else {
            for y in miny..=maxy {
                for x in minx..=maxx {
                    if points
                        .iter()
                        .find(|point| point.x == x && point.y == y)
                        .is_some()
                    {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            print!("\nContinue? (y/n): ");
            io::stdout().flush().unwrap();
            let answer: char = read!();
            answer == 'y'
        };
        if keep_going {
            for point in &mut points {
                point.tic();
            }
            second += 1;
            println!("");
        } else {
            break;
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Point {
    fn tic(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point, Error> {
        let regex = Regex::new(
            r"^position=<\s*([\-\d]+),\s*([\-\d]+)> velocity=<\s*([\-\d]+),\s*([\-\d]+)>$",
        )?;
        let captures = regex.captures(s).unwrap();
        Ok(Point {
            x: captures[1].parse()?,
            y: captures[2].parse()?,
            vx: captures[3].parse()?,
            vy: captures[4].parse()?,
        })
    }
}

fn minmax(points: &[Point]) -> (i64, i64, i64, i64) {
    let mut minx = i64::MAX;
    let mut miny = i64::MAX;
    let mut maxx = i64::MIN;
    let mut maxy = i64::MIN;
    for point in points {
        if point.x < minx {
            minx = point.x;
        }
        if point.x > maxx {
            maxx = point.x;
        }
        if point.y < miny {
            miny = point.y;
        }
        if point.y > maxy {
            maxy = point.y;
        }
    }
    (minx, miny, maxx, maxy)
}

fn example_input() -> &'static str {
    "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
}
