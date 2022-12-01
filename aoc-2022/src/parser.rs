pub mod parser {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn parse(day: i32) -> Vec<i32> {
        let filename = format!("input/day{day}input");
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect()
    }

    pub fn group_parse(day: i32) -> Vec<Vec<i32>> {
        let filename = format!("input/day{day}input");
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
            .split("\n\n")
            .map(|g|
                g
                    .lines()
                    .filter(|l| l.len() > 0)
                    .map(|l| l.parse::<i32>().unwrap())
                    .collect())
            .collect()
    }
}