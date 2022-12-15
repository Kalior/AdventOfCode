use crate::parser::parser;
use std::collections::HashMap;

type Input = Vec<String>;

pub fn solve() {
    let input = parse();
    let one = solve1(input.to_vec());
    println!("Part one: {}", one);
    let two = solve2(input.to_vec());
    println!("Part two: {}", two);
}

fn parse() -> Input {
    parser::parse(7, "$ ", |line| line.to_string())
}

fn solve1(input: Input) -> usize {
    let directory_size = get_directory_sizes(input);

    let mut weird_sum = 0;
    for (_, size) in directory_size.iter() {
        if *size <= 100000 {
            weird_sum += size;
        }
    }
    weird_sum
}

fn solve2(input: Input) -> usize {
    let directory_size = get_directory_sizes(input);

    let free_space = 70000000 - directory_size.get("/").unwrap();
    let space_needed = 30000000 - free_space;

    let mut removed_size = usize::MAX;
    for (_, size) in directory_size.iter() {
        if *size >= space_needed {
            if *size < removed_size {
                removed_size = *size;
            }
        }
    }
    removed_size
}

fn get_directory_sizes(input: Input) -> HashMap<String, usize> {
    let mut cwd: Vec<&str> = vec!["/"];
    let mut directory_size: HashMap<String, usize> = HashMap::new();

    for command_group in input.iter() {
        for line in command_group.lines() {
            let splits: Vec<&str> = line.split(" ").collect();
            let cmd = splits[0];

            let full_path = cwd.join("/");

            if cmd == "cd" {
                let cd_arg = splits[1];
                if cd_arg == ".." {
                    cwd.pop();
                } else if cd_arg == "/" {
                    cwd = vec!["/"];
                } else {
                    cwd.push(cd_arg);
                }
            } else if cmd == "ls" {
            } else if cmd == "dir" {
                let _ = splits[1];
            } else {
                let filesize = cmd.parse::<usize>().unwrap();
                directory_size
                    .entry(full_path)
                    .and_modify(|size| *size += filesize)
                    .or_insert(filesize);

                for i in 0..cwd.len() {
                    let full_path = cwd[..i].join("/");
                    directory_size
                        .entry(full_path)
                        .and_modify(|size| *size += filesize)
                        .or_insert(filesize);
                }
            }
        }
    }
    directory_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = parse();
        assert_eq!(solve1(input), 1454188);
    }

    #[test]
    fn part_two_test() {
        let input = parse();
        assert_eq!(solve2(input), 4183246);
    }
}
