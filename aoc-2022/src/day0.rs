use crate::parser::parser;


pub fn solve() {
    let input = parser::parse(0);
    solve1(input.to_vec());
    solve2(input.to_vec());
}


fn solve1(input: Vec<i32>) -> i32 {
    println!("Part one: {}", -1);
    -1
}

fn solve2(input: Vec<i32>) -> i32 {
    println!("Part two: {}", -1);
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parser::parse(0);
        assert_eq!(solve1(input), -1);
    }

    #[test]
    fn part_two_test() {
        let input = parser::parse(0);
        assert_eq!(solve2(input), -1);
    }
}