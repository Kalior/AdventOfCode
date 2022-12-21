use std::cmp::max;
use std::collections::HashSet;
use std::iter;

use indicatif::ProgressBar;

use crate::parser::parser;

#[derive(Copy, Clone)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

impl Dir {
    fn parse(c: char) -> Dir {
        match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

type Input = Vec<Dir>;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Shape {
    positions: Vec<Pos>,
}

impl Shape {
    fn translate(&mut self, dir: &Dir, n: i64) -> &mut Shape {
        match dir {
            Dir::Left => self.positions.iter_mut().for_each(|p| p.x -= n),
            Dir::Right => self.positions.iter_mut().for_each(|p| p.x += n),
            Dir::Down => self.positions.iter_mut().for_each(|p| p.y -= n),
            Dir::Up => self.positions.iter_mut().for_each(|p| p.y += n),
        }
        self
    }
    fn collides(&self, still_rocks: &HashSet<Pos>) -> bool {
        self.positions.iter().any(|p| still_rocks.contains(p))
    }

    fn out_of_bounds(&self) -> bool {
        self.positions.iter().any(|p| p.x < 0 || p.x >= 7)
    }

    fn horizontal_line() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
            ],
        }
    }
    fn floor() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
                Pos { x: 4, y: 0 },
                Pos { x: 5, y: 0 },
                Pos { x: 6, y: 0 },
            ],
        }
    }
    fn plus() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 1 },
                Pos { x: 2, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 2 },
            ],
        }
    }
    fn l() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 2, y: 2 },
            ],
        }
    }
    fn vertical_line() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 0, y: 3 },
            ],
        }
    }
    fn block() -> Shape {
        Shape {
            positions: vec![
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 },
            ],
        }
    }
}

pub fn solve() {
    let input = parse();
    let one = solve1(&input);
    println!("Part one: {}", one);
    let two = solve2(&input);
    println!("Part two: {}", two);
}

fn parse() -> Input {
    let lines: Vec<Vec<Dir>> = parser::parse(17, "\n", |line| {
        line.chars().map(|v| Dir::parse(v)).collect()
    });

    lines[0].clone()
}

fn solve1(jet_streams: &Input) -> usize {
    simulate_rock_falling(jet_streams, 2022).0
}

fn print_still_rocks(still_rocks: &HashSet<Pos>, height: i64, rocks: &Vec<Pos>) {
    for y in (0..height + 3).rev() {
        for x in 0..7 {
            let p = Pos { x, y };
            if still_rocks.contains(&p) {
                print!("#");
            } else if rocks.contains(&p) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn find_period(heights_after: &Vec<usize>) -> (usize, usize) {
    for start_i in 1..heights_after.len() / 2 {
        for period in 5..start_i {
            let height_diff = heights_after[start_i + period] - heights_after[start_i];

            let periods = 1..(heights_after.len() - start_i) / period;

            if periods.into_iter().all(|x| {
                height_diff
                    == heights_after[start_i + period * (x + 1)]
                        - heights_after[start_i + period * x]
            }) {
                return (start_i, period);
            }
        }
    }
    (0, 0)
}

fn solve2(jet_streams: &Input) -> usize {
    let (_, heights_after) = simulate_rock_falling(jet_streams, 20000);

    let (start, period) = find_period(&heights_after);

    if start == 0 {
        return 0;
    }
    let n_total_iterations = 1000000000000usize;

    let height_diff = heights_after[start + period] - heights_after[start];
    let n_even_periods = (n_total_iterations as f64 / period as f64).floor() as usize - 1;

    let n_iterations_left = n_total_iterations - (start + n_even_periods * period);

    let (v, _) = simulate_rock_falling(jet_streams, (start + period + n_iterations_left) as i64);
    let remaining_height_diff = v - heights_after[start + period];

    let period_height = height_diff * n_even_periods;

    let total_height = heights_after[start] + period_height + remaining_height_diff;
    total_height
}

fn simulate_rock_falling(jet_streams: &Input, n_iterations: i64) -> (usize, Vec<usize>) {
    let rocks = vec![
        Shape::horizontal_line(),
        Shape::plus(),
        Shape::l(),
        Shape::vertical_line(),
        Shape::block(),
    ];

    let mut rocks_i = 0;

    let mut infinite_rocks = iter::repeat_with(|| {
        let rock = rocks[rocks_i].clone();
        rocks_i = (rocks_i + 1) % rocks.len();
        rock
    });

    let mut stream_i = 0;

    let mut infinite_jet_stream = iter::repeat_with(|| {
        let stream_dir = jet_streams[stream_i];
        stream_i = (stream_i + 1) % jet_streams.len();
        stream_dir
    });

    let mut still_rocks = HashSet::new();
    let mut height = 0;
    let mut heights = vec![];

    let mut n_stopped = 0;

    insert_shape(&mut still_rocks, Shape::floor().translate(&Dir::Down, 1));

    let bar = ProgressBar::new(n_iterations as u64);

    loop {
        let mut rock = infinite_rocks.next().unwrap();

        rock.translate(&Dir::Up, (height + 3) as i64);
        rock.translate(&Dir::Right, 2);

        while !rock.collides(&still_rocks) {
            let stream_dir = infinite_jet_stream.next().unwrap();

            rock.translate(&stream_dir, 1);
            if rock.collides(&still_rocks) || rock.out_of_bounds() {
                rock.translate(&stream_dir, -1);
            }

            rock.translate(&Dir::Down, 1);
        }

        // Now the rock has collided (down) and therefore needs to move up
        rock.translate(&Dir::Up, 1);

        height = max(height, insert_shape(&mut still_rocks, &mut rock));
        heights.push(height);

        n_stopped += 1;
        bar.inc(1);

        if n_stopped == n_iterations {
            return (height, heights);
        }
    }
}

fn insert_shape(still_rocks: &mut HashSet<Pos>, rock: &Shape) -> usize {
    let mut height = 0;
    for p in rock.positions.iter() {
        still_rocks.insert(p.clone());
        height = max(height, (p.y + 1) as usize);
    }
    height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(&input), 3109);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(&input), 1541449275365);
    }
}
