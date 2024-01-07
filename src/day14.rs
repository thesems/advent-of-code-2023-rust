use std::{fs, collections::HashSet};

#[derive(PartialEq)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-14").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    part1(lines.clone());
    part2(lines.clone());
}

fn tilt_vertically(arr: &mut Vec<Vec<char>>, direction: Direction) {
    for mut i in 0..arr.len() {
        if direction == Direction::SOUTH {
            if i == 0 {
                continue;
            }
            i = arr.len() - 1 - i;
        } else if direction == Direction::NORTH && i == 0 {
            continue;
        }
        for j in 0..arr[i].len() {
            if arr[i][j] != 'O' {
                continue;
            }

            let mut n: usize;
            let mut moved = false;
            let m = match direction {
                Direction::SOUTH => 1,
                _ => -1,
            };

            n = i;
            while arr[(n as i32 + m) as usize][j] == '.' {
                n = (n as i32 + m) as usize;
                moved = true;

                if direction == Direction::NORTH && n == 0 {
                    break;
                } else if direction == Direction::SOUTH && n == arr.len() - 1 {
                    break;
                }
            }

            if moved {
                arr[n][j] = 'O';
                arr[i][j] = '.';
            }
        }
    }
}

fn tilt_horizontally(arr: &mut Vec<Vec<char>>, direction: Direction) {
    for i in 0..arr.len() {
        let row_size = arr[i].len();

        for mut j in 0..row_size {
            if j == 0 {
                continue;
            }

            let m: i32;

            if direction == Direction::EAST {
                j = row_size - 1 - j;
                m = 1; // move to the right
            } else {
                m = -1; // move to the left
            }

            if arr[i][j] != 'O' {
                continue;
            }

            let mut n = j;
            let mut moved = false;

            while arr[i][(n as i32 + m) as usize] == '.' {
                n = (n as i32 + m) as usize;
                moved = true;

                if direction == Direction::EAST && n == row_size - 1 {
                    break;
                } else if direction == Direction::WEST && n == 0 {
                    break;
                }
            }

            if moved {
                arr[i][j] = '.';
                arr[i][n] = 'O';
            }
        }
    }
}

fn part1(lines: Vec<String>) {
    let mut arr: Vec<Vec<char>> = vec![];

    for line in lines {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        arr.push(row);
    }

    tilt_vertically(&mut arr, Direction::NORTH);

    let mut result = 0;
    for (i, row) in arr.iter().enumerate() {
        for &ch in row {
            if ch == 'O' {
                result += arr.len() - i;
            }
        }
    }

    println!("Result part 1: {}", result);
}

pub fn part2(lines: Vec<String>) {
    let mut arr: Vec<Vec<char>> = vec![];

    for line in lines {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        arr.push(row);
    }

    let mut seen: HashSet<String> = HashSet::new();
    let cache = |x: &Vec<Vec<char>>| {
        let mut s = String::new();
        for line in x {
            for &ch in line {
                s.push(ch);
            }
        }
        return s;
    };

    let mut state;
    let mut cycle = 0;
    let mut arrs = vec![arr.clone()];

    loop {
        tilt_vertically(&mut arr, Direction::NORTH);
        tilt_horizontally(&mut arr, Direction::WEST);
        tilt_vertically(&mut arr, Direction::SOUTH);
        tilt_horizontally(&mut arr, Direction::EAST);
        cycle += 1;
        
        state = cache(&arr);
        if seen.contains(&state) {
            break;
        } else {
            seen.insert(state.clone());
            arrs.push(arr.clone());
        }
    }

    let first = arrs.iter().position(|x| cache(x) == state).unwrap();
    let grid = &arrs[(1000000000 - first) % (cycle - first) + first];

    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for &ch in row {
            if ch == 'O' {
                result += arr.len() - i;
            }
        }
    }

    println!("Result part 2: {}", result);
}
