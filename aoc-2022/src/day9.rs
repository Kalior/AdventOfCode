use crate::parser::parser;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn char_to_dir(c: &str) -> Dir {
    match c {
        "R" => Dir::Right,
        "L" => Dir::Left,
        "U" => Dir::Up,
        "D" => Dir::Down,
        _ => panic!(),
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

type Input = Vec<(Dir, usize)>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}
fn parse_line(line: &str) -> (Dir, usize) {
    let splits: Vec<&str> = line.split(" ").collect();

    (char_to_dir(splits[0]), splits[1].parse::<usize>().unwrap())
}

fn parse() -> Input {
    parser::parse(9, "\n", parse_line)
}

fn are_adjacent(head_pos: &Pos, tail_pos: &Pos) -> bool {
    let euclidean_distance = f32::sqrt(
        f32::powf(head_pos.x as f32 - tail_pos.x as f32, 2.0)
            + f32::powf(head_pos.y as f32 - tail_pos.y as f32, 2.0),
    );

    euclidean_distance < 2.0
}

fn follow_head(head_pos: &Pos, tail_pos: &mut Pos) {
    if !are_adjacent(head_pos, tail_pos) {
        if head_pos.x == tail_pos.x {
            let diff = (tail_pos.y - head_pos.y).signum();
            tail_pos.y -= diff
        } else if head_pos.y == tail_pos.y {
            let diff = (tail_pos.x - head_pos.x).signum();
            tail_pos.x -= diff
        } else {
            let xdiff = (tail_pos.x - head_pos.x).signum();
            let ydiff = (tail_pos.y - head_pos.y).signum();
            tail_pos.x -= xdiff;
            tail_pos.y -= ydiff;
        }
    }
}

fn move_rope(knots_pos: &mut Vec<Pos>, action: (Dir, usize), visited: &mut HashSet<Pos>) {
    let n_moves = action.1;

    for _ in 0..n_moves {
        let mut head_pos = knots_pos.first_mut().unwrap();
        match action.0 {
            Dir::Up => head_pos.y -= 1,
            Dir::Down => head_pos.y += 1,
            Dir::Left => head_pos.x -= 1,
            Dir::Right => head_pos.x += 1,
        }

        for i in 1..knots_pos.len() {
            let (left, right) = knots_pos.split_at_mut(i);
            follow_head(left.last().unwrap(), right.first_mut().unwrap());
        }

        visited.insert(*knots_pos.last().unwrap());
    }
}

fn solve1(input: Input) -> usize {
    let mut visited = HashSet::new();
    let mut knot_pos: Vec<Pos> = vec![Pos { x: 0, y: 0 }; 2];

    for action in input {
        move_rope(&mut knot_pos, action, &mut visited);
    }

    visited.len()
}

fn solve2(input: Input) -> usize {
    let mut visited = HashSet::new();
    let mut knot_pos: Vec<Pos> = vec![Pos { x: 0, y: 0 }; 10];

    for action in input {
        move_rope(&mut knot_pos, action, &mut visited);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent() {
        assert_eq!(are_adjacent(&Pos { x: 0, y: 0 }, &Pos { x: 0, y: 0 }), true);
        assert_eq!(are_adjacent(&Pos { x: 0, y: 0 }, &Pos { x: 1, y: 1 }), true);
        assert_eq!(are_adjacent(&Pos { x: 0, y: 0 }, &Pos { x: 0, y: 1 }), true);
        assert_eq!(
            are_adjacent(&Pos { x: 0, y: 0 }, &Pos { x: 2, y: 1 }),
            false
        );
        assert_eq!(
            are_adjacent(&Pos { x: -1, y: 0 }, &Pos { x: 2, y: 1 }),
            false
        );
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 5695);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), 2434);
    }
}
