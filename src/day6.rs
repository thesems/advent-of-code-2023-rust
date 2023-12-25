use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-6").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let times: Vec<u32> = lines[0]
        .replace("Time: ", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines[1]
        .replace("Distance: ", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let mut ways = 0;
        let mut m = 0;
        while m < *time {
            let dist = (time - m) * m;
            m += 1;

            if dist > distances[i] {
                ways += 1;
            }
        }

        result *= ways;
    }

    println!("result part 1: {}", result);
}

fn part2(lines: Vec<String>) {
    let times: Vec<String> = lines[0]
        .replace("Time: ", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    let mut time_str = String::from("");
    for t in times {
        time_str += t.as_str();
    }
    let time: u32 = time_str.parse().unwrap(); 

    let distances: Vec<String> = lines[1]
        .replace("Distance: ", "")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    let mut distance_str = String::from("");
    for d in distances {
        distance_str += d.as_str();
    }
    let distance: u64 = distance_str.parse().unwrap(); 

    let mut result = 0;
    let mut m = 0;
    while m < time {
        let dist: u64 = (time - m) as u64 * m as u64;
        m += 1;

        if dist as u64 > distance {
            result += 1;
        }
    }

    println!("result part 2: {}", result);
}
