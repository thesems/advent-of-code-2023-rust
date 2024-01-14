use std::{
    collections::{HashMap, HashSet},
    fs,
    time::{SystemTime, UNIX_EPOCH},
};
use priority_queue::DoublePriorityQueue;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-17").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut nodes: HashMap<Coord, usize> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        if y > height as usize {
            height = y as i32;
        }
        for (x, ch) in line.chars().enumerate() {
            if x > width as usize {
                width = x as i32;
            }
            let w: usize = String::from(ch).parse().unwrap();
            nodes.insert(
                Coord {
                    x: x as i32,
                    y: y as i32,
                },
                w,
            );
        }
    }

    part1(&nodes, width, height);
    part2(&nodes, width, height);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_turns(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => {
                return vec![Direction::Left, Direction::Right];
            }
            Direction::Left | Direction::Right => {
                return vec![Direction::Up, Direction::Down];
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pub coord: Coord,
    pub direction: Direction,
    pub distance: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn in_bounds(&self, width: i32, height: i32) -> bool {
        if self.x < 0 || self.x > width || self.y < 0 || self.y > height {
            return false;
        }
        return true;
    }
}

fn part1(nodes: &HashMap<Coord, usize>, width: i32, height: i32) {
    let mut queue = DoublePriorityQueue::new();
    queue.push(
        State {
            coord: Coord { x: 1, y: 0 },
            direction: Direction::Right,
            distance: 1,
        },
        *nodes.get(&Coord { x: 1, y: 0 }).unwrap(),
    );
    queue.push(
        State {
            coord: Coord { x: 0, y: 1 },
            direction: Direction::Down,
            distance: 1,
        },
        *nodes.get(&Coord { x: 0, y: 1 }).unwrap(),
    );
    let mut seen = HashSet::new();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while !queue.is_empty() {
        let item = queue.pop_min().unwrap();
        let current = item.0;
        let current_cost = item.1;

        if seen.get(&current).is_some() {
            continue;
        }
        seen.insert(current.clone());

        if current.coord.x == width && current.coord.y == height {
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            println!(
                "Result part 1: {}. Finished in {}ms.",
                item.1,
                (end - start).as_millis()
            );
            return;
        }

        let turns = current.direction.get_turns();
        for turn in turns {
            let mut next_coord = current.coord.clone();

            match turn {
                Direction::Up => {
                    next_coord.y -= 1;
                }
                Direction::Down => {
                    next_coord.y += 1;
                }
                Direction::Left => {
                    next_coord.x -= 1;
                }
                Direction::Right => {
                    next_coord.x += 1;
                }
            }

            if !next_coord.in_bounds(width, height) {
                continue;
            }

            let cost = current_cost + *nodes.get(&next_coord).unwrap();
            queue.push_decrease(
                State {
                    coord: next_coord,
                    direction: turn.clone(),
                    distance: 1,
                },
                cost,
            );
        }

        // handle moving straight
        if current.distance < 3 {
            let mut next_coord = current.coord.clone();

            match current.direction {
                Direction::Up => {
                    next_coord.y -= 1;
                }
                Direction::Down => {
                    next_coord.y += 1;
                }
                Direction::Left => {
                    next_coord.x -= 1;
                }
                Direction::Right => {
                    next_coord.x += 1;
                }
            }

            if !next_coord.in_bounds(width, height) {
                continue;
            }

            let cost = current_cost + *nodes.get(&next_coord).unwrap();
            queue.push_decrease(
                State {
                    coord: next_coord,
                    direction: current.direction.clone(),
                    distance: current.distance + 1,
                },
                cost,
            );
        }
    }
}

fn part2(nodes: &HashMap<Coord, usize>, width: i32, height: i32) {
    let mut queue = DoublePriorityQueue::new();
    queue.push(
        State {
            coord: Coord { x: 1, y: 0 },
            direction: Direction::Right,
            distance: 1,
        },
        *nodes.get(&Coord { x: 1, y: 0 }).unwrap(),
    );
    queue.push(
        State {
            coord: Coord { x: 0, y: 1 },
            direction: Direction::Down,
            distance: 1,
        },
        *nodes.get(&Coord { x: 0, y: 1 }).unwrap(),
    );
    let mut seen = HashSet::new();
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    while !queue.is_empty() {
        let item = queue.pop_min().unwrap();
        let current = item.0;
        let current_cost = item.1;

        if seen.get(&current).is_some() {
            continue;
        }
        seen.insert(current.clone());

        if current.coord.x == width && current.coord.y == height && current.distance >= 4 {
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            println!(
                "Result part 2: {}. Finished in {}ms.",
                item.1,
                (end - start).as_millis()
            );
            return;
        }

        let turns = current.direction.get_turns();

        if current.distance >= 4 {
            for turn in turns {
                let mut next_coord = current.coord.clone();

                match turn {
                    Direction::Up => {
                        next_coord.y -= 1;
                    }
                    Direction::Down => {
                        next_coord.y += 1;
                    }
                    Direction::Left => {
                        next_coord.x -= 1;
                    }
                    Direction::Right => {
                        next_coord.x += 1;
                    }
                }

                if !next_coord.in_bounds(width, height) {
                    continue;
                }

                let cost = current_cost + *nodes.get(&next_coord).unwrap();
                queue.push_decrease(
                    State {
                        coord: next_coord,
                        direction: turn.clone(),
                        distance: 1,
                    },
                    cost,
                );
            }
        }

        if current.distance < 10 {
            // handle moving straight
            let mut next_coord = current.coord.clone();

            match current.direction {
                Direction::Up => {
                    next_coord.y -= 1;
                }
                Direction::Down => {
                    next_coord.y += 1;
                }
                Direction::Left => {
                    next_coord.x -= 1;
                }
                Direction::Right => {
                    next_coord.x += 1;
                }
            }

            if !next_coord.in_bounds(width, height) {
                continue;
            }

            let cost = current_cost + *nodes.get(&next_coord).unwrap();
            queue.push_decrease(
                State {
                    coord: next_coord,
                    direction: current.direction.clone(),
                    distance: current.distance + 1,
                },
                cost,
            );
        }
    }
}
