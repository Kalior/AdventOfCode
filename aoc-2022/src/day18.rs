use crate::parser::parser;
use std::collections::HashSet;

type Input = Vec<Pos>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    let two = solve2(&input);
    println!("Part two: {}", two);
}

impl Pos {
    fn parse(line: &str) -> Pos {
        let parts: Vec<&str> = line.split(",").collect();

        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        let z = parts[2].parse::<i32>().unwrap();

        Pos { x, y, z }
    }

    fn is_adjacent(&self, other: &Pos) -> bool {
        let x_adjacent = self.y == other.y && self.z == other.z && (self.x - other.x).abs() == 1;
        let y_adjacent = self.x == other.x && self.z == other.z && (self.y - other.y).abs() == 1;
        let z_adjacent = self.y == other.y && self.x == other.x && (self.z - other.z).abs() == 1;

        x_adjacent || y_adjacent || z_adjacent
    }
}

fn parse() -> Input {
    parser::parse(18, "\n", Pos::parse)
}

fn parse_test() -> Input {
    parser::parse_str("18test", "\n", Pos::parse)
}

fn scan_x(scan: &Input) -> i32 {
    scan_dim(scan, |p| p.x, |p| p.y, |p| p.z)
}

fn scan_y(scan: &Input) -> i32 {
    scan_dim(scan, |p| p.y, |p| p.x, |p| p.z)
}

fn scan_z(scan: &Input) -> i32 {
    scan_dim(scan, |p| p.z, |p| p.y, |p| p.x)
}

fn scan_dim(
    scan: &Input,
    scan_dim: fn(&Pos) -> i32,
    dim1: fn(&Pos) -> i32,
    dim2: fn(&Pos) -> i32,
) -> i32 {
    let min_dim_1 = scan.iter().map(dim1).min().unwrap();
    let max_dim_1 = scan.iter().map(dim1).max().unwrap();

    let min_dim_2 = scan.iter().map(dim2).min().unwrap();
    let max_dim_2 = scan.iter().map(dim2).max().unwrap();

    let mut exposed_sides = 0;

    for dim_2 in min_dim_2..max_dim_2 + 1 {
        let pos_at_dim_2: Vec<&Pos> = scan.iter().filter(|p| dim2(p) == dim_2).collect();
        for dim_1 in min_dim_1..max_dim_1 + 1 {
            let mut pos_at_dim_1: Vec<&Pos> = pos_at_dim_2
                .clone()
                .into_iter()
                .filter(|p| dim1(p) == dim_1)
                .collect();
            pos_at_dim_1.sort_by_key(|p| scan_dim(p));

            if pos_at_dim_1.len() != 0 {
                exposed_sides += 2;
            }

            for i in 1..pos_at_dim_1.len() {
                if scan_dim(&pos_at_dim_1[i - 1]) + 1 != scan_dim(&pos_at_dim_1[i]) {
                    exposed_sides += 2;
                }
            }
        }
    }

    exposed_sides
}

fn solve1(scan: &Input) -> i32 {
    scan_x(scan) + scan_y(scan) + scan_z(scan)
}

fn solve2(scan: &Input) -> i32 {
    let mut explored = HashSet::new();
    let min_x = scan.iter().map(|p| p.x).min().unwrap();
    let min_y = scan.iter().map(|p| p.y).min().unwrap();
    let min_z = scan.iter().map(|p| p.z).min().unwrap();

    let max_x = scan.iter().map(|p| p.x).max().unwrap();
    let max_y = scan.iter().map(|p| p.y).max().unwrap();
    let max_z = scan.iter().map(|p| p.z).max().unwrap();

    let mut surface_area = 0;
    let mut to_explore = vec![Pos {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    }];
    while !to_explore.is_empty() {
        let pos = to_explore.pop().unwrap();

        if explored.contains(&pos)
            || scan.contains(&pos)
            || pos.x > max_x + 1
            || pos.x < min_x - 1
            || pos.y > max_y + 1
            || pos.y < min_y - 1
            || pos.z > max_z + 1
            || pos.z < min_z - 1
        {
            continue;
        }

        surface_area += scan.iter().filter(|p| pos.is_adjacent(p)).count();

        explored.insert(pos);

        to_explore.push(Pos {
            x: pos.x + 1,
            y: pos.y,
            z: pos.z,
        });
        to_explore.push(Pos {
            x: pos.x - 1,
            y: pos.y,
            z: pos.z,
        });
        to_explore.push(Pos {
            x: pos.x,
            y: pos.y + 1,
            z: pos.z,
        });
        to_explore.push(Pos {
            x: pos.x,
            y: pos.y - 1,
            z: pos.z,
        });
        to_explore.push(Pos {
            x: pos.x,
            y: pos.y,
            z: pos.z + 1,
        });
        to_explore.push(Pos {
            x: pos.x,
            y: pos.y,
            z: pos.z - 1,
        });
    }
    surface_area as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent() {
        assert!(Pos { x: 1, y: 2, z: 2 }.is_adjacent(&Pos { x: 2, y: 2, z: 2 }));

        assert!(!Pos { x: 0, y: 2, z: 2 }.is_adjacent(&Pos { x: 2, y: 2, z: 2 }));

        assert!(!Pos { x: 1, y: 1, z: 2 }.is_adjacent(&Pos { x: 2, y: 2, z: 2 }));
    }

    #[test]
    fn part_one_test_input() {
        let input = parse_test();
        assert_eq!(solve1(&input), 64);
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 4390);
    }

    #[test]
    fn part_two_test_input() {
        let input = parse_test();
        assert_eq!(solve2(&input), 58);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(&input), 2534);
    }
}
