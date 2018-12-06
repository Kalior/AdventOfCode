use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
extern crate rayon;
use rayon::prelude::*;

pub fn solve() {
    let polymer = parse();

    let answer1 = solve1(&polymer.clone());

    println!("{:?}", answer1);

    let answer2 = solve2(&polymer.clone());

    println!("{:?}", answer2);
}

fn parse() -> String {
    let filename = "input/day5input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.lines().next().unwrap().to_string()
}

fn solve1(polymer: &String) -> usize {
    let remaining_polymer = fully_react_polymer(&polymer);

    return remaining_polymer.len();
}

fn solve2(polymer: &String) -> usize {
    let mut alphabet = HashSet::new();
    for c in polymer.clone().chars() {
        alphabet.insert(c.to_ascii_uppercase());
    }

    let reacted_length = alphabet
        .par_iter()
        .map(|key| {
            let removed = polymer.chars().fold(Vec::new(), |mut acc, c| {
                if c.to_ascii_uppercase() != *key {
                    acc.push(c);
                }
                acc
            });

            let remaining_polymer = fully_react_polymer(&removed.iter().collect());
            remaining_polymer.len()
        })
        .min()
        .unwrap();
    reacted_length
}

fn fully_react_polymer(polymer: &String) -> Vec<char> {
    let mut remaining_polymer: Vec<char> = polymer.clone().chars().collect();
    let mut changed = true;
    while changed {
        let (mut changed_polymer, last_charcter) =
            remaining_polymer
                .iter()
                .fold((Vec::<char>::new(), '0'), |(mut acc, prev), c| {
                    if *c != prev && ((*c).to_ascii_uppercase() == prev.to_ascii_uppercase()) {
                        (acc, '0')
                    } else if prev != '0' {
                        acc.push(prev);
                        (acc, *c)
                    } else {
                        (acc, *c)
                    }
                });
        if last_charcter != '0' {
            changed_polymer.push(last_charcter);
        }

        changed = remaining_polymer.len() != changed_polymer.len();
        remaining_polymer = changed_polymer.clone();
    }

    remaining_polymer
}
