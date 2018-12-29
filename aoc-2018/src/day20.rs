use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

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

    fn possible_adjacent_rooms(self) -> Vec<(Position, Position)> {
        vec![
            (self.leftof(), self.leftof().leftof()),
            (self.rightof(), self.rightof().rightof()),
            (self.above(), self.above().above()),
            (self.under(), self.under().under()),
        ]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
enum MapType {
    Wall,
    Room,
    VerticalDoor,
    HorizontalDoor,
    Unknown,
}

pub fn solve() {
    let input = parse();
    solve1(&input);
    solve2(&input);
}

fn parse() -> String {
    let filename = "input/day20input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .to_string()
}

fn solve1(input: &String) -> i32 {
    let map = build_map(input);

    let room_distance = count_room_distance(&map);
    let furthest_room = *room_distance.values().max().unwrap_or(&-1);

    println!("Part one: {}", furthest_room);

    furthest_room
}

fn build_map(input: &String) -> HashMap<Position, MapType> {
    let mut map = HashMap::new();
    let mut current_position = Position { y: 0, x: 0 };

    let mut position_stack = VecDeque::new();

    for c in input.chars() {
        map.insert(current_position, MapType::Room);
        if c == 'W' {
            let door_at = current_position.leftof();
            map.insert(door_at, MapType::VerticalDoor);
            map.insert(door_at.above(), MapType::Wall);
            map.insert(door_at.under(), MapType::Wall);

            current_position = door_at.leftof();
        } else if c == 'E' {
            let door_at = current_position.rightof();
            map.insert(door_at, MapType::VerticalDoor);
            map.insert(door_at.above(), MapType::Wall);
            map.insert(door_at.under(), MapType::Wall);

            current_position = door_at.rightof();
        } else if c == 'N' {
            let door_at = current_position.above();
            map.insert(door_at, MapType::HorizontalDoor);
            map.insert(door_at.leftof(), MapType::Wall);
            map.insert(door_at.rightof(), MapType::Wall);

            current_position = door_at.above();
        } else if c == 'S' {
            let door_at = current_position.under();
            map.insert(door_at, MapType::HorizontalDoor);
            map.insert(door_at.leftof(), MapType::Wall);
            map.insert(door_at.rightof(), MapType::Wall);

            current_position = door_at.under();
        } else if c == '(' {
            position_stack.push_back(current_position.clone());
        } else if c == ')' {
            position_stack.pop_back();
        } else if c == '|' {
            current_position = position_stack.back().unwrap().clone();
        }
    }
    map
}

fn count_room_distance(map: &HashMap<Position, MapType>) -> HashMap<Position, i32> {
    let mut room_distance = HashMap::new();

    let mut rooms = VecDeque::new();
    rooms.push_back((Position { y: 0, x: 0 }, 0));

    while !rooms.is_empty() {
        let (room_pos, room_score) = rooms.pop_front().unwrap();

        for (maybe_door, maybe_room) in room_pos.possible_adjacent_rooms().iter() {
            let maybe_door_type = map.get(maybe_door).unwrap_or(&MapType::Unknown);
            if maybe_door_type == &MapType::HorizontalDoor
                || maybe_door_type == &MapType::VerticalDoor
            {
                if !room_distance.contains_key(maybe_room) {
                    room_distance.insert(maybe_room.clone(), room_score + 1);

                    rooms.push_back((maybe_room.clone(), room_score + 1));
                }
            }
        }
    }

    room_distance
}

fn solve2(input: &String) {
    let map = build_map(input);

    let room_distance = count_room_distance(&map);
    let n_rooms_above_1000_away = room_distance
        .values()
        .map(|&d| if d >= 1000 { 1 } else { 0 })
        .sum::<i32>();

    println!("Part two: {}", n_rooms_above_1000_away);
}

fn print_map(map: &HashMap<Position, MapType>) {
    let last_x = map.keys().map(|p| p.x).max().unwrap();
    let first_x = map.keys().map(|p| p.x).min().unwrap();

    let first_y = map.keys().map(|p| p.y).min().unwrap();
    let last_y = map.keys().map(|p| p.y).max().unwrap();

    for y in first_y..last_y + 1 {
        for x in first_x..last_x + 1 {
            let pos = Position { y: y, x: x };
            let s = match map.get(&pos).unwrap_or(&MapType::Unknown) {
                MapType::Room => '.',
                MapType::VerticalDoor => '|',
                MapType::HorizontalDoor => '-',
                MapType::Wall => '#',
                MapType::Unknown => '?',
            };
            print!("{}", s);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_example() {
        let s = String::from("^WNE$");
        let furthest_room = solve1(&s);

        assert_eq!(furthest_room, 3);
    }
    #[test]
    fn second_example() {
        let s = String::from("^ENWWW(NEEE|SSE(EE|N))$");
        let furthest_room = solve1(&s);

        assert_eq!(furthest_room, 10);
    }
    #[test]
    fn third_example() {
        let s = String::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        let furthest_room = solve1(&s);

        assert_eq!(furthest_room, 18);
    }
}
