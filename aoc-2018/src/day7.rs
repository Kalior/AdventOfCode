use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;
// use std::slice::SliceConcatExt;
use std::cmp::*;

pub fn solve() {
    let steps = parse();
    // println!("{:?}", coordinates);

    let answer1 = solve1(&steps.clone());
    println!("{:?}", answer1);
    let answer2 = solve2(&steps.clone());
    println!("{:?}", answer2);
}

fn parse() -> HashMap<String, Vec<String>> {
    let filename = "input/day7input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let mut steps = HashMap::new();

    re.captures_iter(&contents).for_each(|cap| {
        let key = cap[2].to_string();
        let mut pre_reqs = steps.get(&key).unwrap_or(&Vec::new()).to_vec();
        pre_reqs.push(cap[1].to_string());
        steps.insert(key, pre_reqs.to_vec());
    });
    steps
}

fn solve1(steps: &HashMap<String, Vec<String>>) -> String {
    let available_chars = get_available_chars(&steps);

    let mut steps_done: Vec<String> = Vec::new();
    let mut steps_ready = get_available_steps(&steps, &available_chars, &steps_done, &steps_done);

    while steps_ready.len() != 0 {
        let step = steps_ready.remove(0);

        steps_done.push(step);

        steps_ready = get_available_steps(&steps, &available_chars, &steps_done, &steps_done);
    }

    steps_done.join("")
}

fn get_available_chars(steps: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut available_chars: Vec<String> = steps
        .iter()
        .flat_map(|(_, pre_reqs)| pre_reqs.to_vec())
        .collect();
    steps
        .keys()
        .for_each(|key| available_chars.push(key.to_string()));

    available_chars.sort_unstable();
    available_chars.dedup();
    available_chars
}

fn get_available_steps(
    steps: &HashMap<String, Vec<String>>,
    available_chars: &Vec<String>,
    steps_done: &Vec<String>,
    steps_dealt_with: &Vec<String>,
) -> Vec<String> {
    let mut available: Vec<String> = available_chars
        .iter()
        .filter(|step| {
            steps
                .get(*step)
                .unwrap_or(&Vec::new())
                .iter()
                .all(|pre_req| steps_done.contains(pre_req))
                && !steps_dealt_with.contains(step)
        })
        .map(|step| step.to_string())
        .collect();

    available.sort_unstable();
    available
}

fn solve2(steps: &HashMap<String, Vec<String>>) -> i32 {
    let available_chars = get_available_chars(&steps);

    let mut available_workers = 5;
    let mut time_now = 0;

    let mut steps_done: Vec<String> = Vec::new();
    let mut steps_dealt_with: Vec<String> = Vec::new();
    let mut steps_ready =
        get_available_steps(&steps, &available_chars, &steps_done, &steps_dealt_with);
    let mut running_steps = Vec::new();

    let time_lookup: HashMap<String, i32> = available_chars
        .iter()
        .enumerate()
        .map(|(i, k)| (k.to_string(), i as i32 + 1))
        .collect();

    loop {
        if available_workers > 0 && steps_ready.len() != 0 {
            if steps_ready.len() == 0 && running_steps.len() == 0 {
                break;
            }

            let step = steps_ready.remove(0);

            let time_stop = time_now + 60 + time_lookup.get(&step).unwrap_or(&0);

            running_steps.push((step.to_string(), time_stop));
            steps_dealt_with.push(step.to_string());
            available_workers -= 1;
        } else {
            if running_steps.len() == 0 {
                break;
            }
            let (step, time_done) = running_steps.iter().fold(
                (String::from(""), 99999999),
                |(last_step, last_time_done), (step, last_time)| {
                    if *last_time < last_time_done {
                        (step.to_string(), *last_time)
                    } else {
                        (last_step, last_time_done)
                    }
                },
            );

            running_steps = running_steps
                .iter()
                .filter(|(s, t)| *s != step)
                .map(|(s, t)| (s.to_string(), *t))
                .collect();

            time_now = time_done;
            available_workers = min(5, available_workers + 1);

            steps_done.push(step);
            steps_ready =
                get_available_steps(&steps, &available_chars, &steps_done, &steps_dealt_with);
        }
    }

    time_now
}
