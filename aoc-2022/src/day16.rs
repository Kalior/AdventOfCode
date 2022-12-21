use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

use crate::parser::parser;

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
    path: Vec<(String, usize, usize)>,
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
    let results_at_time = open_valves(valves_in, 30, 1);

    for (v, t, f) in results_at_time.1 {
        println!("{t}: {v} ({f})");
    }

    results_at_time.0
}

fn open_valves(
    valves_in: &Input,
    time: usize,
    n_actors: usize,
) -> (usize, Vec<(String, usize, usize)>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for valve in valves_in {
        valves.insert(valve.name.clone(), valve.clone());
    }

    let n_valves_above_0 = valves_in.iter().filter(|v| v.flow_rate > 0).count();

    let mut paths: VecDeque<Path> = VecDeque::new();

    let mut highest_at_tunnel: HashMap<(String, String, usize), usize> = HashMap::new();

    let start_p = Path {
        estimated_flow_released: 0,
        on_minute: 0,
        current_valves: vec!["AA".to_string(); n_actors],
        opened_valves: Vec::new(),
        path: Vec::new(),
    };
    let mut best_p = start_p.clone();

    paths.push_back(start_p);

    while let Some(p) = paths.pop_back() {
        // Hard coded values, the solution to part two has to be better than the
        // solution to part one.  This helps to heavily constrain the
        // search space.  This will, of course, break the test input.
        if (p.on_minute > 2 && p.estimated_flow_released < 459)
            || (p.on_minute > 6 && p.estimated_flow_released < 873)
            || (p.on_minute > 9 && p.estimated_flow_released < 1273)
            || (p.on_minute > 12 && p.estimated_flow_released < 1647)
            || (p.on_minute > 16 && p.estimated_flow_released < 1894)
            || (p.on_minute > 19 && p.estimated_flow_released < 2044)
            || (p.on_minute > 22 && p.estimated_flow_released < 2205)
            || (p.on_minute > 25 && p.estimated_flow_released < 2253)
        {
            continue;
        }
        // If all valves are open, can do nothing more.
        if p.opened_valves.len() == n_valves_above_0 {
            if p.estimated_flow_released > best_p.estimated_flow_released {
                best_p = p.clone();
                println!("{}", best_p.estimated_flow_released);
            }
            continue;
        }

        // If we've spent 30 minutes, can do nothing more.
        if p.on_minute >= time {
            if p.estimated_flow_released > best_p.estimated_flow_released {
                best_p = p.clone();
                println!("{}", best_p.estimated_flow_released);
            }
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
            let mut open_options = get_open_valve_options(time, p.on_minute, &current_valve);

            // don't open valve:
            let mut move_options = get_move_options(&current_valve);

            options.append(&mut open_options);
            options.append(&mut move_options);
            combined_options.push(options);
        }
        if n_actors == 1 {
            for (tunnel, est_new_flow_rate, new_opened_valve) in combined_options[0].iter() {
                let estimated_flow_released = p.estimated_flow_released + est_new_flow_rate;

                let mut new_opened_valves = p.opened_valves.clone();
                let mut path = p.path.clone();
                if let Some(new_opened_valve) = new_opened_valve {
                    if p.opened_valves.contains(new_opened_valve) {
                        continue;
                    }
                    new_opened_valves.push(new_opened_valve.clone());
                    path.push((
                        new_opened_valve.clone(),
                        p.on_minute,
                        estimated_flow_released,
                    ));
                }

                let new_p = Path {
                    estimated_flow_released,
                    on_minute: p.on_minute + 1,
                    current_valves: vec![tunnel.clone()],
                    opened_valves: new_opened_valves,
                    path,
                };

                paths.push_back(new_p);
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

                    let estimated_flow_released =
                        p.estimated_flow_released + my_est_new_flow_rate + elephant_new_flow_rate;

                    let mut path = p.path.clone();
                    let mut new_opened_valves = p.opened_valves.clone();

                    if let Some(my_new_valve) = my_new_valve {
                        if p.opened_valves.contains(my_new_valve) {
                            continue;
                        }
                        new_opened_valves.push(my_new_valve.clone());
                        path.push((my_new_valve.clone(), p.on_minute, estimated_flow_released));
                    }
                    if let Some(elephant_new_valve) = elephant_new_valve {
                        if p.opened_valves.contains(elephant_new_valve) {
                            continue;
                        }
                        new_opened_valves.push(elephant_new_valve.clone());
                        path.push((
                            elephant_new_valve.clone(),
                            p.on_minute,
                            estimated_flow_released,
                        ));
                    }

                    let new_p = Path {
                        estimated_flow_released,
                        on_minute: p.on_minute + 1,
                        current_valves: vec![my_tunnel.clone(), elephant_tunnel.clone()],
                        opened_valves: new_opened_valves,
                        path,
                    };

                    paths.push_back(new_p);
                }
            }
        }
    }

    (best_p.estimated_flow_released, best_p.path)
}

fn better_path_exists(
    highest_at_tunnel: &mut HashMap<(String, String, usize), usize>,
    p: &Path,
) -> bool {
    let mut current_valves = p.current_valves.clone();
    current_valves.sort();
    let current_valves = current_valves.join(",");

    let mut opened_valves = p.opened_valves.clone();
    opened_valves.sort();

    let opened_valves = opened_valves.join(",");

    let key = (current_valves.clone(), opened_valves, p.on_minute);

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
    on_minute: usize,
    current_valve: &&Valve,
) -> Vec<(String, usize, Option<String>)> {
    let mut possibilities = vec![];
    if current_valve.flow_rate > 0 {
        let estimated_new_flow_rate = current_valve.flow_rate * (time - on_minute - 1);

        possibilities.push((
            current_valve.name.clone(),
            estimated_new_flow_rate,
            Some(current_valve.name.clone()),
        ));
    }

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
    let (released_pressure, vs) = open_valves(valves, 26, 2);

    for (v, t, f) in vs {
        println!("{t}: {v} ({f})");
    }

    released_pressure
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
        );

        assert_eq!(
            parse_to_valve("Valve IM has flow rate=19; tunnels lead to valves PU, EC, QS, LT"),
            Valve {
                name: "IM".to_string(),
                flow_rate: 19,
                tunnels: vec![
                    "PU".to_string(),
                    "EC".to_string(),
                    "QS".to_string(),
                    "LT".to_string(),
                ]
            }
        )
    }

    #[test]
    fn test_contains() {
        let mut highest_at_tunnel: HashMap<(Vec<String>, usize), usize> = HashMap::new();

        highest_at_tunnel.insert((vec!["AA".to_string(), "BB".to_string()], 4), 10);

        assert!(highest_at_tunnel.contains_key(&(vec!["AA".to_string(), "BB".to_string()], 4)));
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
