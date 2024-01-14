use std::{fs, cmp, isize};

#[derive(Clone, Debug)]
struct Vertex {
    x: isize,
    y: isize,
}

fn shoelace(coords: &Vec<Vertex>) -> usize {
    let mut area: isize = 0;
    for i in 0..coords.len() {
        let p1 = (coords[i].x as isize, coords[i].y as isize);
        let p2 = (
            coords[(i + 1) % coords.len()].x as isize,
            coords[(i + 1) % coords.len()].y as isize,
        );
        area += p1.0 * p2.1 - p1.1 * p2.0;
    }
    return (area.abs() / 2) as usize;
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-18").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut vertices = vec![Vertex { x, y }];
    let mut wall_length = 0;

    for line in lines {
        let tokens: Vec<String> = line.split(' ').map(|x| String::from(x)).collect();
        let length: isize = tokens[1].parse().unwrap();
        let direction = tokens[0].as_bytes()[0] as char;

        match direction {
            'R' => x = x + length,
            'L' => x = x - length,
            'U' => y = y - length,
            'D' => y = y + length,
            _ => panic!(""),
        };

        wall_length += length;
        vertices.push(Vertex { x, y });
    }
    let inner_area = shoelace(&vertices);
    let result = inner_area + (wall_length as usize / 2) + 1;
    println!("Result part 1: {}", result); 
}

fn part2(lines: Vec<String>) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut vertices = vec![Vertex { x, y }];
    let mut wall_length = 0;

    for line in lines {
        let tokens: Vec<String> = line.split(' ').map(|x| String::from(x)).collect();
        let hex = tokens[2].replace("(", "").replace(")", "").replace("#", "");
        let length = isize::from_str_radix(&hex.as_str()[..hex.len()-1], 16).unwrap();
        let direction = match hex.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            x => panic!("{x} is invalid choice."),
        };

        match direction {
            'R' => x = x + length,
            'L' => x = x - length,
            'U' => y = y - length,
            'D' => y = y + length,
            _ => panic!(""),
        };

        wall_length += length;
        vertices.push(Vertex { x, y });
    }
    let inner_area = shoelace(&vertices);
    let result = inner_area + (wall_length as usize / 2) + 1;
    println!("Result part 2: {}", result); 
}
