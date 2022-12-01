use crate::parser::parser;


pub fn solve() {
    let input = parser::parse(1, "\n\n", parse_group);
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}


fn solve1(input: Vec<Vec<i32>>) -> i32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn solve2(input: Vec<Vec<i32>>) -> i32 {
    let mut elf_snacks: Vec<i32> = input.iter().map(|elf| elf.iter().sum()).collect();
    elf_snacks.sort();
    elf_snacks.iter().rev().take(3).sum()
}

fn parse_group(group: &str) -> Vec<i32> {
    group
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parser::parse(1, "\n\n", parse_group);
        assert_eq!(solve1(input), 71924);
    }

    #[test]
    fn part_two_test() {
        let input = parser::parse(1, "\n\n", parse_group);
        assert_eq!(solve2(input), 210406);
    }
}
