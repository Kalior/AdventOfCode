use crate::parser::parser;

type Input = Vec<i32>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn parse() -> Input {
    parser::parse(0, "\n", |line| line.parse::<i32>().unwrap())
}

fn solve1(_input: Input) -> i32 {
    println!("Part one: {}", -1);
    -1
}

fn solve2(_input: Input) -> i32 {
    println!("Part two: {}", -1);
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), -1);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), -1);
    }
}
