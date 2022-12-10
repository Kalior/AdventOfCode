use crate::parser::parser;

type Input = Vec<String>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
}
fn parse_x(line: &String) -> (String, i32) {
    let splits: Vec<&str> = line.split(" ").collect();

    (splits[0].to_string(), splits[1].parse::<i32>().unwrap())
}

fn parse() -> Input {
    parser::parse(10, "\n", |line| line.to_string())
}

fn draw(cycle: i32, x: i32) {
    let crt_pos = (cycle - 1) % 40;

    if (x - crt_pos).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if crt_pos == 39 {
        println!();
    }
}

fn run_program(instructions: Input) -> i32 {
    let mut next_checkpoint: i32 = 20;
    let mut checkpoint_sum = 0;

    let mut x = 1;
    let mut cycle: i32 = 1;
    for instruction in instructions {
        if instruction == "noop" {
            draw(cycle, x);
            cycle += 1;

            if cycle == next_checkpoint {
                checkpoint_sum += next_checkpoint * x;
                next_checkpoint += 40;
            }
        } else {
            let (_, v) = parse_x(&instruction);

            draw(cycle, x);
            cycle += 1;

            if cycle == next_checkpoint {
                checkpoint_sum += next_checkpoint * x;
                next_checkpoint += 40;
            }

            draw(cycle, x);
            cycle += 1;
            x += v;

            if cycle == next_checkpoint {
                checkpoint_sum += next_checkpoint * x;
                next_checkpoint += 40;
            }
        }
    }

    checkpoint_sum
}
fn solve1(input: Input) -> i32 {
    run_program(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 15020);
    }
}
