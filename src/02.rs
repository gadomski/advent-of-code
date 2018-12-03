const EXAMPLE: &'static str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

fn main() {}

fn checksum(s: &str) -> u64 {
    unimplemented!()
}

#[test]
fn example() {
    assert_eq!(12, checksum(EXAMPLE));
}
