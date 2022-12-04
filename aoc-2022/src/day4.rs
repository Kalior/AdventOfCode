use std::ops::Range;
use crate::parser::parser;


pub fn solve() {
    solve1(parse());
    solve2(parse());
}

fn part_to_range(part: &str) -> Range<i32> {
    let range_parts: Vec<&str> = part.split("-").collect();
    let start = range_parts[0].parse::<i32>().unwrap();
    let end = range_parts[1].parse::<i32>().unwrap();

    Range { start, end: end + 1 }
}

fn parse_line(line: &str) -> (Range<i32>, Range<i32>) {
    let parts: Vec<&str> = line.split(",").collect();

    (part_to_range(parts[0]), part_to_range(parts[1]))
}

fn parse() -> Vec<(Range<i32>, Range<i32>)> {
    parser::parse(4, "\n", parse_line)
}

fn range_fully_contains(query: &Range<i32>, container: &Range<i32>) -> bool {
    Range { start: query.start, end: query.end }.into_iter().all(|v| container.contains(&v))
}

fn range_overlaps(query: &Range<i32>, container: &Range<i32>) -> bool {
    Range { start: query.start, end: query.end }.into_iter().any(|v| container.contains(&v))
}


fn solve1(input: Vec<(Range<i32>, Range<i32>)>) -> usize {
    input.iter().filter(|ranges| range_fully_contains(&ranges.0, &ranges.1) || range_fully_contains(&ranges.1, &ranges.0)).count()
}

fn solve2(input: Vec<(Range<i32>, Range<i32>)>) -> usize {
    input.iter().filter(|ranges| range_overlaps(&ranges.0, &ranges.1) || range_overlaps(&ranges.1, &ranges.0)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 459);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), 779);
    }
}
