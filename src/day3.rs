use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    len: i32,
    val: i32,
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-3").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    let mut numbers: Vec<Pos> = vec![];
    let mut symbols: Vec<Pos> = vec![];

    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let mut it = line.chars();

        let mut pos = Pos {
            x: 0,
            y: 0,
            len: 0,
            val: 0,
        };
        let mut num = String::from("");

        loop {
            let ch = match it.next() {
                Some(x) => x,
                None => break,
            };

            if !ch.is_digit(10) {
                if ch != '.' {
                    symbols.push(Pos {
                        x,
                        y,
                        len: 0,
                        val: ch as i32,
                    });
                }

                if !num.is_empty() {
                    pos.x = x - (num.len() as i32);
                    pos.y = y;
                    pos.len = num.len() as i32;
                    pos.val = num.parse().unwrap();

                    numbers.push(pos);
                    pos = Pos {
                        x: 0,
                        y: 0,
                        len: 0,
                        val: 0,
                    };
                    num = String::from("");
                }
            } else {
                num.push(ch);
            }

            x = x + 1;
        }

        if !num.is_empty() {
            pos.x = x - (num.len() as i32);
            pos.y = y;
            pos.len = num.len() as i32;
            pos.val = num.parse().unwrap();

            numbers.push(pos);
        }

        y = y + 1;
        x = 0;
    }

    let mut sum = 0;
    let mut gear = 0;

    for symbol in symbols {
        let mut i = 0;
        let mut last_num = 0;

        for num in numbers.iter() {
            if symbol.y.abs_diff(num.y) > 1 {
                continue;
            }

            if (symbol.x >= num.x - 1) && (symbol.x <= num.x + num.len) {
                sum = sum + num.val;
                i = i + 1;

                if i == 2 {
                    gear = gear + last_num * num.val; 
                }
                last_num = num.val;
            }
        }
    }

    println!("results part 1: {}", sum);
    println!("results part 2: {}", gear);
}
