// Approached taken from https://www.reddit.com/user/LtHummus/
// Somewhat flaky/probabilistic solution. Not very hapy with it.

use rand::seq::SliceRandom;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}
impl Graph {
    fn new(edges: HashMap<String, Vec<String>>) -> Self {
        Self { edges }
    }
    fn path_from(&self, start: &String, end: &String) -> Vec<String> {
        let mut queue: VecDeque<(&String, Vec<String>)> = VecDeque::from([(start, vec![])]);

        while !queue.is_empty() {
            let (node, mut seen) = queue.pop_front().unwrap();
            if seen.contains(node) {
                continue;
            }

            seen.push(node.to_string());
            if node == end {
                return seen;
            }

            let items = self.edges.get(node).unwrap();
            for item in items {
                queue.push_back((item, seen.clone()));
            }
        }

        panic!()
    }
    fn remove_edge(&mut self, a: &String, b: &String) {
        let node_a = self.edges.get_mut(a).unwrap();
        if let Some(index) = node_a.iter().position(|x| x == a || x == b) {
            node_a.remove(index);
        }

        let node_b = self.edges.get_mut(b).unwrap();
        if let Some(index) = node_b.iter().position(|x| x == a || x == b) {
            node_b.remove(index);
        }
    }
    fn count(&self, start: &String) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        let mut queue: Vec<&String> = vec![start];

        while !queue.is_empty() {
            let item = queue.pop().unwrap();
            let res = seen.insert(item.clone());
            if res {
                let items = self.edges.get(item).unwrap();
                queue.extend(items);
            }
        }

        seen.len()
    }
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-25").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let tokens: Vec<String> = line
            .replace(":", "")
            .split(" ")
            .map(|x| String::from(x))
            .collect();

        let name = tokens[0].clone();
        let mut connections: Vec<String> = tokens.into_iter().skip(1).collect();

        for conn in &connections {
            if !edges.contains_key(conn) {
                edges.insert(conn.clone(), vec![name.clone()]);
            } else {
                let conns = edges.get_mut(conn).unwrap();
                conns.push(name.clone());
            }
        }

        if let Some(existing_conns) = edges.get_mut(&name) {
            existing_conns.append(&mut connections);
        } else {
            edges.insert(name, connections);
        }
    }

    let mut graph = Graph::new(edges);
    let mut histogram: HashMap<(String, String), usize> = HashMap::new();

    let keys: Vec<&String> = graph.edges.keys().collect();
    for i in 0..300 {
        let items: Vec<&String> = keys
            .choose_multiple(&mut rand::thread_rng(), 2)
            .map(|x| *x)
            .collect();

        if items[0] == items[1] {
            continue;
        }

        let path = graph.path_from(items[0], items[1]);
        
        let mut prev = path[0].clone();
        for p in path.iter().skip(1) {
            let count = histogram.get(&(prev.clone(), p.clone())).unwrap_or(&0);
            histogram.insert((prev.clone(), p.clone()), count + 1);
            prev = p.clone();
        }
    }

    let mut intermediate: Vec<(&(String, String), &usize)> = histogram.iter().collect();
    intermediate.sort_by(|a, b| b.1.cmp(a.1));

    let mut to_remove: Vec<&(String, String)> = vec![];
    'outer: for edge in intermediate.iter() {
        
        for i in to_remove.iter() {
            if edge.0.0 == i.1 && edge.0.1 == i.0 {
                continue 'outer;
            }
        }

        to_remove.push(edge.0);
        if to_remove.len() == 3 {
            break;
        }
    }

    graph.remove_edge(&to_remove[0].0, &to_remove[0].1);
    graph.remove_edge(&to_remove[1].0, &to_remove[1].1);
    graph.remove_edge(&to_remove[2].0, &to_remove[2].1);

    println!(
        "Result: {}",
        graph.count(&to_remove[0].0) * graph.count(&to_remove[1].0)
    );
}
