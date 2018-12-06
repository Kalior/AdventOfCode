use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
    id: i32,
}

pub fn solve() {
    let coordinates = parse();
    // println!("{:?}", coordinates);

    let answer1: i32 = solve1(coordinates.as_slice());
    println!("{:?}", answer1);
    let answer2 = solve2(coordinates.as_slice());
    println!("{:?}", answer2);
}

fn parse() -> Vec<Coordinate> {
    let filename = "input/day6input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let ints: Vec<&str> = l.split(", ").collect();
            Coordinate {
                x: ints[0].parse::<i32>().unwrap(),
                y: ints[1].parse::<i32>().unwrap(),
                id: i as i32,
            }
        })
        .collect()
}

fn solve1(coordinates: &[Coordinate]) -> i32 {
    let left_corner_y = coordinates.iter().map(|c| c.y).min().unwrap();
    let left_corner_x = coordinates.iter().map(|c| c.x).min().unwrap();
    let right_corner_y = coordinates.iter().map(|c| c.y).max().unwrap();
    let right_corner_x = coordinates.iter().map(|c| c.x).max().unwrap();

    let mut map = HashMap::new();

    for x in left_corner_x..right_corner_x {
        for y in left_corner_y..right_corner_y {
            let null_coordinate = Coordinate {
                x: -1,
                y: -1,
                id: -1,
            };
            let (closest, _) =
                coordinates
                    .iter()
                    .fold((&null_coordinate, 99999999), |(closest, dist), c| {
                        let new_dist = distance(c, x, y);
                        if new_dist < dist {
                            (c, new_dist)
                        } else if new_dist == dist {
                            (&null_coordinate, dist)
                        } else {
                            (closest, dist)
                        }
                    });

            map.insert((x, y), closest.id);
        }
    }

    let mut infinite_areas = HashSet::new();
    for x in left_corner_x..right_corner_x {
        add_infinites(&mut infinite_areas, coordinates, x, left_corner_y);
        add_infinites(&mut infinite_areas, coordinates, x, right_corner_y);
    }
    for y in left_corner_y..right_corner_y {
        add_infinites(&mut infinite_areas, coordinates, left_corner_x, y);
        add_infinites(&mut infinite_areas, coordinates, right_corner_x, y);
    }

    let mut counts = HashMap::new();
    for val in map.values() {
        if infinite_areas.contains(val) {
            continue;
        }

        let counter = counts.entry(val).or_insert(0);
        *counter += 1;
    }

    *counts.values().max().unwrap_or(&0)
}

fn distance(coordinate: &Coordinate, x: i32, y: i32) -> i32 {
    (coordinate.x - x).abs() + (coordinate.y - y).abs()
}

fn add_infinites(infinite_areas: &mut HashSet<i32>, coordinates: &[Coordinate], x: i32, y: i32) {
    let min_dist = coordinates
        .iter()
        .map(|c| distance(&c, x, y))
        .min()
        .unwrap();

    let infinites = coordinates
        .iter()
        .filter(|c| distance(&c, x, y) == min_dist);

    for infinite in infinites {
        infinite_areas.insert(infinite.id);
    }
}

fn solve2(coordinates: &[Coordinate]) -> i32 {
    let left_corner_y = coordinates.iter().map(|c| c.y).min().unwrap();
    let left_corner_x = coordinates.iter().map(|c| c.x).min().unwrap();
    let right_corner_y = coordinates.iter().map(|c| c.y).max().unwrap();
    let right_corner_x = coordinates.iter().map(|c| c.x).max().unwrap();

    let mut map = HashMap::new();

    let cutoff = 10000;
    for x in left_corner_x..right_corner_x {
        for y in left_corner_y..right_corner_y {
            let summed_distance: i32 = coordinates.iter().map(|c| distance(&c, x, y)).sum();
            if summed_distance < cutoff {
                map.insert((x, y), true);
            } else {
                map.insert((x, y), false);
            }
        }
    }
    let mut counts = HashMap::new();
    for val in map.values() {
        let counter = counts.entry(val).or_insert(0);
        *counter += 1;
    }

    *counts.get(&true).unwrap()
}
