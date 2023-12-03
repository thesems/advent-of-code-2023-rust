use std::{fs, collections::HashMap};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-4").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let mut winning_nums: Vec<u32> = vec![];
        let mut picked_nums: Vec<u32> = vec![];

        let mut found_bar = false;
        let tokens = line.split(' ');
        for token in tokens {
            let num: i32 = token.parse().unwrap_or_else(|_| { -1 });

            if token == "|" {
                found_bar = true;
            }

            if num != -1 {
                if !found_bar {
                    winning_nums.push(num as u32);
                } else {
                    picked_nums.push(num as u32);
                }
            }
        }

        let mut points = 0;
        for num in winning_nums {
            if picked_nums.contains(&num) {
                if points == 0 {
                    points = 1;
                } else {
                    points = points + points;
                }
            }
        }

        sum = sum + points;
    }

    println!("Result part 1: {}", sum);
}

fn part2(lines: Vec<String>) {
    let mut scratches = 0;
    let mut map: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let mut winning_nums: Vec<u32> = vec![];
        let mut picked_nums: Vec<u32> = vec![];

        let mut found_bar = false;
        let mut card_id: u32 = 0;
        let tokens = line.split(' ');
        for token in tokens {
            let num: i32 = token.replace(":", "").parse().unwrap_or_else(|_| { -1 });

            if token == "|" {
                found_bar = true;
            }

            if num != -1 {
                if card_id == 0 {
                    card_id = num as u32;
                    continue;
                }

                if !found_bar {
                    winning_nums.push(num as u32);
                } else {
                    picked_nums.push(num as u32);
                }
            }
        }

        let curr_cnt = match map.get(&card_id) {
            Some(x) => *x as u32,
            None => {
                map.insert(card_id, 1);
                1
            }
        };

        let mut cnt = 0;
        for num in winning_nums {
            if picked_nums.contains(&num) {
                cnt = cnt + 1;

                let t = match map.get(&(card_id + cnt)) {
                    Some(x) => *x,
                    None => {
                        map.insert(card_id + cnt, 1);
                        1
                    }
                };

                map.insert(card_id + cnt, curr_cnt + t);
            }
        }

        scratches = scratches + curr_cnt;
    }

    println!("Result part 2: {}", scratches);
}
