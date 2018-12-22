pub fn solve() {
    let input = parse();
    solve1(input);
    solve2(input);
}

fn parse() -> Vec<i32> {
    let filename = "input/day1input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}


fn solve1(input: i32) {
    println!("{:?},{:?}", 0, 0);
}

fn solve2(input: i32) {
    println!("{:?},{:?},{:?}", 0, 0, 0);
}
