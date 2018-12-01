use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let numbers = parse();

    let answer1: i32 = solve1(numbers.as_slice());
    println!("{:?}", answer1);
    let answer2 = solve2(numbers.as_slice());
    println!("{:?}", answer2);
}

fn parse() -> Vec<i32> {
    let filename = "input/day1input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn solve1(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn solve2(numbers: &[i32]) -> i32 {
    let mut memory = HashSet::new();

    numbers
        .iter()
        .cycle()
        .fold_while(0, |acc, x| {
            memory.insert(acc);
            let frequency = acc + x;
            if memory.contains(&frequency) {
                Done(frequency)
            } else {
                Continue(frequency)
            }
        })
        .into_inner()
}
