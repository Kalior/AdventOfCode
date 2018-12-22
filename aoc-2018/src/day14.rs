use std::cmp::max;

pub fn solve() {
    let input = 939601;
    solve1(input);
    solve2(input);
}

fn solve1(iterations: i32) {
    let mut recipie_board = Vec::new();
    recipie_board.push(3);
    recipie_board.push(7);

    let mut elves = Vec::new();
    elves.push(0);
    elves.push(1);

    let mut n_recipie = 2;

    while n_recipie < iterations + 10 {
        let score = elves
            .iter()
            .map(|&i| recipie_board.get(i).unwrap())
            .sum::<i32>();

        if score < 10 {
            recipie_board.push(score);
            n_recipie += 1;
        } else {
            let score_two = score - 10;
            let score_one = score / 10;

            recipie_board.push(score_one);
            recipie_board.push(score_two);

            n_recipie += 2;
        }

        elves = elves
            .iter()
            .map(|&i| (i + (recipie_board.get(i).unwrap() + 1) as usize) % recipie_board.len())
            .collect();
    }

    let last_10 = (iterations..iterations + 10)
        .map(|i| recipie_board.get(i as usize).unwrap())
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("Part one: {:?}", last_10);
}

fn solve2(iterations: i32) {
    let mut recipie_board = Vec::new();
    recipie_board.push(3);
    recipie_board.push(7);

    let mut elves = Vec::new();
    elves.push(0);
    elves.push(1);

    let iterations_string = iterations.to_string();

    loop {
        let score = elves
            .iter()
            .map(|&i| recipie_board.get(i).unwrap())
            .sum::<i32>();

        if score < 10 {
            recipie_board.push(score);
        } else {
            let score_one = score / 10;
            let score_two = score - 10;

            recipie_board.push(score_one);

            let (start_index, last_6) = last_6_string(&recipie_board);

            if last_6 == iterations_string {
                println!("Part two: {:?}", start_index);
                return;
            }

            recipie_board.push(score_two);
        }

        let (start_index, last_6) = last_6_string(&recipie_board);

        if last_6 == iterations_string {
            println!("Part two: {:?}", start_index);
            return;
        }

        elves = elves
            .iter()
            .map(|&i| (i + (recipie_board.get(i).unwrap() + 1) as usize) % recipie_board.len())
            .collect();
    }
}

fn last_6_string(recipie_board: &Vec<i32>) -> (usize, String) {
    let start_index: usize = max((recipie_board.len() as i32 - 6) as usize, 0);

    let last_6 = (start_index..recipie_board.len())
        .map(|i| recipie_board.get(i).unwrap())
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join("");

    return (start_index, last_6);
}
