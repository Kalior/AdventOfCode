use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve() {
    let num_players = 416;
    let last_marble_score = 71617;

    let answer1 = solve1(num_players, last_marble_score);
    println!("{:?}", answer1);
    let answer2 = solve1(num_players, last_marble_score * 100);
    println!("{:?}", answer2);
}

fn solve1(num_players: i32, last_marble_score: i64) -> i64 {
    let mut scores = HashMap::new();
    for i in 0..num_players {
        scores.insert(i, 0 as i64);
    }
    let mut marbles = VecDeque::new();

    marbles.push_back(0);

    let mut current_marble_value = 1 as i64;
    let mut current_player = 0;

    while current_marble_value <= last_marble_score {
        if current_marble_value % 23 == 0 {
            let counter = scores.entry(current_player).or_insert(0);

            *counter += current_marble_value;

            for _ in 1..8 {
                let marble = marbles.pop_back().unwrap();
                marbles.push_front(marble);
            }

            let removed_marble_value = marbles.pop_back().unwrap();

            *counter += removed_marble_value;
        } else {
            let marble = marbles.pop_front().unwrap();
            marbles.push_back(marble);

            marbles.push_front(current_marble_value);
        }
        current_marble_value += 1;
        current_player = (current_player + 1) % num_players;

        let marble = marbles.pop_front().unwrap();
        marbles.push_back(marble);
    }

    *scores.values().max().unwrap_or(&0)
}
