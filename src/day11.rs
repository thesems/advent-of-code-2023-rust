use std::{fs, collections::HashMap};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-11").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    solve(lines.clone(), 2);
    solve(lines, 1_000_000);
}

fn solve(lines: Vec<String>, scale: usize) {
    let mut field: Vec<(usize, usize)> = vec![];

    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                field.push((i, j));
            }
        }
    }

    let mut expand: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // Add new horizontal lines
    for (j, line) in lines.iter().enumerate() {
        if line.find('#').is_none() {
            for f in field.iter_mut() {
                if f.1 > j {
                    let a = match expand.get(f) {
                        Some(x) => (x.0, x.1 + 1),
                        None => (0, 1),
                    };

                    expand.insert(*f, a);
                }
            }
        }
    }

    // Add new vertical lines
    for i in 0..lines[0].len() {
        if lines.iter().all(|x| x.as_bytes()[i] as char == '.') {
            for f in field.iter_mut() {
                if f.0 > i {
                    let a = match expand.get(f) {
                        Some(x) => (x.0 + 1, x.1),
                        None => (1, 0),
                    };

                    expand.insert(*f, a);
                }
            }
        }
    }

    for f in field.iter_mut() {
        match expand.get(f) {
            Some(x) => {
                f.0 += x.0 * (scale - 1);
                f.1 += x.1 * (scale - 1);
                ()
            },
            None => ()
        }
    }

    // Calculate shortest paths
    let mut total = 0;
    for n in &field {
        for m in &field {
            if n == m {
                continue;
            }

            total += n.0.abs_diff(m.0) + n.1.abs_diff(m.1);
        }
    }

    println!("Results for scale {}: {}", scale, total / 2);
}
