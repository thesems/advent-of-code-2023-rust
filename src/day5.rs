use std::fs;
use std::cmp;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-5").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut seeds: Vec<u64> = lines[0]
        .replace("seeds: ", "")
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut mappings: Vec<Vec<Vec<u64>>> = vec![];

    let mut i = 1;
    while i < lines.len() {
        let line = &lines[i];
        let mappings_len = mappings.len();

        if line.find(':') != None {
            mappings.push(vec![]);
        } else {
            let nums: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            mappings[mappings_len - 1].push(nums);
        }

        i = i + 1;
    }

    let mut lowest_location = std::u64::MAX;

    for seed in seeds {
        let mut i = 0;
        let mut out: u64 = seed;

        while i < mappings.len() {
            let map = &mappings[i];

            for range in map {
                let dst = range[0];
                let src = range[1];
                let len = range[2];

                if src <= out && out <= src + len {
                    out = dst + (out - src);
                    break;
                }
            }

            i = i + 1;
        }

        if out < lowest_location {
            lowest_location = out;
        }
    }

    println!("result part 1: {}", lowest_location);
}

fn part2(lines: Vec<String>) {
    let nums: Vec<u64> = lines[0]
        .replace("seeds: ", "")
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut seeds: Vec<(u64, u64)> = vec![];

    let mut i = 0;
    while i < nums.len() {
        seeds.push((nums[i], nums[i + 1]));
        i = i + 2;
    }

    let mut mappings: Vec<Vec<Vec<u64>>> = vec![];

    i = 1;
    while i < lines.len() {
        let line = &lines[i];
        let mappings_len = mappings.len();

        if line.find(':') != None {
            mappings.push(vec![]);
        } else {
            let nums: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            mappings[mappings_len - 1].push(nums);
        }

        i = i + 1;
    }

    let mut lowest_location = std::u64::MAX;

    for (start, end) in seeds {
        let mut out = start;
        let mut consumed = 0;

        while consumed < end {
            let mut min_len = std::u64::MAX;
            out = start + consumed;

            let mut i = 0;
            while i < mappings.len() {
                let map = &mappings[i];

                for range in map {
                    let dst = range[0];
                    let src = range[1];
                    let len = range[2];
                     
                    if src <= out && out < src + len {
                        min_len = cmp::min(min_len, src + len - out);
                        out = out + dst - src;
                        break; 
                    }
                }

                i = i + 1;
            }

            if min_len != std::u64::MAX {
                consumed += min_len;
            }

            if out < lowest_location {
                lowest_location = out;
            }
        }
    }

    println!("result part 2: {}", lowest_location);
}
