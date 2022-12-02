use crate::parser::parser;

#[derive(Copy, Clone)]
enum Hands {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone)]
enum Results {
    Win,
    Draw,
    Lose,
}

fn parse_hand(val: &str) -> Hands {
    match val {
        "A" => Hands::Rock,
        "X" => Hands::Rock,
        "B" => Hands::Paper,
        "Y" => Hands::Paper,
        "C" => Hands::Scissor,
        "Z" => Hands::Scissor,
        _ => {
            panic!()
        }
    }
}

fn hand_to_score(hand: Hands) -> i32 {
    match hand {
        Hands::Rock => 1,
        Hands::Paper => 2,
        Hands::Scissor => 3
    }
}


fn parse_result(val: &str) -> Results {
    match val {
        "X" => Results::Lose,
        "Y" => Results::Draw,
        "Z" => Results::Win,
        _ => {
            panic!()
        }
    }
}

fn result_to_score(res: Results) -> i32 {
    match res {
        Results::Lose => 0,
        Results::Draw => 3,
        Results::Win => 6,
    }
}


pub fn solve() {
    let input = parse();
    solve1(input);
    let input_2 = parse_strategy();
    solve2(input_2);
}

fn parse() -> Vec<(Hands, Hands)> {
    parser::parse(2, "\n", parse_to_hands)
}

fn parse_to_hands(line: &str) -> (Hands, Hands) {
    let vs: Vec<&str> = line.split(" ").collect();

    (parse_hand(vs[0]), parse_hand(vs[1]))
}

fn parse_line(line: &str) -> (Hands, Results) {
    let vs: Vec<&str> = line.split(" ").collect();

    let hand = parse_hand(vs[0]);
    let result = parse_result(vs[1]);
    (hand, result)
}

fn parse_strategy() -> Vec<(Hands, Results)> {
    parser::parse(2, "\n", parse_line)
}


fn score_result(they_play: Hands, i_play: Hands) -> i32 {
    let result = match they_play {
        Hands::Rock => match i_play {
            Hands::Rock => Results::Draw,
            Hands::Paper => Results::Win,
            Hands::Scissor => Results::Lose,
        },
        Hands::Paper => match i_play {
            Hands::Rock => Results::Lose,
            Hands::Paper => Results::Draw,
            Hands::Scissor => Results::Win,
        },
        Hands::Scissor => match i_play {
            Hands::Rock => Results::Win,
            Hands::Paper => Results::Lose,
            Hands::Scissor => Results::Draw,
        },
    };


    result_to_score(result) + hand_to_score(i_play)
}

fn use_strategy(result_pair: (Hands, Results)) -> i32 {
    let (they_play, my_result) = result_pair;

    let i_play = match they_play {
        Hands::Rock => match my_result {
            Results::Win => Hands::Paper,
            Results::Draw => Hands::Rock,
            Results::Lose => Hands::Scissor,
        },
        Hands::Paper => match my_result {
            Results::Win => Hands::Scissor,
            Results::Draw => Hands::Paper,
            Results::Lose => Hands::Rock,
        },
        Hands::Scissor => match my_result {
            Results::Win => Hands::Rock,
            Results::Draw => Hands::Scissor,
            Results::Lose => Hands::Paper,
        },
    };


    hand_to_score(i_play) + result_to_score(my_result)
}

fn solve1(input: Vec<(Hands, Hands)>) -> i32 {
    input.into_iter().map(|hand_pair| score_result(hand_pair.0, hand_pair.1)).sum()
}

fn solve2(input: Vec<(Hands, Results)>) -> i32 {
    input.into_iter().map(use_strategy).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 17189);
    }

    #[test]
    fn part_two_test() {
        let input = parse_strategy();
        assert_eq!(solve2(input), 13490);
    }
}
