use crate::parser::parser;
use std::collections::HashSet;

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

fn index_of_first_window_with_all_unique_chars(input: Input, window_size: usize) -> usize {
    input
        .as_slice()
        .windows(window_size)
        .enumerate()
        .filter(|(_, window)| HashSet::<&String>::from_iter(*window).len() == window_size)
        .map(|(i, _)| i + window_size)
        .next()
        .unwrap_or_default()
}

fn solve1(input: Input) -> usize {
    index_of_first_window_with_all_unique_chars(input, 4)
}

fn solve2(input: Input) -> usize {
    index_of_first_window_with_all_unique_chars(input, 14)
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
