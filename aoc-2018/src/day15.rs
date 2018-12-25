use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
struct Position {
    y: i32,
    x: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
enum Type {
    Space,
    Wall,
    Elf { hp: i32, id: i32 },
    Goblin { hp: i32, id: i32 },
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve() {
    let input = parse();
    solve1(input.clone());
    solve2(input.clone());
}

fn parse() -> HashMap<Position, Type> {
    let filename = "input/day15input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut map = HashMap::new();

    let mut id = 0;

    for (y, l) in contents.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = Position {
                y: y as i32,
                x: x as i32,
            };

            if c == '#' {
                map.insert(pos, Type::Wall);
            } else if c == '.' {
                map.insert(pos, Type::Space);
            } else if c == 'G' {
                map.insert(pos, Type::Goblin { hp: 200, id: id });
                id += 1;
            } else if c == 'E' {
                map.insert(pos, Type::Elf { hp: 200, id: id });
                id += 1;
            }
        }
    }

    return map;
}

fn solve1(map: HashMap<Position, Type>) {
    let mut map = map.clone();

    let mut combat_ongoing = true;
    let mut ticks = 0;

    let mut removed_creatures = HashSet::new();
    while combat_ongoing {
        let creatures: Vec<(Position, Type)> = get_creatures(&map);

        if is_combat_over(&map) || creatures.len() == 0 {
            break;
        }

        for (pos, creature) in creatures {
            let creature_id = match creature {
                Type::Elf { hp: _, id } => id,
                Type::Goblin { hp: _, id } => id,
                _ => -1,
            };

            if removed_creatures.contains(&creature_id) {
                continue;
            }

            if is_combat_over(&map) {
                combat_ongoing = false;
                break;
            }

            let mut position = pos.clone();

            if !is_adjacent_to_enemy(&map, pos, &creature) {
                position = move_creature(&mut map, position);
            }

            let removed_id = attack(&mut map, position, 3);

            if removed_id.is_some() {
                removed_creatures.insert(removed_id.unwrap());
            }
        }

        if combat_ongoing {
            ticks += 1;
        }
    }
    let score = ticks
        * get_creatures(&map)
            .iter()
            .map(|(p, _)| match map.get(&p).unwrap() {
                Type::Elf { hp, id: _ } => hp,
                Type::Goblin { hp, id: _ } => hp,
                _ => &0,
            })
            .sum::<i32>();

    println!("Part one: {:?}", score);
}

fn is_combat_over(map: &HashMap<Position, Type>) -> bool {
    let creatures = get_creatures(map);

    creatures.iter().all(|(_, p_type)| match p_type {
        Type::Elf { hp: _, id: _ } => true,
        _ => false,
    }) || creatures.iter().all(|(_, p_type)| match p_type {
        Type::Goblin { hp: _, id: _ } => true,
        _ => false,
    })
}

fn get_creatures(map: &HashMap<Position, Type>) -> Vec<(Position, Type)> {
    let mut creatures: Vec<(Position, Type)> = map
        .iter()
        .filter(|(_, pos_type)| match pos_type {
            Type::Elf { hp: _, id: _ } => true,
            Type::Goblin { hp: _, id: _ } => true,
            _ => false,
        })
        .map(|(pos, pos_type)| (pos.clone(), pos_type.clone()))
        .collect();

    creatures.sort_by_key(|(pos, _)| pos.clone());
    creatures
}

