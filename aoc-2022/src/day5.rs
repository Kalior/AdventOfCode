use std::collections::VecDeque;
use crate::parser::parser;


pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn parse_stacks(stacks: &str) -> Vec<VecDeque<char>> {
    let mut parsed_stacks = vec![];
    for line in stacks.lines() {
        if !line.contains("[") {
            continue;
        }

        let line: Vec<char> = line.chars().collect();
        for (stack_i, line_i) in (1..line.len()).step_by(4).enumerate() {
            if parsed_stacks.len() <= stack_i {
                parsed_stacks.push(VecDeque::new());
            }
            if line[line_i] != ' ' {
                parsed_stacks[stack_i].push_front(line[line_i]);
            }
        }
    }

    return parsed_stacks;
}

fn parse() -> Vec<String> {
    parser::parse(5, "\n\n", |group| group.to_string()).to_vec()
}

fn solve1(input: Vec<String>) -> String {
    let mut stacks = parse_stacks(&input[0]);

    for line in input[1].lines() {
        if line.is_empty() {
            continue;
        }
        let splits: Vec<&str> = line.split(" ").collect();
        let n_to_move = splits[1].parse::<usize>().unwrap();
        let move_from = splits[3].parse::<usize>().unwrap() - 1;
        let move_to = splits[5].parse::<usize>().unwrap() - 1;

        for _ in 0..n_to_move {
            let v = stacks[move_from].pop_back().expect("Error, empty stack");
            stacks[move_to].push_back(v);
        }
    }

    for stack in stacks {
        print!("{}", stack.back().unwrap());
    }

    return "".to_string();
}

fn solve2(input: Vec<String>) -> String {
    let mut stacks = parse_stacks(&input[0]);

    for stack in stacks.iter() {
        for v in stack {
            print!("{} ", v);
        }
        println!();
    }

    for line in input[1].lines() {
        if line.is_empty() {
            continue;
        }
        let splits: Vec<&str> = line.split(" ").collect();
        let n_to_move = splits[1].parse::<usize>().unwrap();
        let move_from = splits[3].parse::<usize>().unwrap() - 1;
        let move_to = splits[5].parse::<usize>().unwrap() - 1;

        let mut to_move = VecDeque::new();
        for _ in 0..n_to_move {
            let v = stacks[move_from].pop_back().expect("Error, empty stack");
            to_move.push_front(v);
        }

        for v in to_move.iter() {
            stacks[move_to].push_back(*v);
        }
    }

    for stack in stacks {
        print!("{}", stack.back().unwrap());
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), "");
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), "");
    }
}
