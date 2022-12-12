use crate::parser::parser;

type Input = Vec<Vec<char>>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}
type Path = Vec<Pos>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn parse() -> Input {
    parser::parse(12, "\n", |line| line.chars().collect())
}

fn new_path_if_climable(
    test_p: Pos,
    current_height: char,
    path: &Path,
    new_paths: &mut Vec<Path>,
    heightmap: &Input,
) {
    let mut test_height = heightmap[test_p.y][test_p.x];

    if test_height == 'E' {
        test_height = 'z';
    } else if test_height == 'S' {
        test_height = 'a';
    }
    //print!("{}, {} :", current_height, test_height);

    let test_height = test_height as u8;
    let current_height: u8 = current_height as u8;

    //println!("{}",current_height + 1 >= test_height);
    if current_height + 1 >= test_height {
        let mut new_path = path.clone();
        new_path.push(test_p);
        new_paths.push(new_path);
    }
}

fn walk_path(start_pos: Pos, heightmap: &Input) -> usize {
    let mut paths = vec![vec![start_pos]];

    let max_height = heightmap.len();
    let max_width = heightmap[0].len();

    let mut reachable_in = vec![vec![usize::MAX; max_width]; max_height];

    while paths.len() > 0 {
        let mut new_paths = vec![];
        for path in paths {
            let p = path.last().unwrap();

            if reachable_in[p.y][p.x] > path.len() {
                reachable_in[p.y][p.x] = path.len();
            } else {
                continue;
            }

            let mut current_height = heightmap[p.y][p.x];

            if current_height == 'E' {
                return path.len() - 1;
            } else if current_height == 'S' {
                current_height = 'a';
            }

            let up_pos = Pos {
                x: p.x,
                y: p.y.overflowing_sub(1).0,
            };
            let down_pos = Pos { x: p.x, y: p.y + 1 };
            let left_pos = Pos {
                x: p.x.overflowing_sub(1).0,
                y: p.y,
            };
            let right_pos = Pos { x: p.x + 1, y: p.y };

            if p.y > 0 && !path.contains(&up_pos) {
                new_path_if_climable(up_pos, current_height, &path, &mut new_paths, &heightmap);
            }
            if p.y < max_height - 1 && !path.contains(&down_pos) {
                new_path_if_climable(down_pos, current_height, &path, &mut new_paths, &heightmap);
            }
            if p.x > 0 && !path.contains(&left_pos) {
                new_path_if_climable(left_pos, current_height, &path, &mut new_paths, &heightmap);
            }
            if p.x < max_width - 1 && !path.contains(&right_pos) {
                new_path_if_climable(right_pos, current_height, &path, &mut new_paths, &heightmap);
            }
        }
        paths = new_paths;
    }

    usize::MAX
}

fn solve1(heightmap: Input) -> usize {
    walk_path(Pos { x: 0, y: 20 }, &heightmap)
}

fn solve2(heightmap: Input) -> usize {
    let mut all_as = vec![];

    for (y, vs) in heightmap.iter().enumerate() {
        for (x, v) in vs.iter().enumerate() {
            if *v == 'a' || *v == 'S' {
                all_as.push(Pos { x, y })
            }
        }
    }

    all_as
        .iter()
        .map(|p| walk_path(*p, &heightmap))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 391);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), 386);
    }
}
