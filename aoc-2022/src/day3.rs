use std::collections::{HashMap, HashSet};
use std::iter::zip;
use crate::parser::parser;


pub fn solve() {
    let input = parse();
    solve1(input.to_vec());
    solve2(input.to_vec());
}


fn solve1(input: Vec<(Vec<char>, Vec<char>)>) -> usize {
    let item_types = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let char_val: HashMap<char, usize> = HashMap::from_iter(zip(item_types.chars(), (1..1 + item_types.len()).into_iter()));

    let mut sum = 0;
    for (first_half, second_half) in input {
        let first_set: HashSet<char> = HashSet::from_iter(first_half.into_iter());
        let second_set: HashSet<char> = HashSet::from_iter(second_half.into_iter());

        let mut intersection = first_set.intersection(&second_set);

        let shared_item = intersection.next().expect("No intersection");
        let item_value = char_val.get(shared_item).expect("Char not in map");
        sum += item_value;

        if intersection.count() != 0 {
            panic!("More than one item in intersection.");
        }
    }

    sum
}

fn solve2(input: Vec<(Vec<char>, Vec<char>)>) -> usize {
    let item_types = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let char_val: HashMap<char, usize> = HashMap::from_iter(zip(item_types.chars(), (1..1 + item_types.len()).into_iter()));

    let mut sum = 0;
    for i in (0..input.len()).step_by(3) {
        let shared_item = get_shared_item(&input, item_types, &i);
        let item_value = char_val.get(&shared_item).expect("Char not in map");
        sum += item_value;
    }

    sum
}

fn get_shared_item(input: &Vec<(Vec<char>, Vec<char>)>, item_types: &str, i: &usize) -> char {
    let mut set: HashSet<char> = HashSet::from_iter(item_types.chars());
    for j in 0..3 {
        let mut new_set = HashSet::new();
        for v in &input[i + j].0 {
            new_set.insert(*v);
        }
        for v in &input[i + j].1 {
            new_set.insert(*v);
        }
        set = set.intersection(&new_set).map(|c| *c).collect();
    }

    let shared_item = set.iter().next().expect("No intersection");

    if set.len() != 1 {
        panic!("More than one item in intersection.");
    }

    *shared_item
}

fn split_line(line: &str) -> (Vec<char>, Vec<char>) {
    let half_length = (line.len() / 2) as usize;
    (line[..half_length].chars().collect(), line[half_length..].chars().collect())
}

fn parse() -> Vec<(Vec<char>, Vec<char>)> {
    parser::parse(3, "\n", split_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 7737);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), 2697);
    }
}
