use crate::parser::parser;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::{thread, time};

type Input = HashMap<Pos, Fill>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Fill {
    Air,
    Sand,
    Rock,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    //let two = solve2(&input);
    //println!("Part two: {}", two);
}

fn to_pos(pair: &str) -> Pos {
    let splits = pair.split(",").collect::<Vec<&str>>();

    Pos {
        x: splits[0].parse::<usize>().unwrap(),
        y: splits[1].parse::<usize>().unwrap(),
    }
}

fn add_line_to_map(cave: &mut HashMap<Pos, Fill>, line: &str) {
    let positions: Vec<Pos> = line.split(" -> ").map(to_pos).collect();

    for (from_pos, to_pos) in positions.iter().zip(positions[1..].iter()) {
        if from_pos.x == to_pos.x {
            for y in min(from_pos.y, to_pos.y)..max(from_pos.y, to_pos.y) + 1 {
                cave.insert(Pos { x: from_pos.x, y }, Fill::Rock);
            }
        } else {
            for x in min(from_pos.x, to_pos.x)..max(from_pos.x, to_pos.x) + 1 {
                cave.insert(Pos { x, y: from_pos.y }, Fill::Rock);
            }
        }
    }
}

fn parse() -> Input {
    let mut cave = HashMap::new();
    let lines = parser::parse(14, "\n", |line| line.to_string());

    for line in lines {
        add_line_to_map(&mut cave, &line);
    }

    cave
}

enum Dir {
    Down,
    DownLeft,
    DownRight,
}

fn can_move(cave: &Input, pos: &Pos) -> Option<Dir> {
    return if cave
        .get(&Pos {
            x: pos.x,
            y: pos.y + 1,
        })
        .unwrap_or(&Fill::Air)
        == &Fill::Air
    {
        Some(Dir::Down)
    } else if cave
        .get(&Pos {
            x: pos.x - 1,
            y: pos.y + 1,
        })
        .unwrap_or(&Fill::Air)
        == &Fill::Air
    {
        Some(Dir::DownLeft)
    } else if cave
        .get(&Pos {
            x: pos.x + 1,
            y: pos.y + 1,
        })
        .unwrap_or(&Fill::Air)
        == &Fill::Air
    {
        Some(Dir::DownRight)
    } else {
        None
    };
}

fn add_sand(cave: &mut Input, max_y: usize) -> Pos {
    let mut sand_position = Pos { x: 500, y: 0 };

    let mut move_dir = Some(Dir::Down);
    while move_dir.is_some() {
        move_dir = can_move(cave, &sand_position);
        match move_dir {
            None => {}
            Some(Dir::Down) => {
                sand_position = Pos {
                    x: sand_position.x,
                    y: sand_position.y + 1,
                };
            }
            Some(Dir::DownLeft) => {
                sand_position = Pos {
                    x: sand_position.x - 1,
                    y: sand_position.y + 1,
                }
            }
            Some(Dir::DownRight) => {
                sand_position = Pos {
                    x: sand_position.x + 1,
                    y: sand_position.y + 1,
                }
            }
        }
        if sand_position.y >= max_y {
            move_dir = None;
        }
    }

    //println!("{}, {}", sand_position.x, sand_position.y);
    cave.insert(sand_position, Fill::Sand);
    sand_position
}

fn solve1(cave: &Input) -> i32 {
    let mut cave = cave.clone();

    let min_x = cave.keys().map(|p| p.x).min().unwrap();

    let max_y = cave.keys().map(|p| p.y).max().unwrap();
    let max_x = cave.keys().map(|p| p.x).max().unwrap();

    let mut sand_fell_into_abyss = false;
    let mut n_sands = 0;
    while !sand_fell_into_abyss {
        let sand_pos = add_sand(&mut cave, max_y);
        if sand_pos.y >= max_y {
            sand_fell_into_abyss = true;
        }
        n_sands += 1;
        //print_cave(&mut cave, min_y, min_x, max_y, max_x);
        print!("\x1B[2J\x1B[1;1H");
        print_cave(&mut cave, 0, min_x - 5, max_y + 3, max_x + 5);
        thread::sleep(time::Duration::from_millis(10));
    }
    print_cave(&mut cave, 0, min_x - 5, max_y + 3, max_x + 5);

    n_sands - 1
}

fn print_cave(
    cave: &mut HashMap<Pos, Fill>,
    min_y: usize,
    min_x: usize,
    max_y: usize,
    max_x: usize,
) {
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let c = match cave.entry(Pos { x, y }).or_insert(Fill::Air) {
                Fill::Air => " ",
                Fill::Sand => "+",
                Fill::Rock => "#",
            };
            print!("{}", c);
        }
        println!()
    }
    println!()
}

fn solve2(cave: &Input) -> i32 {
    let mut cave = cave.clone();

    let min_x = cave.keys().map(|p| p.x).min().unwrap();

    let max_y = cave.keys().map(|p| p.y).max().unwrap();
    let max_x = cave.keys().map(|p| p.x).max().unwrap();

    for x in 0..max_x + 500 {
        cave.insert(Pos { x, y: max_y + 2 }, Fill::Rock);
    }

    let mut sand_covers_source = false;
    let mut n_sands = 0;
    while !sand_covers_source {
        let sand_pos = add_sand(&mut cave, max_y + 2);
        if sand_pos.y == 0 && sand_pos.x == 500 {
            sand_covers_source = true;
        }
        n_sands += 1;
    }
    print_cave(&mut cave, 0, min_x - 100, max_y + 3, max_x + 100);

    n_sands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 838);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(&input), 27539);
    }
}
