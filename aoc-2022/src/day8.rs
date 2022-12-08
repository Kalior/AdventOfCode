use crate::parser::parser;

#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

type Trees = Vec<Vec<usize>>;

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
    lines
}

fn is_visible_from_left(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos.y][pos.x];
    (0..pos.x).all(|x| trees[pos.y][x] < height)
}
fn is_visible_from_right(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos.y][pos.x];
    (pos.x + 1..trees[pos.y].len()).all(|x| trees[pos.y][x] < height)
}
fn is_visible_from_up(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos.y][pos.x];
    (0..pos.y).all(|y| trees[y][pos.x] < height)
}
fn is_visible_from_down(pos: &Pos, trees: &Trees) -> bool {
    let height = trees[pos.y][pos.x];
    (pos.y + 1..trees.len()).all(|y| trees[y][pos.x] < height)
}

fn is_visible(pos: &Pos, trees: &Trees) -> bool {
    let max_x = trees[pos.y].len();
    let max_y = trees.len();

    if pos.x == 0 || pos.y == 0 || pos.x == max_x || pos.y == max_y {
        return true;
    }

    return is_visible_from_left(pos, trees)
        || is_visible_from_right(pos, trees)
        || is_visible_from_up(pos, trees)
        || is_visible_from_down(pos, trees);
}

fn scenic_score(pos: &Pos, trees: &Trees) -> usize {
    let height = trees[pos.y][pos.x];

    let max_x = trees[pos.y].len();
    let max_y = trees.len();

    let mut left_scenic = (0..pos.x)
        .rev()
        .take_while(|x| trees[pos.y][*x] < height)
        .count()
        + 1;
    let mut right_scenic = (pos.x + 1..max_x)
        .take_while(|x| trees[pos.y][*x] < height)
        .count()
        + 1;
    let mut up_scenic = (0..pos.y)
        .rev()
        .take_while(|y| trees[*y][pos.x] < height)
        .count()
        + 1;
    let mut down_scenic = (pos.y + 1..max_y)
        .take_while(|y| trees[*y][pos.x] < height)
        .count()
        + 1;

    if is_visible_from_left(pos, trees) {
        left_scenic -= 1
    }
    if is_visible_from_right(pos, trees) {
        right_scenic -= 1
    }
    if is_visible_from_up(pos, trees) {
        up_scenic -= 1
    }
    if is_visible_from_down(pos, trees) {
        down_scenic -= 1
    }

    return (left_scenic) * (right_scenic) * (up_scenic) * (down_scenic);
}

fn solve1(trees: &Trees) -> usize {
    trees
        .iter()
        .enumerate()
        .map(|(y, vals)| {
            vals.iter()
                .enumerate()
                .filter(|(x, _)| is_visible(&Pos { x: *x, y: y }, trees))
                .count()
        })
        .sum()
}

fn solve2(trees: &Trees) -> usize {
    trees
        .iter()
        .enumerate()
        .map(|(y, vals)| {
            vals.iter()
                .enumerate()
                .map(|(x, _)| scenic_score(&Pos { x: x, y: y }, trees))
                .max()
                .unwrap()
        })
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
