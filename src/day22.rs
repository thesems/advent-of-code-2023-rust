use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
    supported_by: Vec<usize>,
    supporting: Vec<usize>,
}

impl Brick {
    fn new(id: usize, x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) -> Self {
        Self {
            id,
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            supported_by: vec![],
            supporting: vec![],
        }
    }

    fn intersect(&self, brick: &Brick) -> bool {
        let mut intersect = false;
        for i in self.x1..self.x2 + 1 {
            if (brick.x1..brick.x2 + 1).contains(&i) {
                intersect = true;
            }
        }

        if !intersect {
            return false;
        }

        intersect = false;
        for i in self.y1..self.y2 + 1 {
            if (brick.y1..brick.y2 + 1).contains(&i) {
                intersect = true;
            }
        }
        if !intersect {
            return false;
        }
        
        self.z1 <= brick.z2 && brick.z1 <= self.z2
    }
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-22").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut bricks: Vec<Brick> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let tokens: Vec<&str> = line.split("~").collect();
        let start: Vec<&str> = tokens[0].split(",").collect();
        let end: Vec<&str> = tokens[1].split(",").collect();

        bricks.push(Brick::new(
            i,
            start[0].parse().unwrap(),
            start[1].parse().unwrap(),
            start[2].parse().unwrap(),
            end[0].parse().unwrap(),
            end[1].parse().unwrap(),
            end[2].parse().unwrap(),
        ));
    }

    part1(&mut bricks);
}

fn part1(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|x| x.z1);

    for i in 0..bricks.len() {
        let (at_rest, falling) = bricks.split_at_mut(i);

        let brick = falling.iter_mut().next().unwrap();
        loop {
            if brick.z1 == 1 {
                break;
            }
            brick.z1 -= 1;
            brick.z2 -= 1;

            let mut collided = false;
            for other in at_rest.iter_mut() {
                if brick.intersect(other) {
                    brick.supported_by.push(other.id);
                    other.supporting.push(brick.id);
                    collided = true;
                }
            }

            if collided {
                brick.z1 += 1;
                brick.z2 += 1;
                break;
            }
        }
    }

    let mut deletable = HashSet::new();
    for brick in bricks.iter() {
        if brick.supporting.len() == 0 {
            deletable.insert(brick.id);
        }

        'outer: for supported_by in brick.supported_by.iter() {
            for other in bricks.iter() {
                if other.supported_by.contains(&supported_by) && other.supported_by.len() == 1 {
                    continue 'outer;
                }
            }

            deletable.insert(*supported_by);
        }
    }

    println!("Result part 1: {}", deletable.len());
}
