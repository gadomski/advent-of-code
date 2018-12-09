#[macro_use]
extern crate failure;

use std::str::FromStr;

fn main() -> Result<(), failure::Error> {
    let input = include_str!("../input/08.txt");
    println!("{}", sum_of_metadata(input)?);
    Ok(())
}

fn sum_of_metadata(input: &str) -> Result<u64, failure::Error> {
    let tree: Node = input.parse()?;
    Ok(tree.sum_of_metadata())
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u64>,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "invalid header: {:?}", _0)]
    InvalidHeader(Vec<u64>),

    #[fail(display = "metadata mismatch, expected {} metadata, got: {:?}", _0, _1)]
    MetadataMismatch(u64, Vec<u64>),
}

impl Node {
    fn new(integers: &mut Vec<u64>) -> Result<Node, Error> {
        if integers.len() < 2 {
            return Err(Error::InvalidHeader(integers.clone()));
        }
        let num_children = integers.remove(0);
        let num_metadata = integers.remove(0);
        let mut children = Vec::new();
        for _ in 0..num_children {
            children.push(Node::new(integers)?);
        }
        if num_metadata > integers.len() as u64 {
            return Err(Error::MetadataMismatch(num_metadata, integers.clone()));
        }
        let mut metadata = Vec::new();
        for _ in 0..num_metadata {
            metadata.push(integers.remove(0));
        }
        Ok(Node {
            children: children,
            metadata: metadata,
        })
    }

    fn sum_of_metadata(&self) -> u64 {
        self.children
            .iter()
            .map(|child| child.sum_of_metadata())
            .sum::<u64>()
            + self.metadata.iter().sum::<u64>()
    }
}

impl FromStr for Node {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Node, failure::Error> {
        let mut integers = s
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        Node::new(&mut integers).map_err(failure::Error::from)
    }
}

#[test]
fn part_1() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(138, sum_of_metadata(input).unwrap());
}
