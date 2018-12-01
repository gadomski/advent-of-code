use std::collections::HashSet;

fn main() {
    let input: Vec<i64> = include_str!("../input/01.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut frequency = 0;
    for delta in &input {
        frequency += delta;
    }
    println!("Part 1: {}", frequency);

    let mut frequency = 0;
    let mut seen = HashSet::new();
    seen.insert(frequency);
    'outer: loop {
        for delta in &input {
            frequency += delta;
            if !seen.insert(frequency) {
                break 'outer;
            }
        }
    }
    println!("Part 2: {}", frequency);
}
