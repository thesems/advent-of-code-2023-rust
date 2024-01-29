use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn get_neighbours(&self) -> Vec<Position> {
        let mut neighbours: Vec<Position> = Vec::new();
        neighbours.push(Position::new(self.x.saturating_sub(1), self.y));
        neighbours.push(Position::new(self.x + 1, self.y));
        neighbours.push(Position::new(self.x, self.y.saturating_sub(1)));
        neighbours.push(Position::new(self.x, self.y + 1));
        neighbours
    }
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-21").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut fields: HashMap<Position, char> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            fields.insert(Position::new(x, y), ch);
        }
    }

    part1(&fields, 64);
}

fn part1(fields: &HashMap<Position, char>, max_steps: usize) {
    let start_pos = fields.iter().find(|field| *field.1 == 'S').unwrap().0;
    let mut queue = VecDeque::from(vec![(0, start_pos.clone())]);
    let mut seen = HashSet::new();
    let mut visited: HashSet<(usize, Position)> = HashSet::new();

    while !queue.is_empty() {
        let (steps, pos) = queue.pop_front().unwrap();
        if steps > max_steps || seen.contains(&pos) {
            continue;
        }
        visited.insert((steps, pos.clone()));
        seen.insert(pos.clone());

        let neighbours = pos.get_neighbours();
        for n in neighbours {
            if !fields.contains_key(&n) {
                continue;
            }
            if fields.get(&n).unwrap() == &'#' {
                continue;
            }
            if seen.contains(&n) {
                continue;
            }

            queue.push_back((steps + 1, n.clone()));
        }
    }

    let reachable_plots = visited
        .iter()
        .filter(|(steps, _)| steps % 2 == max_steps % 2)
        .count();
    println!("Results part 1: {}", reachable_plots);
}
