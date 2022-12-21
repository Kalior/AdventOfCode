use crate::parser::parser;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::zip;

type Input = Vec<Valve>;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Valve {
    name: String,
    tunnels: Vec<String>,
    flow_rate: usize,
}

fn parse_to_valve(line: &str) -> Valve {
    // Valve AA has flow rate=0; tunnels lead to valves YS, XT, TE, GY, FS

    let splits: Vec<&str> = line.split(" ").collect();

    let name = splits[1].to_string();
    let flow_rate = splits[4]
        .split("=")
        .last()
        .unwrap()
        .replace(";", "")
        .parse::<usize>()
        .unwrap();

    let tunnels = splits[9..]
        .iter()
        .map(|v| v.replace(",", "").to_string())
        .collect();

    Valve {
        name,
        flow_rate,
        tunnels,
    }
}

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    let two = solve2(&input);
    println!("Part two: {}", two);
}

fn parse() -> Input {
    parser::parse(16, "\n", parse_to_valve)
}

fn parse_test() -> Input {
    parser::parse_str("16test", "\n", parse_to_valve)
}

#[derive(Clone, Eq, PartialEq, PartialOrd)]
struct Path {
    estimated_flow_released: usize,
    on_minute: usize,
    current_valves: Vec<String>,
    opened_valves: Vec<String>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.estimated_flow_released
            .cmp(&other.estimated_flow_released)
            .then_with(|| self.on_minute.cmp(&other.on_minute))
    }
}

fn solve1(valves_in: &Input) -> usize {
    let results_at_time = open_valves(valves_in, 30, 1, Vec::new());

    results_at_time.0
}

fn open_valves(
    valves_in: &Input,
    time: usize,
    n_actors: usize,
    opened_valves: Vec<String>,
) -> (usize, Vec<String>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for valve in valves_in {
        valves.insert(valve.name.clone(), valve.clone());
    }

    let mut paths: BinaryHeap<Path> = BinaryHeap::new();

    let mut highest_at_tunnel: HashMap<(Vec<String>, usize), usize> = HashMap::new();

    let start_p = Path {
        estimated_flow_released: 0,
        on_minute: 0,
        current_valves: vec!["AA".to_string(); n_actors],
        opened_valves: opened_valves.clone(),
    };
    paths.push(start_p);

    let mut highest_at_time: Vec<(usize, Vec<String>)> = Vec::new();

    while let Some(p) = paths.pop() {
        // If all valves are open, can do nothing more.
        if p.opened_valves.len() == valves.len() {
            highest_at_time.push((p.estimated_flow_released, p.opened_valves.clone()));
            continue;
        }

        // If we've spent 30 minutes, can do nothing more.
        if p.on_minute >= time {
            highest_at_time.push((p.estimated_flow_released, p.opened_valves.clone()));
            continue;
        }

        // Probably not worth continuing this path if we've been here at this minute before with
        // a higher estimated flow release value.
        if better_path_exists(&mut highest_at_tunnel, &p) {
            continue;
        }

        let mut combined_options = Vec::new();
        for position in p.current_valves.iter() {
            // Can choose to either open valve and spend a minute, or not open valve
            // and move on.
            let current_valve = valves.get(position).unwrap();

            let mut options: Vec<(String, usize, Option<String>)> = Vec::new();
            // open valve:
            let mut open_options = get_open_valve_options(time, &p, &current_valve);

            // don't open valve:
            let mut move_options = get_move_options(&current_valve);

            options.append(&mut open_options);
            options.append(&mut move_options);
            combined_options.push(options);
        }
        if n_actors == 1 {
            for (tunnel, est_new_flow_rate, new_opened_valve) in combined_options[0].iter() {
                let mut new_opened_valves = p.opened_valves.clone();
                if new_opened_valve.is_some() {
                    if p.opened_valves.contains(&new_opened_valve.clone().unwrap()) {
                        continue;
                    }
                    new_opened_valves.push(new_opened_valve.clone().unwrap());
                }

                let new_p = Path {
                    estimated_flow_released: p.estimated_flow_released + est_new_flow_rate,
                    on_minute: p.on_minute + 1,
                    current_valves: vec![tunnel.clone()],
                    opened_valves: new_opened_valves,
                };

                paths.push(new_p);
            }
        } else if n_actors == 2 {
            for (my_tunnel, my_est_new_flow_rate, my_new_valve) in combined_options[0].iter() {
                for (elephant_tunnel, elephant_new_flow_rate, elephant_new_valve) in
                    combined_options[1].iter()
                {
                    if my_new_valve.is_some() && my_new_valve == elephant_new_valve {
                        // Both can't open the same valve!
                        continue;
                    }

                    let mut new_opened_valves = p.opened_valves.clone();
                    if my_new_valve.is_some() {
                        if new_opened_valves.contains(&my_new_valve.clone().unwrap()) {
                            continue;
                        }
                        new_opened_valves.push(my_new_valve.clone().unwrap());
                    }
                    if elephant_new_valve.is_some() {
                        if new_opened_valves.contains(&elephant_new_valve.clone().unwrap()) {
                            continue;
                        }
                        new_opened_valves.push(elephant_new_valve.clone().unwrap());
                    }

                    let new_p = Path {
                        estimated_flow_released: p.estimated_flow_released
                            + my_est_new_flow_rate
                            + elephant_new_flow_rate,
                        on_minute: p.on_minute + 1,
                        current_valves: vec![my_tunnel.clone(), elephant_tunnel.clone()],
                        opened_valves: new_opened_valves,
                    };

                    paths.push(new_p);
                }
            }
        }
    }

    /*for (val, opened) in highest_at_time.iter() {
        print!("{val}: ");
        for open in opened.iter() {
            print!("{open} ")
        }
        println!();
    }*/

    highest_at_time.sort_by_key(|v| v.0);

    highest_at_time[highest_at_time.len() - 1].clone()
}

