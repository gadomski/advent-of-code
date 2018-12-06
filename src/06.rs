#[macro_use]
extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::i64;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/06.txt");
    println!("Part 1: {}", size_of_largest_area(input)?);
    Ok(())
}

fn size_of_largest_area(input: &str) -> Result<usize, Error> {
    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, Error>>()?;
    let min = Point {
        x: coordinates
            .iter()
            .map(|point| point.x)
            .min()
            .ok_or(NoPoints)?,
        y: coordinates
            .iter()
            .map(|point| point.y)
            .min()
            .ok_or(NoPoints)?,
    };
    let max = Point {
        x: coordinates
            .iter()
            .map(|point| point.x)
            .max()
            .ok_or(NoPoints)?,
        y: coordinates
            .iter()
            .map(|point| point.y)
            .max()
            .ok_or(NoPoints)?,
    };
    let mut map = HashMap::new();
    let mut candidate_coordinates: HashSet<Point> =
        HashSet::from_iter(coordinates.iter().map(|&p| p));
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let point = Point { x: x, y: y };
            let closest_coordinate = closest_coordinate(&coordinates, point);
            if let Some(closest_coordinate) = closest_coordinate {
                if x == min.x || x == max.x || y == min.y || y == max.y {
                    candidate_coordinates.remove(&closest_coordinate);
                }
            }
            map.insert(point, closest_coordinate);
        }
    }
    candidate_coordinates
        .into_iter()
        .map(|coordinate| {
            map.values()
                .filter(|closest_coordinate| {
                    closest_coordinate
                        .map(|closest_coordinate| closest_coordinate == coordinate)
                        .unwrap_or(false)
                }).count()
        }).max()
        .ok_or(NoFiniteAreas.into())
}

fn closest_coordinate(coordinates: &[Point], point: Point) -> Option<Point> {
    let mut min_distance = i64::MAX;
    let mut closest_coordinate = None;
    for coordinate in coordinates {
        let distance = coordinate.distance_to(point);
        if distance == min_distance {
            closest_coordinate = None;
        } else if distance < min_distance {
            min_distance = distance;
            closest_coordinate = Some(*coordinate);
        }
    }
    closest_coordinate
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Fail)]
#[fail(display = "invalid point format: {}", _0)]
struct InvalidPointFormat(String);

#[derive(Debug, Fail)]
#[fail(display = "no points provided in input")]
struct NoPoints;

#[derive(Debug, Fail)]
#[fail(display = "no finite areas found")]
struct NoFiniteAreas;

impl Point {
    fn distance_to(&self, point: Point) -> i64 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Point, Error> {
        let regex = Regex::new(r"^(\d+), (\d+)$")?;
        let captures = regex.captures(s).ok_or(InvalidPointFormat(s.to_string()))?;
        Ok(Point {
            x: captures[1].parse()?,
            y: captures[2].parse()?,
        })
    }
}

#[test]
fn part_1() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(17, size_of_largest_area(input).unwrap());
}
