use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Rectangle {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(PartialEq)]
enum Square {
    Free,
    Claimed,
    Overlaps,
}

pub fn solve() {
    let rectangles = parse();

    let answer1: i32 = solve1(rectangles.as_slice());
    println!("{:?}", answer1);

    let answer2 = solve2(rectangles.as_slice());
    println!("{:?}", answer2);
}

fn parse() -> Vec<Rectangle> {
    let filename = "input/day3input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // #758 @ 738,834: 21x13
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    contents
        .lines()
        .map(|l| {
            re.captures_iter(l)
                .map(|cap| Rectangle {
                    id: cap[1].parse::<i32>().unwrap(),
                    x: cap[2].parse::<i32>().unwrap(),
                    y: cap[3].parse::<i32>().unwrap(),
                    width: cap[4].parse::<i32>().unwrap(),
                    height: cap[5].parse::<i32>().unwrap(),
                })
                .next()
                .unwrap()
        })
        .collect()
}

fn solve1(rectangles: &[Rectangle]) -> i32 {
    let map = build_map(rectangles);

    let mut sum = 0;
    for (_, square) in &map {
        if square == &Square::Overlaps {
            sum += 1;
        }
    }

    sum
}

fn build_map(rectangles: &[Rectangle]) -> HashMap<(i32, i32), Square> {
    let mut map = HashMap::new();

    for rectangle in rectangles {
        insert_rectangle(&mut map, rectangle)
    }
    map
}

fn insert_rectangle(map: &mut HashMap<(i32, i32), Square>, rectangle: &Rectangle) {
    let rectangle_end_x = rectangle.x + rectangle.width;
    let rectangle_enx_y = rectangle.y + rectangle.height;
    for x in rectangle.x..rectangle_end_x {
        for y in rectangle.y..rectangle_enx_y {
            if map.get(&(x, y)).unwrap_or(&Square::Free) == &Square::Free {
                map.insert((x, y), Square::Claimed);
            } else {
                map.insert((x, y), Square::Overlaps);
            }
        }
    }
}

fn solve2(rectangles: &[Rectangle]) -> i32 {
    let map = build_map(rectangles);

    for rectangle in rectangles {
        if !rectangle_overlaps(&map, rectangle) {
            return rectangle.id;
        }
    }
    0
}

fn rectangle_overlaps(map: &HashMap<(i32, i32), Square>, rectangle: &Rectangle) -> bool {
    let rectangle_end_x = rectangle.x + rectangle.width;
    let rectangle_enx_y = rectangle.y + rectangle.height;
    for x in rectangle.x..rectangle_end_x {
        for y in rectangle.y..rectangle_enx_y {
            if map.get(&(x, y)).unwrap_or(&Square::Free) == &Square::Overlaps {
                return true;
            }
        }
    }
    return false;
}
