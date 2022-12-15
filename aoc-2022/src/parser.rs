pub mod parser {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn parse_to_ints(day: i32) -> Vec<i32> {
        parse(day, "\n", |l| l.parse::<i32>().unwrap())
    }

    pub fn parse<T>(day: i32, split_by: &str, fun: fn(&str) -> T) -> Vec<T> {
        parse_str(&day.to_string(), split_by, fun)
    }

    pub fn parse_str<T>(day: &str, split_by: &str, fun: fn(&str) -> T) -> Vec<T> {
        let filename = format!("input/day{day}input");
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
            .split(split_by)
            .filter(|l| l.len() > 0)
            .map(fun)
            .collect()
    }
}
