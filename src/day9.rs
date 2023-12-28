use std::{fs, collections::VecDeque};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-9").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut result = 0;

    for line in lines.iter() {
        let mut nums: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut diffs: Vec<Vec<i32>> = vec![nums.clone()];

        loop {
            let mut diff: Vec<i32> = vec![];
            for i in 0..nums.len()-1{
                diff.push(nums[i + 1] - nums[i]);
            }
            
            diffs.push(diff.clone());
            
            let sum: i32 = diff.iter().sum();
            if sum == 0 {
                break;
            }

            nums = diff;
        }

        diffs = diffs.into_iter().rev().collect();
        for i in 0..diffs.len() { 
            if i == 0 {
                diffs[i].push(0);
                continue;
            }

            let prediction = diffs[i-1].iter().last().unwrap() + diffs[i].iter().last().unwrap();
            diffs[i].push(prediction);

            if i == diffs.len() - 1 {
                result += prediction;
            }
        }
    }

    println!("Result part 1: {}", result);
}

fn part2(lines: Vec<String>) {
    let mut result = 0;

    for line in lines.iter().rev() {
        let mut nums: VecDeque<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut diffs: VecDeque<VecDeque<i32>> = VecDeque::from([nums.clone()]);

        loop {
            let mut diff: VecDeque<i32> = VecDeque::new();
            for i in 0..nums.len()-1{
                diff.push_back(nums[i + 1] - nums[i]);
            }
            
            diffs.push_back(diff.clone());
            
            let sum: i32 = diff.iter().sum();
            if sum == 0 {
                break;
            }

            nums = diff;
        }

        diffs = diffs.into_iter().rev().collect();
        for i in 0..diffs.len() { 
            if i == 0 {
                diffs[i].push_front(0);
                continue;
            }

            let prediction = diffs[i].iter().next().unwrap() - diffs[i-1].iter().next().unwrap();
            diffs[i].push_front(prediction);

            if i == diffs.len() - 1 {
                result += prediction;
            }
        }
    }

    println!("Result part 2: {}", result);
}
