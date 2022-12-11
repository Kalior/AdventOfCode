use crate::parser::parser;
use std::ops::Rem;

#[derive(Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: fn(u128) -> u128,
    test: fn(u128) -> bool,
    iftrue: usize,
    iffalse: usize,
}

impl Monkey {
    fn parse(raw_monkey: &str) -> Monkey {
        let lines: Vec<&str> = raw_monkey.lines().collect();

        let item_line = lines[0];
        let items = item_line.split(" : ").last().unwrap();
        let _items: Vec<i32> = items
            .split(", ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        lines[1].split(" = ").last().unwrap();
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            operation: |v| v * 5,
            test: |v| (v % 7) == 0,
            iftrue: 6,
            iffalse: 7,
        }
    }
}

type Input = Vec<Monkey>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn test_monkeys() -> Input {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |v| v * 19,
            test: |v| (v % 23) == 0,
            iftrue: 2,
            iffalse: 3,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |v| v + 6,
            test: |v| (v % 19) == 0,
            iftrue: 2,
            iffalse: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |v| v * v,
            test: |v| (v % 13) == 0,
            iftrue: 1,
            iffalse: 3,
        },
        Monkey {
            items: vec![74],
            operation: |v| v + 3,
            test: |v| (v % 17) == 0,
            iftrue: 0,
            iffalse: 1,
        },
    ]
}

fn parse() -> Input {
    vec![
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            operation: |v| v * 5,
            test: |v| (v % 7) == 0,
            iftrue: 6,
            iffalse: 7,
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85],
            operation: |v| v + 1,
            test: |v| (v % 17) == 0,
            iftrue: 0,
            iffalse: 6,
        },
        Monkey {
            items: vec![83, 89, 53],
            operation: |v| v + 8,
            test: |v| (v % 11) == 0,
            iftrue: 5,
            iffalse: 3,
        },
        Monkey {
            items: vec![95, 94, 85, 57],
            operation: |v| v + 4,
            test: |v| (v % 13) == 0,
            iftrue: 0,
            iffalse: 1,
        },
        Monkey {
            items: vec![82, 98],
            operation: |v| v + 7,
            test: |v| (v % 19) == 0,
            iftrue: 5,
            iffalse: 2,
        },
        Monkey {
            items: vec![69],
            operation: |v| v + 2,
            test: |v| (v % 2) == 0,
            iftrue: 1,
            iffalse: 3,
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65],
            operation: |v| v * 11,
            test: |v| (v % 5) == 0,
            iftrue: 7,
            iffalse: 4,
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82],
            operation: |v| v * v,
            test: |v| (v % 3) == 0,
            iftrue: 4,
            iffalse: 2,
        },
    ]
}

fn solve1(monkeys: Input) -> u128 {
    let mut monkeys = monkeys.to_vec();
    let mut inpsected_per_monkey: Vec<u128> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in &monkey.items {
                inpsected_per_monkey[i] += 1;

                let mut new_worry = (monkey.operation)(*item);
                new_worry = (new_worry as f64 / 3.0).floor() as u128;

                if (monkey.test)(new_worry) {
                    monkeys[monkey.iftrue].items.push(new_worry);
                } else {
                    monkeys[monkey.iffalse].items.push(new_worry);
                }
            }
            monkeys[i].items = vec![];
        }
    }

    inpsected_per_monkey.sort();
    inpsected_per_monkey.iter().rev().take(2).product()
}

fn solve2(monkeys: Input) -> u128 {
    let mut monkeys = monkeys.to_vec();
    let mut inpsected_per_monkey: Vec<u128> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in &monkey.items {
                inpsected_per_monkey[i] += 1;

                let mut new_worry = (monkey.operation)(*item);

                new_worry = new_worry.rem(2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 as u128);

                if (monkey.test)(new_worry) {
                    monkeys[monkey.iftrue].items.push(new_worry);
                } else {
                    monkeys[monkey.iffalse].items.push(new_worry);
                }
            }
            monkeys[i].items = vec![];
        }
    }

    for v in inpsected_per_monkey.to_vec() {
        println!("{}", v);
    }

    inpsected_per_monkey.sort();
    inpsected_per_monkey.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = test_monkeys();
        assert_eq!(solve1(input), 10605);

        let input = parse();
        assert_eq!(solve1(input), 55930);
    }

    #[test]
    fn part_two_test() {
        let input = test_monkeys();
        assert_eq!(solve2(input), 2713310158);

        let input = parse();
        assert_eq!(solve2(input), 14636993466);
    }
}
