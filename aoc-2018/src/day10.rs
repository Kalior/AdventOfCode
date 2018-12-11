use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn move_point(point: &Point) -> Point {
    Point {
        x: point.x + point.dx,
        y: point.y + point.dy,
        dx: point.dx,
        dy: point.dy,
    }
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

pub fn solve() {
    let points = parse();

    solve1(points.to_vec());
}

fn parse() -> Vec<Point> {
    let filename = "input/day10input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // position=<-21751,  11136> velocity=< 2, -1>
    let re =
        Regex::new(r"position=<\s?(-?\d+),\s*(-?\d+)>\s+velocity=<\s?(-?\d+),\s*(-?\d+)>").unwrap();

    re.captures_iter(&contents)
        .map(|cap| Point {
            x: cap[1].parse::<i32>().unwrap_or(-1),
            y: cap[2].parse::<i32>().unwrap_or(-1),
            dx: cap[3].parse::<i32>().unwrap_or(-1),
            dy: cap[4].parse::<i32>().unwrap_or(-1),
        })
        .collect()
}

fn solve1(points: Vec<Point>) {
    let mut points = points.clone();
    let mut i = 0;
    while !points_aligned(&points) {
        i += 1;
        points = points.iter().map(|p| move_point(p)).collect();
    }

    println!("Part one:");
    draw_map(points);
    println!("Part two:");
    println!("{:?}", i);
}

fn draw_map(points: Vec<Point>) {
    let left_corner_y = points.iter().map(|p| p.y).min().unwrap();
    let left_corner_x = points.iter().map(|p| p.x).min().unwrap();
    let right_corner_y = points.iter().map(|p| p.y).max().unwrap();
    let right_corner_x = points.iter().map(|p| p.x).max().unwrap();

    let width = right_corner_x - left_corner_x;
    let height = right_corner_y - left_corner_y;

    let mut map = vec![vec!['.'; (width + 1) as usize]; (height + 1) as usize];

    for p in points {
        let idx = (p.x - left_corner_x) as usize;
        let idy = (p.y - left_corner_y) as usize;
        map[idy][idx] = '#';
    }

    for line in map {
        println!("{:?}", line.iter().map(|c| c.to_string()).join(""))
    }
}

fn points_aligned(points: &Vec<Point>) -> bool {
    points
        .iter()
        .all(|p1| points.iter().any(|p2| p1 != p2 && distance(&p1, &p2) < 3))
}
