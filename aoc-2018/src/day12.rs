extern crate regex;
use itertools::zip;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(PartialEq, Debug, Clone)]
enum Pot {
    Empty,
    Plant,
}

#[derive(Debug, Clone)]
struct Pattern {
    pattern: Vec<Pot>,
    result: Pot,
}
impl Pattern {
    fn matches(&self, input: &Vec<Pot>) -> bool {
        zip(&self.pattern, input).all(|(x, y)| *x == *y)
    }
}

pub fn solve() {
    let (state, patterns) = parse();

    let task1_now = Instant::now();

    solve1(state.clone(), patterns.clone());

    println!("{}ms", task1_now.elapsed().as_millis());

    let task2_now = Instant::now();

    solve2(state.clone(), patterns.clone());

    println!(
        "{}ms, both: {}ms",
        task2_now.elapsed().as_millis(),
        task1_now.elapsed().as_millis()
    );
}

fn parse() -> (Vec<Pot>, Vec<Pattern>) {
    let filename = "input/day12input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let raw_state = &contents.lines().next().unwrap().clone();
    let state = raw_state[15..115]
        .chars()
        .map(|c| match c {
            '.' => Pot::Empty,
            '#' => Pot::Plant,
            _ => Pot::Empty,
        })
        .collect();

    // .###. => #
    let re = Regex::new(r"(.+) => (.)").unwrap();

    let patterns: Vec<Pattern> = re
        .captures_iter(&contents)
        .map(|cap| parse_pattern(cap[1].to_string(), cap[2].to_string()))
        .collect();

    (state, patterns)
}

fn parse_pattern(pattern: String, result: String) -> Pattern {
    let pattern = pattern
        .chars()
        .map(|c| match c {
            '.' => Pot::Empty,
            '#' => Pot::Plant,
            _ => Pot::Empty,
        })
        .collect();
    let result = match result.as_ref() {
        "." => Pot::Empty,
        "#" => Pot::Plant,
        _ => Pot::Empty,
    };

    Pattern {
        pattern: pattern,
        result: result,
    }
}

fn solve1(initial_state: Vec<Pot>, patterns: Vec<Pattern>) {
    let mut state = HashMap::new();
    for (i, p) in initial_state.iter().enumerate() {
        state.insert(i as i32, p.clone());
    }

    for _ in 0..20 {
        state = next_generation(state, &patterns);
    }
    let plant_index_sum = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| i)
        .sum::<i32>();

    println!("Part 1: {:?}", plant_index_sum);
}

fn next_generation(state: HashMap<i32, Pot>, patterns: &Vec<Pattern>) -> HashMap<i32, Pot> {
    let leftmost_plant_index = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| i)
        .min()
        .unwrap();
    let rightmost_plant_index = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| i)
        .max()
        .unwrap();

    let new_state: HashMap<i32, Pot> = (leftmost_plant_index - 10..rightmost_plant_index + 10)
        .map(|i| {
            let input: Vec<Pot> = (-2..3)
                .map(|j| state.get(&(i + j)).unwrap_or(&Pot::Empty).clone())
                .collect();

            let pattern = patterns
                .iter()
                .filter(|&pattern| pattern.matches(&input))
                .next();

            match pattern {
                Some(p) => (i, p.result.clone()),
                _ => (i, Pot::Empty),
            }
        })
        .collect();

    new_state
}

fn state_string(state: &HashMap<i32, Pot>) -> String {
    let leftmost_plant_index = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| i)
        .min()
        .unwrap();
    let rightmost_plant_index = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| i)
        .max()
        .unwrap();

    let state_str: Vec<String> = (leftmost_plant_index - 10..rightmost_plant_index + 10)
        .map(|i| match state.get(&i).unwrap_or(&Pot::Empty) {
            Pot::Plant => String::from("#"),
            Pot::Empty => String::from("."),
        })
        .collect();
    state_str.join("")
}

fn solve2(initial_state: Vec<Pot>, patterns: Vec<Pattern>) {
    let mut state = HashMap::new();
    for (i, p) in initial_state.iter().enumerate() {
        state.insert(i as i32, p.clone());
    }

    let mut prev_states = HashMap::new();

    let total_generations: i64 = 50000000000i64;

    let mut gen_index = 0;
    let mut sum_diff = 0;

    for i in 0..1000 {
        state = next_generation(state, &patterns);

        let state_string = state_string(&state);

        let plant_index_sum = &state
            .iter()
            .filter(|(_, p)| **p == Pot::Plant)
            .map(|(i, _)| (*i as i64))
            .sum::<i64>();

        if prev_states.contains_key(&state_string) {
            let prev_value = prev_states.get(&state_string).unwrap();
            sum_diff = plant_index_sum - prev_value;
            gen_index = i;
            break;
        }
        prev_states.insert(state_string.clone(), *plant_index_sum);
    }

    let generations_left = total_generations - gen_index - 1;
    let offset = generations_left * sum_diff as i64;

    let plant_index_sum = state
        .iter()
        .filter(|(_, p)| **p == Pot::Plant)
        .map(|(i, _)| (*i as i64))
        .sum::<i64>()
        + offset;

    println!("Part 2: {:?}", plant_index_sum);
}
