use std::{
    collections::VecDeque,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-12").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    solve(lines.clone());
}

fn get_permutations(springs: Vec<char>) -> Vec<Vec<char>> {
    let mut perms = vec![];
    let mut check_springs = VecDeque::from([springs]);

    while let Some(mut sp) = check_springs.pop_front() {
        if sp.iter().all(|x| *x != '?') {
            perms.push(sp);
            continue;
        }

        for i in 0..sp.len() {
            if sp[i] == '?' {
                let mut sp_alt = sp.clone();

                sp[i] = '.';
                sp_alt[i] = '#';

                check_springs.push_back(sp.clone());
                check_springs.push_back(sp_alt);
                break;
            }
        }
    }

    perms
}

fn is_valid(springs: &Vec<char>, sizes: &Vec<u32>, org_sizes: &Vec<u32>) -> bool {
    let mut idx = 0;
    let mut count = sizes[idx];

    for &ch in springs {
        if ch == '#' {
            if count == 0 {
                return false;
            }
            count -= 1;
        } else if ch == '.' {
            if idx == sizes.len() {
                continue;
            }
            if count == 0 {
                idx += 1;
                if idx < sizes.len() {
                    count = sizes[idx];
                } else {
                    count = 0;
                }
            } else if count != org_sizes[idx] {
                return false;
            }
        }
    }

    return count == 0 && idx >= sizes.len() - 1;
}

fn solve(lines: Vec<String>) {
    let mut result = 0;
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for (i, line) in lines.iter().enumerate() {
        let tokens: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
        let springs: VecDeque<char> = tokens[0].chars().into_iter().collect();
        let sizes: VecDeque<i32> = tokens[1]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let perms = get_permutations(springs);
        let org_sizes = sizes.clone();
        result += perms
            .iter()
            .filter(|x| is_valid(x, &sizes, &org_sizes))
            .count();
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
    println!("Result part 1: {}", result);
}
