use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::Range;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::parser::parser;

type Input = Vec<(Pos, Pos)>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn solve() {
    let input = parse();
    //let one = solve1(input.to_vec());
    //println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn to_sensor_reading(line: &str) -> (Pos, Pos) {
    let splits: Vec<&str> = line.split(" ").collect();

    let sensor_x = splits[2]
        .replace("x=", "")
        .replace(",", "")
        .parse::<i64>()
        .unwrap();
    let sensor_y = splits[3]
        .replace("y=", "")
        .replace(":", "")
        .parse::<i64>()
        .unwrap();

    let beacon_x = splits[8]
        .replace("x=", "")
        .replace(",", "")
        .parse::<i64>()
        .unwrap();
    let beacon_y = splits[9].replace("y=", "").parse::<i64>().unwrap();

    (
        Pos {
            x: sensor_x,
            y: sensor_y,
        },
        Pos {
            x: beacon_x,
            y: beacon_y,
        },
    )
}

fn parse() -> Input {
    parser::parse(15, "\n", to_sensor_reading)
}

enum Contains {
    NoBeacon,
    Beacon,
    Sensor,
}

fn solve1(sensor_readings: Input) -> i32 {
    let mut no_beacon_at_2000000: HashSet<Pos> = HashSet::new();
    let mut beacon_at_2000000: HashSet<Pos> = HashSet::new();

    let y_of_interest = 2000000;

    for (sensor, beacon) in sensor_readings {
        if beacon.y == y_of_interest {
            beacon_at_2000000.insert(beacon);
        }

        let manhattan_d = sensor.manhattan_distance(&beacon);

        for y in (sensor.y - manhattan_d)..(sensor.y + manhattan_d + 1) {
            for x in (sensor.x - manhattan_d)..(sensor.x + manhattan_d + 1) {
                let p = Pos { x, y };

                if sensor.manhattan_distance(&p) <= manhattan_d {
                    if p.y == y_of_interest {
                        no_beacon_at_2000000.insert(p);
                    }
                }
            }
        }
    }

    (&no_beacon_at_2000000 - &beacon_at_2000000).len() as i32
}

fn print_tunnels(
    tunnels: &mut HashMap<Pos, Contains>,
    min_y: i64,
    min_x: i64,
    max_y: i64,
    max_x: i64,
) {
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let c = match tunnels.get(&Pos { x, y }) {
                None => " ",
                Some(Contains::NoBeacon) => "#",
                Some(Contains::Beacon) => "B",
                Some(Contains::Sensor) => "S",
            };
            print!("{}", c);
        }
        println!()
    }
    println!()
}

fn is_not_covered(pos: &Pos, sensor_readings: &Input) -> bool {
    sensor_readings.iter().all(|&(sensor, beacon)| {
        sensor.manhattan_distance(&beacon) < sensor.manhattan_distance(pos)
    })
}

fn get_sensors_range(sensor: &Pos, beacon: &Pos, y: i64) -> Option<Range<i64>> {
    let plusminus = sensor.manhattan_distance(&beacon) - (sensor.y - y).abs();
    let plus_x = sensor.x + plusminus;
    let minus_x = sensor.x - plusminus;

    if plusminus < 0 {
        return None;
    }

    Some(min(minus_x, plus_x)..max(minus_x, plus_x) + 1)
}

fn get_sensor_ranges(sensor_readings: &Input, y: i64) -> Vec<Range<i64>> {
    sensor_readings
        .iter()
        .map(|(s, b)| get_sensors_range(s, b, y))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect()
}

fn solve2(sensor_readings: Input) -> i64 {
    let step_size: i64 = 1;
    let y_ranges: Vec<_> = (0..4000000)
        .step_by(step_size as usize)
        .collect::<Vec<i64>>();

    *y_ranges
        .par_iter()
        .progress_count(y_ranges.len() as u64)
        .map(|&y| check_y_value(&sensor_readings, y))
        .filter(|&v| v != -1)
        .collect::<Vec<i64>>()
        .get(0)
        .unwrap()
}

fn check_y_value(sensor_readings: &Input, y: i64) -> i64 {
    let mut ranges = get_sensor_ranges(&sensor_readings, y);

    ranges.sort_by_key(|r| r.start);

    for x in 0..ranges[0].start {
        let p = Pos { x, y };
        if is_not_covered(&p, &sensor_readings) {
            println!("{}, {}", x, y);
            return x * 4000000 + y;
        }
    }
    for i in 1..ranges.len() {
        for x in ranges[i - 1].end..ranges[i].start {
            let p = Pos { x, y };

            if is_not_covered(&p, &sensor_readings) {
                println!("{}, {}", x, y);
                return x * 4000000 + y;
            }
        }
    }
    for x in ranges[ranges.len() - 1].end..4000000 + 1 {
        let p = Pos { x, y };
        if is_not_covered(&p, &sensor_readings) {
            println!("{}, {}", x, y);
            return x * 4000000 + y;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test() -> Input {
        parser::parse_str("15test", "\n", to_sensor_reading)
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(
            Pos { x: 0, y: 1 }.manhattan_distance(&Pos { x: 3, y: 6 }),
            8
        );

        assert_eq!(
            Pos {
                x: 3275729,
                y: 2937931
            }
            .manhattan_distance(&Pos {
                x: 3454717,
                y: 2547103
            }),
            569816
        );
        assert_eq!(
            Pos {
                x: 545413,
                y: 533006
            }
            .manhattan_distance(&Pos {
                x: -538654,
                y: 69689
            }),
            1547384
        );
    }

    #[test]
    fn test_is_covered() {
        let input = parse_test();
        assert_eq!(is_not_covered(&Pos { x: 14, y: 11 }, &input), true);
    }

    #[test]
    fn test_get_range() {
        assert_eq!(
            get_sensors_range(&Pos { x: 8, y: 7 }, &Pos { x: 2, y: 10 }, 7),
            Some(-1..18)
        );

        assert_eq!(
            get_sensors_range(&Pos { x: 8, y: 7 }, &Pos { x: 2, y: 10 }, -1),
            Some(7..10)
        );

        assert_eq!(
            get_sensors_range(&Pos { x: 14, y: 3 }, &Pos { x: 15, y: 3 }, 11),
            None
        );
    }

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 5403290);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), -1);
    }
}
