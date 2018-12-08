use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Node {
    n_children: i32,
    n_metadata: i32,
    children: Vec<Node>,
    metadata: Vec<i32>,
}

pub fn solve() {
    let (root, meta_sum) = parse();
    // println!("{:?}", coordinates);

    let answer1 = meta_sum;
    println!("{:?}", answer1);
    let answer2 = solve2(&root);
    println!("{:?}", answer2);
}

fn parse() -> (Node, i32) {
    let filename = "input/day8input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let raw_numbers = contents
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let (root, _, meta_sum) = parse_node(&raw_numbers, 0);
    (root, meta_sum)
}

fn parse_node(raw_numbers: &Vec<i32>, start: usize) -> (Node, usize, i32) {
    let n_children = raw_numbers[start];
    let mut children = Vec::new();
    let mut next_child_start = start + 2;

    let mut meta_sum = 0;
    for _ in 0..n_children {
        let (child, end, child_meta) = parse_node(raw_numbers, next_child_start);
        next_child_start = end;
        children.push(child);
        meta_sum += child_meta;
    }

    let n_metadata = raw_numbers[start + 1];
    let mut next_entry_start = next_child_start;
    let mut metadata = Vec::new();
    for _ in 0..n_metadata {
        let meta = raw_numbers[next_entry_start];
        metadata.push(meta);
        meta_sum += meta;
        next_entry_start += 1;
    }
    let node = Node {
        n_children: n_children,
        n_metadata: n_metadata,
        children: children.to_vec(),
        metadata: metadata.to_vec(),
    };
    (node, next_entry_start, meta_sum)
}

fn solve2(root: &Node) -> i32 {
    node_value(root)
}

fn node_value(node: &Node) -> i32 {
    let mut value_sum = 0;
    if node.n_children == 0 {
        value_sum = node.metadata.iter().sum();
    } else {
        for m in &node.metadata {
            if *m > 0 && (*m as usize) - 1 < node.children.len() {
                let node_value = node_value(&node.children[*m as usize - 1]);
                value_sum += node_value;
            }
        }
    }

    value_sum
}
