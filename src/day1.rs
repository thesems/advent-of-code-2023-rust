use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-1").unwrap();
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
        let mut i = 0;
        let mut c: char = ' ';
        for ch in line.chars() {
            if ch.is_digit(10) {
                if i == 0 {
                    i = ch.to_digit(10).unwrap() * 10;
                }
                c = ch;
            }
        }
        if c != ' ' {
            i = i + c.to_digit(10).unwrap();
        } else {
            i = i + (i / 10);
        }

        sum = sum + i;
    }

    println!("result part 1: {}", sum);
}

fn part2(lines: Vec<String>) {
    let mut sum = 0;
    for mut line in lines {
        let mut i = 0;
        let mut c: char = ' ';

        let nums = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
        ];

        // replace letter digits
        for num in nums {
            loop {
                let idx = line.find(num);
                if idx == None {
                    break;
                }
                let idx = idx.unwrap();
                let r = match num {
                    "one" => '1',
                    "two" => '2',
                    "three" => '3',
                    "four" => '4',
                    "five" => '5',
                    "six" => '6',
                    "seven" => '7',
                    "eight" => '8',
                    "nine" => '9',
                    _ => ' ',
                };
                line.replace_range(idx+1..idx+num.len()-1, r.to_string().as_str());
            }
        }

        for ch in line.chars() {
            if ch.is_digit(10) {
                if i == 0 {
                    i = ch.to_digit(10).unwrap() * 10;
                }
                c = ch;
            }
        }

        if c != ' ' {
            i = i + c.to_digit(10).unwrap();
        } else {
            i = i + (i / 10);
        }

        sum = sum + i;
    }

    println!("result part 2: {}", sum);
}
