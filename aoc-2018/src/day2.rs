use itertools::zip;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let ids = parse();

    let answer1: i32 = solve1(ids.as_slice());
    println!("{:?}", answer1);
    let answer2 = solve2(ids.as_slice());
    println!("{:?}", answer2);
}

fn parse() -> Vec<String> {
    let filename = "input/day2input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.lines().map(|l| String::from(l)).collect()
}

fn solve1(ids: &[String]) -> i32 {
    let mut three_counts = 0;
    let mut two_counts = 0;
    for line in ids {
        let mut counts = HashMap::new();
        for s in line.chars() {
            counts.insert(s, counts.get(&s).unwrap_or(&0) + &1);
        }
        for (character, count) in &counts {
            if count == &2 {
                two_counts += 1;
                break;
            }
        }

        for (character, count) in &counts {
            if count == &3 {
                three_counts += 1;
                break;
            }
        }
    }

    return three_counts * two_counts;
}

fn solve2(ids: &[String]) -> String {
    for outer_line in ids {
        for inner_line in ids {
            let mut differing = 0;
            let mut diff_index = 0;
            for (i, (c1, c2)) in zip(outer_line.chars(), inner_line.chars()).enumerate() {
                if c1 != c2 {
                    if differing == 0 {
                        diff_index = i;
                        differing += 1;
                    } else {
                        differing += 1;
                        break;
                    }
                }
            }
            if differing == 1 {
                let mut new_line = inner_line.clone();
                new_line.remove(diff_index);
                return new_line;
            }
        }
    }
    return String::from("");
}
