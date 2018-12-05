extern crate failure;

use failure::Error;
use std::iter::FromIterator;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/05.txt").trim();
    println!("Part 1: {}", remaining_units_after_reaction(input));
    println!("Part 2: {}", remaining_units_after_best_reaction(input));
    Ok(())
}

fn remaining_units_after_reaction(input: &str) -> usize {
    let polymer = react_all(input);
    polymer.len()
}

fn remaining_units_after_best_reaction(input: &str) -> usize {
    let mut polymers = Vec::new();
    for to_remove in (b'a'..=b'z').map(char::from) {
        let input = String::from_iter(
            input
                .chars()
                .filter(|&c| c.to_lowercase().next().unwrap() != to_remove),
        );
        polymers.push(react_all(&input));
    }
    polymers
        .into_iter()
        .map(|polymer| polymer.len())
        .min()
        .unwrap()
}

fn react_all(input: &str) -> String {
    let mut units: Vec<char> = input.chars().collect();
    while react_one(&mut units) {}
    String::from_iter(units)
}

fn react_one(units: &mut Vec<char>) -> bool {
    if units.is_empty() {
        return false;
    }
    for i in 0..(units.len() - 1) {
        let a = units[i];
        let b = units[i + 1];
        if a != b && a.to_lowercase().next() == b.to_lowercase().next() {
            units.remove(i);
            units.remove(i);
            return true;
        }
    }
    return false;
}

#[test]
fn part_1() {
    assert_eq!(10, remaining_units_after_reaction("dabAcCaCBAcCcaDA"));
}

#[test]
fn part_2() {
    assert_eq!(4, remaining_units_after_best_reaction("dabAcCaCBAcCcaDA"));
}
