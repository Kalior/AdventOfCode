use std::collections::HashSet;
use crate::parser::parser;

type Input = Vec<String>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn parse() -> Input {
    parser::parse(6, "", |c| c.to_string())
}

fn solve1(input: Input) -> usize {
    for i in 3..input.len() {
        let potential_marker = &input[i-3..=i];
        if HashSet::<&String>::from_iter(potential_marker).len() == 4 {
            return i + 1;
        }

    }
    0
}

fn solve2(input: Input) -> usize {
    for i in 13..input.len() {
        let potential_marker = &input[i-13..=i];
        if HashSet::<&String>::from_iter(potential_marker).len() == 14 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input(string: &str) -> Input {
        string.chars().map(|c| c.to_string()).collect()
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(to_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(solve1(to_input("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(solve1(to_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(solve1(to_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
        assert_eq!(solve1(input), 1804);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(to_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(solve2(to_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(solve2(to_input("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(solve2(to_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(solve2(to_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
        assert_eq!(solve2(input), 2508);
    }
}
