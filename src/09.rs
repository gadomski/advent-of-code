use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", winning_score(418, 71339));
    println!("Part 2: {}", winning_score(418, 71339 * 100));
}

fn winning_score(num_players: usize, last_marble: u64) -> u64 {
    let mut circle = VecDeque::with_capacity(last_marble as usize);
    circle.push_front(0);
    let mut player = 0;
    let mut scores = vec![0; num_players];
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            scores[player] += marble;
            for _ in 0..7 {
                let n = circle.pop_back().unwrap();
                circle.push_front(n);
            }
            scores[player] += circle.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let n = circle.pop_front().unwrap();
                circle.push_back(n);
            }
            circle.push_front(marble);
        }
        player = (player + 1) % num_players;
    }
    scores.into_iter().max().unwrap_or(0)
}

#[test]
fn part_1() {
    assert_eq!(32, winning_score(9, 25));
    assert_eq!(8317, winning_score(10, 1618));
    assert_eq!(146373, winning_score(13, 7999));
    assert_eq!(2764, winning_score(17, 1104));
    assert_eq!(54718, winning_score(21, 6111));
    assert_eq!(37305, winning_score(30, 5807));
}
