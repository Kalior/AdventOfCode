use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn under(self) -> Position {
        Position {
            y: self.y + 1,
            x: self.x,
        }
    }

    fn rightof(self) -> Position {
        Position {
            y: self.y,
            x: self.x + 1,
        }
    }

    fn leftof(self) -> Position {
        Position {
            y: self.y,
            x: self.x - 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
enum Type {
    Clay,
    Sand,
    StillWater,
    FlowingWater,
}

pub fn solve() {
    let map = parse();

    solve1(&map);
    solve2(&map);
}

fn parse() -> HashMap<Position, Type> {
    let filename = "input/day17input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut map = HashMap::new();

    let x_re = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    let y_re = Regex::new(r"y=(\d+), +x=(\d+)..(\d+)").unwrap();

    for cap in x_re.captures_iter(&contents) {
        let x = cap[1].parse::<i32>().unwrap();
        let y_start = cap[2].parse::<i32>().unwrap();
        let y_end = cap[3].parse::<i32>().unwrap();
        for y in y_start..y_end + 1 {
            let pos = Position {
                y: y as i32,
                x: x as i32,
            };
            map.insert(pos, Type::Clay);
        }
    }

    for cap in y_re.captures_iter(&contents) {
        let y = cap[1].parse::<i32>().unwrap();
        let x_start = cap[2].parse::<i32>().unwrap();
        let x_end = cap[3].parse::<i32>().unwrap();
        for x in x_start..x_end + 1 {
            let pos = Position {
                y: y as i32,
                x: x as i32,
            };
            map.insert(pos, Type::Clay);
        }
    }

    map
}

fn solve1(map: &HashMap<Position, Type>) {
    let largest_y = map.keys().map(|p| p.y).max().unwrap();
    let lowest_y = map.keys().map(|p| p.y).min().unwrap();

    let simulated_map = run_water_simulation(&map);

    let n_water = number_of_water(&simulated_map, largest_y, lowest_y);

    println!("Part one: {}", n_water);
}

fn number_of_water(map: &HashMap<Position, Type>, largest_y: i32, lowest_y: i32) -> i32 {
    map.iter()
        .filter(|(p, _)| p.y <= largest_y && p.y >= lowest_y)
        .map(|(_, t)| match t {
            Type::FlowingWater => 1,
            Type::StillWater => 1,
            _ => 0,
        })
        .sum::<i32>()
}

fn run_water_simulation(map: &HashMap<Position, Type>) -> HashMap<Position, Type> {
    let largest_y = map.keys().map(|p| p.y).max().unwrap();
    let mut map = map.clone();
    let start_water_pos = Position { y: 0, x: 500 };
    map.insert(start_water_pos, Type::FlowingWater);

    let mut to_consider = VecDeque::new();
    to_consider.push_back((start_water_pos, VecDeque::new()));

    while !to_consider.is_empty() {
        let (pos_to_consider, parent_positions) = to_consider.pop_back().unwrap();

        if pos_to_consider.y > largest_y {
            continue;
        }

        let sides: Vec<(Position, Type)> = [
            pos_to_consider.under(),
            pos_to_consider.leftof(),
            pos_to_consider.rightof(),
        ]
        .iter()
        .map(|pos| (*pos, map.get(pos).unwrap_or(&Type::Sand).clone()))
        .collect();

        let (under_pos, under) = sides.get(0).unwrap();

        if (under == &Type::Clay || under == &Type::StillWater)
            && in_closed_space(pos_to_consider, &map)
            && sides[1..]
                .iter()
                .any(|(_, t)| t == &Type::Clay || t == &Type::StillWater)
            && sides[1..].iter().all(|(_, t)| t != &Type::Sand)
        {
            map.insert(pos_to_consider, Type::StillWater);

            let mut new_parent_positions = parent_positions.clone();
            let first_parent = new_parent_positions.pop_back();

            if first_parent.is_some() {
                let first_parent: Position = first_parent.unwrap();
                to_consider.push_back((first_parent, new_parent_positions.clone()));
            }

            if map.get(&pos_to_consider.leftof()).unwrap_or(&Type::Sand) == &Type::FlowingWater {
                to_consider.push_back((pos_to_consider.leftof(), parent_positions.clone()));
            }

            if map.get(&pos_to_consider.rightof()).unwrap_or(&Type::Sand) == &Type::FlowingWater {
                to_consider.push_back((pos_to_consider.rightof(), parent_positions.clone()));
            }
        } else {
            if under == &Type::Sand {
                map.insert(*under_pos, Type::FlowingWater);

                let mut new_parent_positions = parent_positions.clone();
                new_parent_positions.push_back(pos_to_consider);

                to_consider.push_back((*under_pos, new_parent_positions));
            } else if under == &Type::Clay || under == &Type::StillWater {
                for (pos, pos_type) in sides[1..].iter() {
                    if pos_type == &Type::Sand {
                        map.insert(*pos, Type::FlowingWater);

                        let mut new_parent_positions = parent_positions.clone();
                        new_parent_positions.push_back(pos_to_consider);

                        to_consider.push_back((*pos, new_parent_positions));
                    }
                }
            }
        }
    }
    map
}

fn print_state(map: &HashMap<Position, Type>) {
    let last_x = map.keys().map(|p| p.x).max().unwrap();
    let first_x = map.keys().map(|p| p.x).min().unwrap();

    let last_y = map.keys().map(|p| p.y).max().unwrap();

    for y in 0..last_y + 1 {
        for x in first_x..last_x + 1 {
            let pos = Position { y: y, x: x };
            let s = match map.get(&pos).unwrap_or(&Type::Sand) {
                Type::StillWater => '~',
                Type::FlowingWater => '|',
                Type::Sand => '.',
                Type::Clay => '#',
            };
            print!("{}", s);
        }
        print!("\n");
    }
}

fn in_closed_space(position: Position, map: &HashMap<Position, Type>) -> bool {
    let mut right = position.rightof();
    let mut right_type = map.get(&right).unwrap_or(&Type::Sand);

    while right_type != &Type::Clay {
        let under = map.get(&right.under()).unwrap_or(&Type::Sand);
        if under == &Type::Sand || under == &Type::FlowingWater {
            return false;
        }

        right = right.rightof();
        right_type = map.get(&right).unwrap_or(&Type::Sand);
    }

    let mut left = position.leftof();
    let mut left_type = map.get(&left).unwrap_or(&Type::Sand);

    while left_type != &Type::Clay {
        let under = map.get(&left.under()).unwrap_or(&Type::Sand);
        if under == &Type::Sand || under == &Type::FlowingWater {
            return false;
        }

        left = left.leftof();
        left_type = map.get(&left).unwrap_or(&Type::Sand);
    }

    return true;
}

fn solve2(map: &HashMap<Position, Type>) {
    let simulated_map = run_water_simulation(&map);

    let n_water = simulated_map
        .iter()
        .map(|(_, t)| match t {
            Type::StillWater => 1,
            _ => 0,
        })
        .sum::<i32>();

    println!("Part two: {}", n_water);
}
