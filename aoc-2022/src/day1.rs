use crate::parser::parser;


pub fn solve() {
    let input = parser::group_parse(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parser::group_parse(1);
        assert_eq!(solve1(input), 71924);
    }

    #[test]
    fn part_two_test() {
        let input = parser::group_parse(1);
        assert_eq!(solve2(input), 210406);
    }
}
