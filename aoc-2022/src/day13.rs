use crate::parser::parser;
use std::cmp::Ordering;
use std::iter::zip;

use crate::day13::Packet::{List, Num};

type Input = Vec<(Packet, Packet)>;

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    let two = solve2(&input);
    println!("Part two: {}", two);
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Packet {
    Num(i64),
    List(Vec<Packet>),
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, right: &Packet) -> Ordering {
        match (self, right) {
            (Num(_), List(_)) => List(vec![self.clone()]).cmp(right),
            (List(_), Num(_)) => self.cmp(&List(vec![right.clone()])),
            (Num(l), Num(r)) => l.cmp(r),
            (List(ls), List(rs)) => zip(ls, rs)
                .map(|(v, r)| v.cmp(r))
                .filter(|v| v.is_ne())
                .next()
                .unwrap_or(ls.len().cmp(&rs.len())),
        }
    }
}

fn scan_to_comma_or_bracket(line: &str, i: usize) -> usize {
    let mut i = i;
    while !line[i..i + 1].eq(",") && !line[i..i + 1].eq("]") {
        i += 1;
    }
    i
}

fn scan_to_matching_bracket(line: &str, i: usize) -> usize {
    let mut n_left_brackets_found = 0;
    let mut i = i + 1;
    while !(line[i..i + 1].eq("]") && n_left_brackets_found == 0) {
        if line[i..i + 1].eq("[") {
            n_left_brackets_found += 1;
        } else if line[i..i + 1].eq("]") {
            n_left_brackets_found -= 1;
        }

        i += 1;
    }
    i
}

fn parse_line(line: &str) -> Packet {
    let mut internal_packets = Vec::new();
    let mut i = 1;
    //println!("Rec: {}", line.to_string());
    if line.len() == 0 {
        return List(Vec::new());
    }
    while i < line.len() - 1 {
        if line[i..i + 1].eq("[") {
            let end_i = scan_to_matching_bracket(line, i);
            let p = parse_line(&line[i..end_i + 1]);
            internal_packets.push(p);
            i = end_i + 2;
        } else {
            let end_i = scan_to_comma_or_bracket(line, i);
            /*println!(
                "{} ({}, {}), ({})",
                line[i..end_i].to_string(),
                i,
                end_i,
                line.to_string()
            );*/
            let v = line[i..end_i].to_string().parse::<i64>().unwrap();
            internal_packets.push(Num(v));
            i = end_i + 1;
        }
    }

    List(internal_packets)
}

fn parse_group(lines: &str) -> (Packet, Packet) {
    let lines: Vec<&str> = lines.lines().collect();
    (parse_line(lines[0]), parse_line(lines[1]))
}

fn parse() -> Input {
    parser::parse(13, "\n\n", parse_group)
}

fn is_pair_sorted(packets: &(Packet, Packet)) -> bool {
    packets.0.cmp(&packets.1).is_lt()
}

fn solve1(pairs: &Input) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| is_pair_sorted(*pair))
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve2(packets: &Input) -> usize {
    let mut all_packets = Vec::new();
    for pair in packets {
        all_packets.push(pair.0.clone());
        all_packets.push(pair.1.clone());
    }

    let first_divider = List(vec![List(vec![Num(2)])]);
    let second_divider = List(vec![List(vec![Num(6)])]);

    all_packets.push(first_divider.clone());
    all_packets.push(second_divider.clone());

    all_packets.sort();

    let first_i = all_packets
        .iter()
        .position(|v| v.eq(&first_divider))
        .unwrap()
        + 1;
    let second_i = all_packets
        .iter()
        .position(|v| v.eq(&second_divider))
        .unwrap()
        + 1;

    first_i * second_i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("[]"), List(Vec::new()));
        assert_eq!(parse_line("[[7]]"), List(vec![List(vec![Num(7)])]));
        assert_eq!(
            parse_line("[3,3,9,9,9]"),
            List(vec![Num(3), Num(3), Num(9), Num(9), Num(9)])
        );
        assert_eq!(
            parse_line("[[[9,[0],7,[6,6,10],7],[2,9,9]]]"),
            List(vec![List(vec![
                List(vec![
                    Num(9),
                    List(vec![Num(0)]),
                    Num(7),
                    List(vec![Num(6), Num(6), Num(10)]),
                    Num(7)
                ]),
                List(vec![Num(2), Num(9), Num(9)])
            ])])
        );
    }

    #[test]
    fn test_scan_to_matching_bracket() {
        assert_eq!(scan_to_matching_bracket("[[7]]", 1), 3);
        assert_eq!(
            scan_to_matching_bracket("[9,[0],7,[6,6,10],7],[2,9,9]", 3),
            5
        );
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 5185);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(&input), 23751);
    }
}
