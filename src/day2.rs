use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-2").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut sum = 0;
    for mut line in lines {
        line = line.replace(":", "");
        line = line.replace(",", "");
        line = line.replace(";", " ;");

        let tokens: Vec<&str> = line.split(" ").collect();
 
        let mut disquality = false; 
        let mut game_id: i32 = -1;

        let mut prev_token = "";
        for token in &tokens {
            if prev_token == "Game" {
                game_id = token.parse().unwrap();
            }

            if token == &"blue" {
                let count: u32 = prev_token.parse().unwrap();
                if count > blue_limit {
                    disquality = true;
                    break;
                }
            } else if token == &"red" {
                let count: u32 = prev_token.parse().unwrap();
                if count > red_limit {
                    disquality = true;
                    break;
                }
            } else if token == &"green" {
                let count: u32 = prev_token.parse().unwrap();
                if count > green_limit {
                    disquality = true;
                    break;
                }
            }

            prev_token = token;
        }

        if !disquality {
            sum = sum + game_id;
        }
    }

    println!("result part 1: {}", sum);
}

fn part2(lines: Vec<String>) {
    let mut sum = 0;
    for mut line in lines {
        line = line.replace(":", "");
        line = line.replace(",", "");
        line = line.replace(";", " ;");

        let tokens: Vec<&str> = line.split(" ").collect();

        let mut red_needs = 0;
        let mut blue_needs = 0;
        let mut green_needs = 0;

        let mut prev_token = "";
        for token in &tokens {
            if token == &"blue" {
                let count: u32 = prev_token.parse().unwrap();
                blue_needs = std::cmp::max(blue_needs, count);
            } else if token == &"red" {
                let count: u32 = prev_token.parse().unwrap();
                red_needs = std::cmp::max(red_needs, count);
            } else if token == &"green" {
                let count: u32 = prev_token.parse().unwrap();
                green_needs = std::cmp::max(green_needs, count);
            }

            prev_token = token;
        }

        sum = sum + (blue_needs * red_needs * green_needs);
    }

    println!("result part 2: {}", sum);
}
