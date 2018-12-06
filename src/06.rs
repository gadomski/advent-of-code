extern crate failure;

use failure::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/06.txt");
    println!("Part 1: {}", size_of_largest_area(input)?);
    Ok(())
}

fn size_of_largest_area(input: &str) -> Result<u64, Error> {
    unimplemented!()
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