fn attack(map: &mut HashMap<Position, Type>, from: Position, elf_power: i32) -> Option<i32> {
    let from_type = map.get(&from).unwrap();

    let mut targets: Vec<Position> = [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(|(dy, dx)| Position {
            y: from.y + dy,
            x: from.x + dx,
        })
        .filter(|p| p.y > 0 && p.x > 0)
        .filter(|p| match (map.get(&p).unwrap(), from_type) {
            (Type::Goblin { hp: _, id: _ }, Type::Elf { hp: _, id: _ }) => true,
            (Type::Elf { hp: _, id: _ }, Type::Goblin { hp: _, id: _ }) => true,
            _ => false,
        })
        .collect();

    if targets.len() > 0 {
        targets.sort_by_key(|p| match map.get(&p).unwrap() {
            Type::Goblin { hp, id: _ } => hp,
            Type::Elf { hp, id: _ } => hp,
            _ => &0,
        });

        let target = targets.first().unwrap();

        let new_type = match map.get(&target).unwrap() {
            Type::Goblin { hp, id } => Type::Goblin {
                hp: hp - elf_power,
                id: *id,
            },
            Type::Elf { hp, id } => Type::Elf {
                hp: hp - 3,
                id: *id,
            },
            _ => Type::Goblin { hp: 0, id: -1 },
        };

        let (is_dead, id) = match new_type {
            Type::Goblin { hp, id } => (hp <= 0, id),
            Type::Elf { hp, id } => (hp <= 0, id),
            _ => (false, -1),
        };

        if is_dead {
            map.insert(target.clone(), Type::Space);
            return Some(id);
        } else {
            map.insert(target.clone(), new_type);
            return None;
        }
    }

    return None;
}

fn move_creature(map: &mut HashMap<Position, Type>, from: Position) -> Position {
    let from_type = map.get(&from).unwrap();

    let mut potential_positions = VecDeque::new();
    for (dy, dx, dir) in [
        (-1, 0, Direction::Up),
        (0, -1, Direction::Left),
        (0, 1, Direction::Right),
        (1, 0, Direction::Down),
    ]
    .iter()
    {
        let pos = Position {
            y: from.y + dy,
            x: from.x + dx,
        };

        let is_space = match map.get(&pos) {
            Some(Type::Space) => true,
            _ => false,
        };
        if from.y + dy > 0 && from.x + dx > 0 && is_space {
            potential_positions.push_back((pos, dir, 0));
        }
    }

    let mut checked_positions = HashSet::new();
    let mut reachable_enemies = Vec::new();

    while !potential_positions.is_empty() {
        let (pos, dir, steps) = potential_positions.pop_front().unwrap();
        if checked_positions.contains(&pos) {
            continue;
        } else {
            checked_positions.insert(pos.clone());
        }

        if is_adjacent_to_enemy(map, pos, from_type) {
            reachable_enemies.push((steps, pos, dir));
        } else {
            let positions: Vec<Position> = [(-1, 0), (0, -1), (0, 1), (1, 0)]
                .iter()
                .map(|(dy, dx)| Position {
                    y: pos.y + dy,
                    x: pos.x + dx,
                })
                .filter(|p| p.y > 0 && p.x > 0)
                .filter(|p| match map.get(&p) {
                    Some(Type::Space) => true,
                    _ => false,
                })
                .collect();
            for p in positions {
                potential_positions.push_back((p, dir, steps + 1));
            }
        }
    }

    if reachable_enemies.len() > 0 {
        reachable_enemies.sort_by_key(|(steps, pos, _)| (steps.clone(), pos.clone()));
        let (_, _, &direction) = reachable_enemies.first().unwrap();

        let new_pos = if direction == Direction::Up {
            Position {
                y: from.y - 1,
                x: from.x,
            }
        } else if direction == Direction::Down {
            Position {
                y: from.y + 1,
                x: from.x,
            }
        } else if direction == Direction::Left {
            Position {
                y: from.y,
                x: from.x - 1,
            }
        } else if direction == Direction::Right {
            Position {
                y: from.y,
                x: from.x + 1,
            }
        } else {
            from.clone()
        };

        map.insert(new_pos, from_type.clone());
        map.insert(from.clone(), Type::Space);

        return new_pos.clone();
    }

    return from.clone();
}

fn is_adjacent_to_enemy(map: &HashMap<Position, Type>, from: Position, from_type: &Type) -> bool {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(|(dy, dx)| Position {
            y: from.y + dy,
            x: from.x + dx,
        })
        .any(|pos| {
            pos != from
                && match (map.get(&pos), from_type) {
                    (Some(Type::Goblin { hp: _, id: _ }), Type::Elf { hp: _, id: _ }) => true,
                    (Some(Type::Elf { hp: _, id: _ }), Type::Goblin { hp: _, id: _ }) => true,
                    _ => false,
                }
        })
}

fn print_map(map: &HashMap<Position, Type>) {
    let last_x = map.keys().map(|p| p.x).max().unwrap();
    let last_y = map.keys().map(|p| p.y).max().unwrap();
    for y in 0..last_y + 1 {
        for x in 0..last_x + 1 {
            let pos = Position { y: y, x: x };
            let s = match map.get(&pos).unwrap() {
                Type::Goblin { hp: _, id: _ } => 'G',
                Type::Elf { hp: _, id: _ } => 'E',
                Type::Space => '.',
                Type::Wall => '#',
            };
            print!("{}", s);
        }
        print!("\n");
    }
}

fn solve2(og_map: HashMap<Position, Type>) {
    let mut elf_power = 3;
    let mut elf_killed = true;

    let elf_ids: Vec<i32> = get_creatures(&og_map)
        .iter()
        .filter(|(_, creature)| match creature {
            Type::Elf { hp: _, id: _ } => true,
            _ => false,
        })
        .map(|(_, creature)| match creature {
            Type::Elf { hp: _, id } => *id,
            _ => 0,
        })
        .collect();

    while elf_killed {
        elf_killed = false;
        elf_power += 1;

        let mut map = og_map.clone();
        let mut removed_creatures = HashSet::new();
        let mut combat_ongoing = true;
        let mut ticks = 0;

        while combat_ongoing {
            let creatures: Vec<(Position, Type)> = get_creatures(&map);

            if is_combat_over(&map) || creatures.len() == 0 {
                break;
            }

            for (pos, creature) in creatures {
                let creature_id = match creature {
                    Type::Elf { hp: _, id } => id,
                    Type::Goblin { hp: _, id } => id,
                    _ => -1,
                };

                if removed_creatures.contains(&creature_id) {
                    continue;
                }

                if is_combat_over(&map) {
                    combat_ongoing = false;
                    break;
                }

                let mut position = pos.clone();

                if !is_adjacent_to_enemy(&map, pos, &creature) {
                    position = move_creature(&mut map, position);
                }

                let removed_id = attack(&mut map, position, elf_power);

                if removed_id.is_some() {
                    let removed_id = removed_id.unwrap();
                    removed_creatures.insert(removed_id);

                    if elf_ids.contains(&removed_id) {
                        elf_killed = true;
                        combat_ongoing = false;
                        break;
                    }
                }
            }

            if combat_ongoing {
                ticks += 1;
            }
        }
        if !elf_killed {
            let score = ticks
                * get_creatures(&map)
                    .iter()
                    .map(|(p, _)| match map.get(&p).unwrap() {
                        Type::Elf { hp, id: _ } => hp,
                        Type::Goblin { hp, id: _ } => hp,
                        _ => &0,
                    })
                    .sum::<i32>();
            println!("Part two: {:?}", score);
            return;
        }
    }
}
