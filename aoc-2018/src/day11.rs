pub fn solve() {
    let serial_number = 5719;

    println!("{:?}", cell_power_level(3, 5, 8));
    println!("{:?}", cell_power_level(122, 79, 57));
    println!("{:?}", cell_power_level(217, 196, 39));
    println!("{:?}", cell_power_level(101, 153, 71));
    solve1(serial_number);
    solve2(serial_number);
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

    let mut max_val: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut max_pool: i32 = 0;

    for pool_size in 1..300 {
        println!("{:?}", pool_size);
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
                    max_pool = pool_size;
                }
            }
        }
    }

    println!("{:?},{:?},{:?}", max_x, max_y, max_pool);
}