fn better_path_exists(
    highest_at_tunnel: &mut HashMap<(Vec<String>, usize), usize>,
    p: &Path,
) -> bool {
    let key = (p.current_valves.clone(), p.on_minute);
    if highest_at_tunnel.contains_key(&key) {
        if highest_at_tunnel[&key] >= p.estimated_flow_released {
            true
        } else {
            highest_at_tunnel.insert(key.clone(), p.estimated_flow_released);
            false
        }
    } else {
        highest_at_tunnel.insert(key.clone(), p.estimated_flow_released);
        false
    }
}

fn get_open_valve_options(
    time: usize,
    p: &Path,
    current_valve: &&Valve,
) -> Vec<(String, usize, Option<String>)> {
    let mut possibilities = vec![];
    let estimated_new_flow_rate = current_valve.flow_rate * (time - p.on_minute - 1);

    possibilities.push((
        current_valve.name.clone(),
        estimated_new_flow_rate,
        Some(current_valve.name.clone()),
    ));

    possibilities
}

fn get_move_options(current_valve: &&Valve) -> Vec<(String, usize, Option<String>)> {
    let mut possibilities = vec![];
    for tunnel in &current_valve.tunnels {
        possibilities.push((tunnel.clone(), 0, None));
    }

    possibilities
}

fn solve2(valves: &Input) -> usize {
    let (released_pressure, opened) = open_valves(valves, 26, 1, Vec::new());

    let (elephant_released, elephant_opened) = open_valves(valves, 26, 1, opened);

    released_pressure + elephant_released
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valve() {
        assert_eq!(
            parse_to_valve("Valve AA has flow rate=0; tunnels lead to valves YS, XT, TE, GY, FS"),
            Valve {
                name: "AA".to_string(),
                flow_rate: 0,
                tunnels: vec![
                    "YS".to_string(),
                    "XT".to_string(),
                    "TE".to_string(),
                    "GY".to_string(),
                    "FS".to_string()
                ]
            }
        )
    }

    #[test]
    fn part_one_test_input() {
        let input = parse_test();
        assert_eq!(solve1(&input), 1651);
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 2253);
    }

    #[test]
    fn part_two_test_input() {
        let input = parse_test();
        assert_eq!(solve2(&input), 1707);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(&input), 0);
    }
}
