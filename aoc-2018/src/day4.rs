use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;

use regex::Regex;

#[derive(Debug, Eq)]
struct Entry {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    action: Action,
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        (self.year, self.month, self.day, self.hour, self.minute).cmp(&(
            other.year,
            other.month,
            other.day,
            other.hour,
            other.minute,
        ))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    BeginShift { guard_id: i32 },
    FallAsleep,
    WakeUp,
}

pub fn solve() {
    let entries = parse();

    // println!("{:?}", entries);
    let answer1: i32 = solve1(entries.as_slice());

    println!("{:?}", answer1);

    let answer2 = solve2(entries.as_slice());

    println!("{:?}", answer2);
}

fn parse() -> Vec<Entry> {
    let filename = "input/day4input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // [1518-05-23 00:59] wakes up
    let re = Regex::new(r".(\d+).(\d+).(\d+) (\d+).(\d+). (.+)").unwrap();

    let mut entries: Vec<Entry> = re
        .captures_iter(&contents)
        .map(|cap| Entry {
            year: cap[1].parse::<i32>().unwrap_or(-1),
            month: cap[2].parse::<i32>().unwrap_or(-1),
            day: cap[3].parse::<i32>().unwrap_or(-1),
            hour: cap[4].parse::<i32>().unwrap_or(-1),
            minute: cap[5].parse::<i32>().unwrap_or(-1),
            action: parse_action(&cap[6]),
        })
        .collect();
    entries.sort_unstable();
    entries
}

fn parse_action(action: &str) -> Action {
    if action == "wakes up" {
        return Action::WakeUp;
    } else if action == "falls asleep" {
        return Action::FallAsleep;
    } else {
        let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        return re
            .captures_iter(action)
            .map(|cap| Action::BeginShift {
                guard_id: cap[1].parse::<i32>().unwrap_or(-1),
            })
            .next()
            .unwrap();
    }
}

fn solve1(entries: &[Entry]) -> i32 {
    let sleeping = create_sleeping_schedule(entries);

    let (guard_id, _, guard_times) = sleeping.iter().fold(
        (0, 0, Vec::<i32>::new()),
        |(guard_id, longest_sleep, guard_times), (new_guard_id, times)| {
            if times.len() >= longest_sleep {
                (*new_guard_id, times.len(), times.to_vec())
            } else {
                (guard_id, longest_sleep, guard_times)
            }
        },
    );

    let (most_sleeping_minute, _) = get_most_sleeping_minute(guard_times.to_vec());

    return guard_id * most_sleeping_minute;
}

fn create_sleeping_schedule(entries: &[Entry]) -> HashMap<i32, Vec<i32>> {
    let mut sleeping = HashMap::new();

    let mut current_guard = 0;
    let mut fell_asleep_at = 0;
    for entry in entries {
        match entry.action {
            Action::BeginShift { guard_id } => current_guard = guard_id,
            Action::FallAsleep => fell_asleep_at = entry.minute,
            Action::WakeUp => {
                insert_sleeping_time(&mut sleeping, current_guard, fell_asleep_at, entry.minute)
            }
        }
    }
    sleeping
}

fn insert_sleeping_time(
    sleeping: &mut HashMap<i32, Vec<i32>>,
    guard_id: i32,
    fell_asleep_at: i32,
    woke_up_at: i32,
) {
    let mut times = sleeping.get(&guard_id).unwrap_or(&Vec::new()).to_vec();
    for minute in fell_asleep_at..woke_up_at {
        times.push(minute);
    }
    sleeping.insert(guard_id, times.to_vec());
}

fn solve2(entries: &[Entry]) -> i32 {
    let sleeping = create_sleeping_schedule(entries);

    let mut most_sleeping_guard_id = 0;
    let mut most_sleeping_guard_minute = 0;
    let mut minutes_spent_asleep = 0;
    for (guard_id, times) in &sleeping {
        let (most_sleeping_minute, most_sleeping_times) = get_most_sleeping_minute(times.to_vec());

        if most_sleeping_times >= minutes_spent_asleep {
            minutes_spent_asleep = most_sleeping_times;
            most_sleeping_guard_minute = most_sleeping_minute;
            most_sleeping_guard_id = *guard_id;
        }
    }

    most_sleeping_guard_id * most_sleeping_guard_minute
}

fn get_most_sleeping_minute(times: Vec<i32>) -> (i32, i32) {
    let mut minutes = HashMap::new();
    for minute in times {
        let counter = minutes.entry(minute).or_insert(0);
        *counter += 1;
    }

    let mut most_sleeping_minute = 0;
    let mut most_sleeping_times = 0;
    for (minute, times) in minutes {
        if times >= most_sleeping_times {
            most_sleeping_times = times;
            most_sleeping_minute = minute;
        }
    }

    return (most_sleeping_minute, most_sleeping_times);
}
