extern crate pbr;

use pbr::ProgressBar;
use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", winning_score(418, 71339));
    println!("Part 2: {}", winning_score(418, 71339 * 100));
}

fn winning_score(num_players: usize, last_marble: u64) -> u64 {
    let mut circle = VecDeque::with_capacity(last_marble as usize);
    circle.push_front(0);
    let mut marble = 1;
    let mut current_marble_index = 0;
    let mut player = 0;
    let mut scores = vec![0; num_players];
    let mut progress_bar = ProgressBar::new(last_marble);
    loop {
        if marble % 23 == 0 {
            scores[player] += marble;
            let index = (current_marble_index + circle.len() - 7) % circle.len();
            scores[player] += circle.remove(index).unwrap();
            current_marble_index = index % circle.len();
        } else {
            let index = (current_marble_index + 2) % circle.len();
            if index == 0 {
                circle.push_back(marble);
                current_marble_index = circle.len() - 1;
            } else {
                circle.insert(index, marble);
                current_marble_index = index;
            }
        }
        if marble == last_marble {
            break;
        } else {
            marble += 1;
        }
        player = (player + 1) % num_players;
        progress_bar.inc();
    }
    progress_bar.finish();
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
