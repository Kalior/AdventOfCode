use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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

    fn above(self) -> Position {
        Position {
            y: self.y - 1,
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

    fn adjacent(self) -> Vec<Position> {
        vec![
            self.above(),
            self.above().leftof(),
            self.leftof(),
            self.leftof().under(),
            self.under(),
            self.under().rightof(),
            self.rightof(),
            self.rightof().above(),
        ]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
enum Type {
    Open,
    Trees,
    Lumberyard,
}

pub fn solve() {
    let map = parse();
    solve1(&map);
    solve2(&map);
}

fn parse() -> HashMap<Position, Type> {
    let filename = "input/day18input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut map = HashMap::new();

    for (y, l) in contents.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = Position {
                y: y as i32,
                x: x as i32,
            };

            if c == '.' {
                map.insert(pos, Type::Open);
            } else if c == '|' {
                map.insert(pos, Type::Trees);
            } else if c == '#' {
                map.insert(pos, Type::Lumberyard);
            }
        }
    }

    map
}

fn solve1(map: &HashMap<Position, Type>) {
    let mut map = map.clone();

    for _ in 0..10 {
        map = map
            .iter()
            .map(|(&pos, pos_type)| match pos_type {
                Type::Lumberyard => (pos, tick_lumberyard(pos, &map)),
                Type::Open => (pos, tick_open(pos, &map)),
                Type::Trees => (pos, tick_trees(pos, &map)),
            })
            .collect();

        print_lumber_collection_area(&map);
    }

    let score = score(&map);

    println!("Part one: {}", score);
}

fn print_lumber_collection_area(map: &HashMap<Position, Type>) {
    let last_x = map.keys().map(|p| p.x).max().unwrap();
    let first_x = map.keys().map(|p| p.x).min().unwrap();

    let last_y = map.keys().map(|p| p.y).max().unwrap();

    for y in 0..last_y + 1 {
        for x in first_x..last_x + 1 {
            let pos = Position { y: y, x: x };
            let s = match map.get(&pos).unwrap_or(&Type::Open) {
                Type::Open => '.',
                Type::Trees => '|',
                Type::Lumberyard => '#',
            };
            print!("{}", s);
        }
        print!("\n");
    }
}

fn tick_open(pos: Position, map: &HashMap<Position, Type>) -> Type {
    if pos
        .adjacent()
        .iter()
        .filter(|p| map.get(&p).unwrap_or(&Type::Open) == &Type::Trees)
        .collect::<Vec<&Position>>()
        .len()
        >= 3
    {
        Type::Trees
    } else {
        Type::Open
    }
}

fn tick_trees(pos: Position, map: &HashMap<Position, Type>) -> Type {
    if pos
        .adjacent()
        .iter()
        .filter(|p| map.get(&p).unwrap_or(&Type::Open) == &Type::Lumberyard)
        .collect::<Vec<&Position>>()
        .len()
        >= 3
    {
        Type::Lumberyard
    } else {
        Type::Trees
    }
}

fn tick_lumberyard(pos: Position, map: &HashMap<Position, Type>) -> Type {
    let any_adjacent_lumberyard = pos
        .adjacent()
        .iter()
        .any(|p| map.get(&p).unwrap_or(&Type::Open) == &Type::Lumberyard);
    let any_adjacent_trees = pos
        .adjacent()
        .iter()
        .any(|p| map.get(&p).unwrap_or(&Type::Open) == &Type::Trees);

    if any_adjacent_lumberyard && any_adjacent_trees {
        Type::Lumberyard
    } else {
        Type::Open
    }
}

fn solve2(map: &HashMap<Position, Type>) {
    let mut map = map.clone();

    let mut prev_hashes = HashMap::new();

    let mut iterations_left = 0;

    let n_iterations = 1000000000;

    for i in 0..n_iterations {
        map = map
            .iter()
            .map(|(&pos, pos_type)| match pos_type {
                Type::Lumberyard => (pos, tick_lumberyard(pos, &map)),
                Type::Open => (pos, tick_open(pos, &map)),
                Type::Trees => (pos, tick_trees(pos, &map)),
            })
            .collect();

        print_lumber_collection_area(&map);

        if i < 1000 {
            continue;
        }

        let hash = hash(&map);
        if prev_hashes.contains_key(&hash) {
            let periodicity = i - prev_hashes.get(&hash).unwrap();

            iterations_left = (n_iterations - i - 1) % periodicity;

            break;
        } else {
            prev_hashes.insert(hash, i);
        }
    }

    for _ in 0..iterations_left {
        map = map
            .iter()
            .map(|(&pos, pos_type)| match pos_type {
                Type::Lumberyard => (pos, tick_lumberyard(pos, &map)),
                Type::Open => (pos, tick_open(pos, &map)),
                Type::Trees => (pos, tick_trees(pos, &map)),
            })
            .collect();
    }

    let score = score(&map);
    println!("Part two: {}", score);
}

fn score(map: &HashMap<Position, Type>) -> i32 {
    let n_trees = map
        .iter()
        .map(|(_, t)| match t {
            Type::Trees => 1,
            _ => 0,
        })
        .sum::<i32>();

    let n_lumberyards = map
        .iter()
        .map(|(_, t)| match t {
            Type::Lumberyard => 1,
            _ => 0,
        })
        .sum::<i32>();

    n_lumberyards * n_trees
}

fn hash(map: &HashMap<Position, Type>) -> i64 {
    let n_trees = map
        .iter()
        .map(|(_, t)| match t {
            Type::Trees => 1,
            _ => 0,
        })
        .sum::<i64>();

    let n_lumberyards = map
        .iter()
        .map(|(_, t)| match t {
            Type::Lumberyard => 1,
            _ => 0,
        })
        .sum::<i64>();

    let n_open = map
        .iter()
        .map(|(_, t)| match t {
            Type::Open => 1,
            _ => 0,
        })
        .sum::<i64>();

    (n_lumberyards * 17) * (n_trees * 11) * (n_open * 13)
}
