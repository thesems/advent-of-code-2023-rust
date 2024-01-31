use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
    SlopeRight,
    SlopeLeft,
    SlopeDown,
    SlopeUp,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
    next: Vec<Coord>,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y, next: vec![] }
    }
    fn get_neighbours(&self) -> Vec<Coord> {
        let mut neighbours: Vec<Coord> = Vec::new();
        neighbours.push(Coord::new(self.x.saturating_sub(1), self.y));
        neighbours.push(Coord::new(self.x + 1, self.y));
        neighbours.push(Coord::new(self.x, self.y.saturating_sub(1)));
        neighbours.push(Coord::new(self.x, self.y + 1));

        neighbours.into_iter().filter(|x| x != self).collect()
    }
}

#[derive(Debug)]
struct Map {
    values: HashMap<Coord, Tile>,
    edges: Vec<(Coord, Coord)>,
    start: Coord,
    end: Coord,
    width: usize,
    height: usize,
}
impl Map {
    fn new(
        values: HashMap<Coord, Tile>,
        start: Coord,
        end: Coord,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            values,
            edges: vec![],
            start,
            end,
            width,
            height,
        }
    }
    fn bounds(&self, coord: &Coord) -> bool {
        if coord.x > self.width || coord.y > self.height {
            return false;
        }
        return true;
    }
    fn compress(&mut self) {}
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-23").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut values: HashMap<Coord, Tile> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '>' => Tile::SlopeRight,
                '<' => Tile::SlopeLeft,
                'v' => Tile::SlopeDown,
                '^' => Tile::SlopeUp,
                x => panic!("Unhandled char: {x}"),
            };

            // TODO:
            // count neighbours, if > 2 => mark as junction
            // later, use DFS to figure out lengths and add to edges
            // build a new grid

            let coord = Coord::new(x, y);
            values.insert(coord, tile);

            if x > width {
                width = x;
            }
        }
        if y > height {
            height = y;
        }
    }

    let mut map = Map::new(
        values,
        Coord::new(1, 0),
        Coord::new(width - 1, height),
        width,
        height,
    );

    // part1(&map);
    part2(&mut map);
}

fn search(coord: Coord, steps: usize, map: &Map, mut visited: HashSet<Coord>) -> usize {
    if coord == map.end {
        println!("{steps}");
        return steps;
    }
    if visited.contains(&coord) {
        return 0;
    }
    visited.insert(coord.clone());

    let mut max_steps = 0;
    for neighbour in coord.get_neighbours().iter() {
        if !map.bounds(&neighbour) {
            continue;
        }

        let tile = map.values.get(&neighbour).unwrap();
        let skip = match tile {
            Tile::Forest => true,
            Tile::Path => false,
            Tile::SlopeDown => coord.y > neighbour.y,
            Tile::SlopeRight => coord.x > neighbour.x,
            Tile::SlopeLeft => coord.x < neighbour.x,
            Tile::SlopeUp => coord.y < neighbour.y,
        };
        if !skip {
            let s = search(neighbour.clone(), steps + 1, map, visited.clone());

            if s > max_steps {
                max_steps = s;
            }
        }
    }
    max_steps
}

fn part1(map: &Map) {
    let result = search(map.start.clone(), 0, map, HashSet::new());
    println!("Result part 1: {result}");
}

fn part2(map: &mut Map) {
    for (_, tile) in map.values.iter_mut() {
        if *tile != Tile::Forest {
            *tile = Tile::Path;
        }
    }

    let result = search(map.start.clone(), 0, map, HashSet::new());
    println!("Result part 2: {result}");
}
