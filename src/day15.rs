use std::{collections::HashMap, fs};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-15").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut result = 0;

    for line in lines {
        let tokens: Vec<String> = line.split(',').map(|x| String::from(x)).collect();
        for token in tokens {
            let mut temp = 0;
            for ch in token.as_bytes() {
                temp += *ch as u64;
                temp *= 17;
                temp %= 256;
            }
            result += temp;
        }
    }

    println!("Result part 1: {}", result);
}

pub fn part2(lines: Vec<String>) {
    let mut map: HashMap<u32, Vec<String>> = HashMap::new();

    let hash = |x: String| -> u32 {
        let mut temp = 0;
        for ch in x.as_bytes() {
            temp += *ch as u32;
            temp *= 17;
            temp %= 256;
        }
        temp
    };

    for line in lines {
        let tokens: Vec<String> = line.split(',').map(|x| String::from(x)).collect();
        for token in tokens {
            let mut add = false;
            if token.contains("=") {
                add = true;
            }

            let text = token.replace('=', " ").replace('-', " ");
            let num = hash(text.split(' ').next().unwrap().to_string());

            if map.get(&num).is_none() {
                map.insert(num, vec![]);
            }

            let ele = map.get_mut(&num).unwrap();
            let idx = ele.iter().position(|x| {
                let l = x.split(' ')
                    .into_iter()
                    .next()
                    .unwrap();
                let t = &text.split(' ').into_iter().next().unwrap();
                return &l == t;
            });

            if add {
                if idx.is_some() {
                    ele[idx.unwrap()] = text;
                } else {
                    ele.push(text);
                }
            } else {
                if idx.is_some() {
                    ele.remove(idx.unwrap());
                }
            }
        }
    }

    let mut result = 0;
    for (k, v) in map.iter() {
        for (i, e) in v.iter().enumerate() {
            let num = e
                .split(' ')
                .into_iter()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let t = (k + 1) * ((i + 1) as u32) * num;
            result += t;
        }
    }

    println!("Result part 2: {}", result);
}
