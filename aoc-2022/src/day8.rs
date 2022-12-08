use std::collections::HashMap;

use crate::parser::parser;

#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

type Trees = HashMap<Pos, usize>;

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    let two = solve2(&input);
    println!("Part two: {}", two);
}

fn parse() -> Trees {
    let lines = parser::parse(8, "\n", |line| {
        line.chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    });

    let mut hashmap = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            hashmap.insert(Pos { x, y }, *height);
        }
    }

    hashmap
}

fn is_visible_from_left(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos];
    (0..pos.x).all(|x| trees[&Pos { x, y: pos.y }] < height)
}
fn is_visible_from_right(pos: &Pos, trees: &Trees, max_x: usize) -> bool {
    let height = trees[pos];
    (pos.x + 1..=max_x).all(|x| trees[&Pos { x, y: pos.y }] < height)
}
fn is_visible_from_up(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos];
    (0..pos.y).all(|y| trees[&Pos { x: pos.x, y }] < height)
}
fn is_visible_from_down(pos: &Pos, trees: &Trees, max_y: usize) -> bool {
    let height = trees[pos];
    (pos.y + 1..=max_y).all(|y| trees[&Pos { x: pos.x, y }] < height)
}

fn is_visible(pos: &Pos, trees: &Trees) -> bool {
    let max_x = trees.keys().map(|p| p.x).max().unwrap();
    let max_y = trees.keys().map(|p| p.y).max().unwrap();

    if pos.x == 0 || pos.y == 0 || pos.x == max_x || pos.y == max_y {
        return true;
    }

    return is_visible_from_left(pos, trees)
        || is_visible_from_right(pos, trees, max_x)
        || is_visible_from_up(pos, trees)
        || is_visible_from_down(pos, trees, max_y);
}

fn scenic_score(pos: &Pos, trees: &Trees) -> usize {
    let height = trees[pos];

    let max_x = trees.keys().map(|p| p.x).max().unwrap();
    let max_y = trees.keys().map(|p| p.y).max().unwrap();

    let mut left_scenic = (0..pos.x)
        .rev()
        .take_while(|x| trees[&Pos { x: *x, y: pos.y }] < height)
        .count()
        + 1;
    let mut right_scenic = (pos.x + 1..=max_x)
        .take_while(|x| trees[&Pos { x: *x, y: pos.y }] < height)
        .count()
        + 1;
    let mut up_scenic = (0..pos.y)
        .rev()
        .take_while(|y| trees[&Pos { x: pos.x, y: *y }] < height)
        .count()
        + 1;
    let mut down_scenic = (pos.y + 1..=max_y)
        .take_while(|y| trees[&Pos { x: pos.x, y: *y }] < height)
        .count()
        + 1;

    if is_visible_from_left(pos, trees) {
        left_scenic -= 1
    }
    if is_visible_from_right(pos, trees, max_x) {
        right_scenic -= 1
    }
    if is_visible_from_up(pos, trees) {
        up_scenic -= 1
    }
    if is_visible_from_down(pos, trees, max_y) {
        down_scenic -= 1
    }

    return (left_scenic) * (right_scenic) * (up_scenic) * (down_scenic);
}

fn solve1(trees: &Trees) -> usize {
    trees.keys().filter(|pos| is_visible(pos, trees)).count()
}

fn solve2(trees: &Trees) -> usize {
    trees
        .keys()
        .map(|pos| scenic_score(pos, trees))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 1801);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        //        assert_eq!(scenic_score(&Pos { x: 2, y: 3 }, &input), 8);
        assert_eq!(solve2(&input), 209880);
    }
}
