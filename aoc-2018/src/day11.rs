extern crate rayon;
use rayon::prelude::*;

use std::time::Instant;

pub fn solve() {
    let serial_number = 5719;

    solve1(serial_number);

    let now = Instant::now();

    solve2(serial_number);
    println!("{}ms", now.elapsed().as_millis());
}

fn solve1(serial_number: i32) {
    let mut power_grid: Vec<Vec<i32>> = vec![vec![0; 300]; 300];
    for x in 1..300 {
        for y in 1..300 {
            power_grid[x][y] = cell_power_level(x as i32, y as i32, serial_number);
        }
    }

    let mut max_val: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    let pool_size: i32 = 3;

    for x in 1..300 - pool_size {
        for y in 1..300 - pool_size {
            let pool_val = (x..x + pool_size)
                .map(|xp| {
                    (y..y + pool_size)
                        .map(|yp| power_grid[xp as usize][yp as usize])
                        .sum::<i32>()
                })
                .sum::<i32>();

            if pool_val > max_val {
                max_val = pool_val;
                max_x = x;
                max_y = y;
            }
        }
    }

    println!("{:?},{:?}", max_x, max_y);
}

fn cell_power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;

    let power_level: Vec<String> = power_level
        .to_string()
        .chars()
        .map(|c| c.to_string())
        .rev()
        .collect();

    if power_level.len() < 3 {
        0
    } else {
        power_level[2].parse::<i32>().unwrap_or(-1) - 5
    }
}

fn solve2(serial_number: i32) {
    let mut power_grid: Vec<Vec<i32>> = vec![vec![0; 300]; 300];
    for x in 1..300 {
        for y in 1..300 {
            power_grid[x][y] = cell_power_level(x as i32, y as i32, serial_number);
        }
    }

    let mut space = vec![vec![(0, 0); 300]; 300];

    for x in 0..300 {
        space[x][0] = (0, 0);
    }

    for y in 0..300 {
        space[0][y] = (0, 0);
    }

    let space: Vec<Vec<(i32, usize)>> = (0..300usize)
        .into_par_iter()
        .map(|x| {
            (0..300)
                .map(|y| square_val(&power_grid, x, y, 300))
                .collect()
        })
        .collect();

    let mut max_val = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_pool = 0;
    for x in 0..300 {
        for y in 0..300 {
            if space[x][y].0 > max_val {
                max_val = space[x][y].0;
                max_x = x;
                max_y = y;
                max_pool = space[x][y].1;
            }
        }
    }

    println!("{:?},{:?},{:?}", max_x, max_y, max_pool);
}

fn square_val(power_grid: &Vec<Vec<i32>>, x: usize, y: usize, max_size: usize) -> (i32, usize) {
    let mut size = 0;

    let mut best_size = 0;
    let mut best_val: i32 = 0;
    let mut new_val: i32 = 0;
    while (best_val - new_val).abs() < 30 {
        size += 1;
        if x + size > max_size || y + size > max_size {
            return (best_val, best_size);
        }

        new_val = (x..x + size)
            .map(|xp| {
                (y..y + size)
                    .map(|yp| power_grid[xp as usize][yp as usize])
                    .sum::<i32>()
            })
            .sum::<i32>();

        if new_val > best_val {
            best_val = new_val;
            best_size = size;
        }
    }

    (best_val, best_size)
}
