extern crate failure;

use failure::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../input/08.txt");
    println!("{}", sum_of_metadata_entries(input)?);
    Ok(())
}

fn sum_of_metadata_entries(input: &str) -> Result<u64, Error> {
    unimplemented!()
}

#[test]
fn part_1() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(138, sum_of_metadata_entries(input).unwrap());
}
