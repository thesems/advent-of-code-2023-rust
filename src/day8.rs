use std::{collections::HashMap, fs};
use std::cmp::{max, min};

#[derive(Debug)]
struct GraphNode {
    label: String,
    left: String,
    right: String,
}

// Source: https://rosettacode.org/wiki/Least_common_multiple#Rust
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-8").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut graph: HashMap<String, GraphNode> = HashMap::new();

    for line in &lines[1..] {
        let tokens: Vec<String> = line
            .replace(" ", "")
            .split("=")
            .map(|x| String::from(x))
            .collect();

        let other_labels: Vec<String> = tokens[1]
            .replace(" ", "")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|x| String::from(x))
            .collect();

        graph.insert(
            tokens[0].clone(),
            GraphNode {
                label: tokens[0].clone(),
                left: other_labels[0].clone(),
                right: other_labels[1].clone(),
            },
        );
    }

    let mut steps = 0;
    let mut current_node = graph.get("AAA").unwrap();

    for ch in lines[0].chars().cycle() {
        if ch == 'L' {
            let node = graph.get(&current_node.left).unwrap();
            current_node = &node;
        } else if ch == 'R' {
            let node = graph.get(&current_node.right).unwrap();
            current_node = &node;
        }

        steps += 1;
        if current_node.label == "ZZZ" {
            break;
        }
    }

    println!("result part 1: {}", steps);
}

fn part2(lines: Vec<String>) {
    let mut graph: HashMap<String, GraphNode> = HashMap::new();

    for line in &lines[1..] {
        let tokens: Vec<String> = line
            .replace(" ", "")
            .split("=")
            .map(|x| String::from(x))
            .collect();

        let other_labels: Vec<String> = tokens[1]
            .replace(" ", "")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|x| String::from(x))
            .collect();

        graph.insert(
            tokens[0].clone(),
            GraphNode {
                label: tokens[0].clone(),
                left: other_labels[0].clone(),
                right: other_labels[1].clone(),
            },
        );
    }

    let mut current_nodes: Vec<&GraphNode> = graph
        .iter()
        .filter(|x| x.0.chars().last().unwrap() == 'A')
        .map(|x| x.1)
        .collect();

    let nodes_len = current_nodes.len();
    let mut steps_per_node: Vec<usize> = vec![];

    for i in 0..nodes_len {
        let mut steps = 0;
        for ch in lines[0].chars().cycle() {
            if ch == 'L' {
                current_nodes[i] = graph.get(&current_nodes[i].left).unwrap();
            } else {
                current_nodes[i] = graph.get(&current_nodes[i].right).unwrap();
            }
            
            steps += 1;
            if current_nodes[i].label.chars().last().unwrap() == 'Z' {
                steps_per_node.push(steps);
                break;
            }
        }
    }

    let mut result = lcm(steps_per_node[0], steps_per_node[1]);
    let steps_len = steps_per_node.len();

    for i in 2..steps_len {
        result = lcm(result, steps_per_node[i]);
    }

    println!("result part 2: {}", result);
}
