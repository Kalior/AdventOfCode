use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
struct Instruction {
    opcode: i32,
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Sample {
    before: Vec<i32>,
    instruction: Instruction,
    after: Vec<i32>,
}

pub fn solve() {
    let (samples, instructions) = parse();

    println!("{:?}", instructions[0]);

    solve1(samples.clone());
    solve2(samples.clone(), instructions);
}

fn parse() -> (Vec<Sample>, Vec<Instruction>) {
    let filename = "input/day16input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // Before: [2, 3, 2, 2]
    // 0 3 3 0
    // After:  [0, 3, 2, 2]

    let sample_re = Regex::new(r"Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let instruction_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

    let samples: Vec<Sample> = sample_re
        .captures_iter(&contents)
        .map(|cap| Sample {
            before: (1..5).map(|i| cap[i].parse::<i32>().unwrap()).collect(),
            instruction: Instruction {
                opcode: cap[5].parse::<i32>().unwrap(),
                a: cap[6].parse::<i32>().unwrap(),
                b: cap[7].parse::<i32>().unwrap(),
                c: cap[8].parse::<i32>().unwrap(),
            },
            after: (9..13).map(|i| cap[i].parse::<i32>().unwrap()).collect(),
        })
        .collect();

    let instructions: Vec<Instruction> = instruction_re
        .captures_iter(&contents)
        .map(|cap| Instruction {
            opcode: cap[1].parse::<i32>().unwrap(),
            a: cap[2].parse::<i32>().unwrap(),
            b: cap[3].parse::<i32>().unwrap(),
            c: cap[4].parse::<i32>().unwrap(),
        })
        .collect();

    let n_samples: usize = samples.len();

    (samples, instructions[n_samples..].to_vec())
}

fn addr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] + registers[b as usize];
}

fn addi(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] + b;
}

fn mulr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] * registers[b as usize];
}

fn muli(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] * b;
}

fn banr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] & registers[b as usize];
}

fn bani(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] & b;
}

fn borr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] | registers[b as usize];
}

fn bori(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] | b;
}

fn setr(registers: &mut Vec<i32>, a: i32, _b: i32, c: i32) {
    registers[c as usize] = registers[a as usize];
}

fn seti(registers: &mut Vec<i32>, a: i32, _b: i32, c: i32) {
    registers[c as usize] = a;
}

fn gtir(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if a > registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn gtri(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if registers[a as usize] > b {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn gtrr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if registers[a as usize] > registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqir(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if a == registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqri(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if registers[a as usize] == b {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqrr(registers: &mut Vec<i32>, a: i32, b: i32, c: i32) {
    if registers[a as usize] == registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn solve1(samples: Vec<Sample>) {
    let mut n_behaves_like_3_or_more = 0;

    let operations: Vec<fn(&mut Vec<i32>, i32, i32, i32)> = vec![
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    for sample in samples {
        let mut n_behaves_like = 0;
        for op in &operations {
            let mut registers = sample.before.clone();
            op(
                &mut registers,
                sample.instruction.a,
                sample.instruction.b,
                sample.instruction.c,
            );

            if registers
                .iter()
                .zip(sample.after.clone())
                .all(|(&a, b)| a == b)
            {
                n_behaves_like += 1;
            }
        }

        if n_behaves_like > 2 {
            n_behaves_like_3_or_more += 1;
        }
    }
    println!("Part 1: {}", n_behaves_like_3_or_more);
}

fn solve2(samples: Vec<Sample>, instructions: Vec<Instruction>) {
    let operations: Vec<String> = vec![
        String::from("addr"),
        String::from("addi"),
        String::from("mulr"),
        String::from("muli"),
        String::from("banr"),
        String::from("bani"),
        String::from("borr"),
        String::from("bori"),
        String::from("setr"),
        String::from("seti"),
        String::from("gtir"),
        String::from("gtri"),
        String::from("gtrr"),
        String::from("eqir"),
        String::from("eqri"),
        String::from("eqrr"),
    ];

    let mut opcode_matches: HashMap<i32, HashSet<String>> = HashMap::new();

    for sample in samples {
        for op in &operations {
            let mut registers = sample.before.clone();
            str_to_fun(op)(
                &mut registers,
                sample.instruction.a,
                sample.instruction.b,
                sample.instruction.c,
            );

            if registers
                .iter()
                .zip(sample.after.clone())
                .all(|(&a, b)| a == b)
            {
                let prev_list = opcode_matches
                    .entry(sample.instruction.opcode)
                    .or_insert(HashSet::new());
                (*prev_list).insert(op.clone().to_string());
            }
        }
    }

    println!("{:?}", opcode_matches);

    let mut final_opcode_matches = HashMap::new();
    let mut assigned_operations = HashSet::new();

    while final_opcode_matches.len() < 16 {
        for (opcode, possible_operations) in opcode_matches.clone().iter() {
            if possible_operations.len() == 1 {
                let operation = possible_operations
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .first()
                    .unwrap()
                    .clone();

                final_opcode_matches.insert(opcode.clone(), operation.clone());
                assigned_operations.insert(operation.clone());
            }
        }

        opcode_matches = opcode_matches
            .clone()
            .iter()
            .map(|(&code, possible_operations)| {
                (
                    code,
                    possible_operations
                        .iter()
                        .filter(|s| !assigned_operations.contains(&s.to_string()))
                        .map(|s| s.to_string())
                        .collect::<HashSet<String>>()
                        .clone(),
                )
            })
            .collect();
    }

    let mut registers = vec![0, 0, 0, 0];

    for instruction in &instructions {
        let operation = final_opcode_matches.get(&instruction.opcode).unwrap();
        str_to_fun(operation)(&mut registers, instruction.a, instruction.b, instruction.c);
    }

    println!("Part two: {}", registers[0]);
}

fn str_to_fun(s: &String) -> fn(&mut Vec<i32>, i32, i32, i32) {
    match s.as_ref() {
        "addr" => addr,
        "addi" => addi,
        "mulr" => mulr,
        "muli" => muli,
        "banr" => banr,
        "bani" => bani,
        "borr" => borr,
        "bori" => bori,
        "setr" => setr,
        "seti" => seti,
        "gtir" => gtir,
        "gtri" => gtri,
        "gtrr" => gtrr,
        "eqir" => eqir,
        "eqri" => eqri,
        "eqrr" => eqrr,
        _ => addr,
    }
}
