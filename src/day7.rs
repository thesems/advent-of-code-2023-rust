use std::cmp::Ordering;
use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-7").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn count_chars(line: &String, ch: &char) -> u16 {
    let mut count = 0;
    for c in line.chars() {
        if c == *ch {
            count += 1;
        }
    }
    return count;
}

fn part1(lines: Vec<String>) {
    let mut hands: Vec<(String, u16, u16)> = vec![];
    for line in lines {
        let tokens: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
        let hand: String = tokens[0].clone();
        let bid: u16 = tokens[1].parse().unwrap();

        let mut strength = 0;
        let mut consumed: Vec<char> = vec![];
        for ch in hand.chars() {
            if consumed.contains(&ch) {
                continue;
            }

            let count = count_chars(&hand, &ch);
            consumed.push(ch);

            if count == 2 {
                strength += 1;
            } else if count == 3 {
                strength += 3;
            } else if count == 4 {
                strength += 5;
            } else if count == 5 {
                strength += 5;
            }

            // println!("ch-{}: count:{}, strength:{}", ch, count, strength);
        }

        hands.push((hand, strength, bid));
    }

    fn card_mapper(card: u8) -> u8 {
        match card {
            65 => 62,
            75 => 61,
            81 => 60,
            74 => 59,
            84 => 58,
            _ => card,
        }
    }

    hands.sort_by(|a, b| {
        if a.1 == b.1 {
            for i in 0..5 {
                let pow1 = card_mapper(a.0.as_bytes()[i]);
                let pow2 = card_mapper(b.0.as_bytes()[i]);

                let comp = pow1.cmp(&pow2);
                if comp != Ordering::Equal {
                    return comp;
                }
            }
            a.1.cmp(&b.1)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut result: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.2 as u32 * (i + 1) as u32;
    }

    println!("result part 1: {}", result);
}

fn part2(lines: Vec<String>) {
    let mut hands: Vec<(String, u16, u16)> = vec![];
    for line in lines {
        let tokens: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
        let hand: String = tokens[0].clone();
        let bid: u16 = tokens[1].parse().unwrap();

        let mut strength = 0;
        let mut consumed: Vec<char> = vec![];

        let mut j_count = count_chars(&hand, &'J');
        let mut j_consumed = false;
        for ch in hand.chars() {
            if consumed.contains(&ch) || ch == 'J' {
                continue;
            }

            let count = count_chars(&hand, &ch);

            consumed.push(ch);

            if j_consumed {
                j_count = 0;
            }

            // five kind = 6
            // four kind = 5
            // full house = 4
            // three kind = 3
            // two pair = 2
            // one pair = 1

            if count == 2 {
                j_consumed = true;
                if j_count >= 3 {
                    strength += 6;
                } else if j_count == 2 {
                    strength += 5;
                } else if j_count == 1 {
                    // simulate full house
                    strength += 3;
                } else {
                    // no jokers
                    strength += 1;
                }
            } else if count == 3 {
                j_consumed = true;
                if j_count >= 2 {
                    strength += 6;
                } else if j_count == 1 {
                    strength += 5;
                } else {
                    strength += 3;
                }
            } else if count == 4 {
                j_consumed = true;
                if j_count > 0 {
                    strength += 6;
                } else {
                    strength += 5;
                }
            } else if count == 5 {
                j_consumed = true;
                strength += 6;
            }
        }

        if strength == 0 {
            if j_count >= 4 {
                strength += 6;
            } else if j_count == 3 {
                strength += 5;
            } else if j_count == 2 {
                strength += 3;
            } else if j_count == 1 {
                strength += 1;
            }
        }

        hands.push((hand, strength, bid));
    }

    fn card_mapper(card: u8) -> u8 {
        match card {
            65 => 62,
            75 => 61,
            81 => 60,
            74 => 48, // J is the weakest now
            84 => 58,
            _ => card,
        }
    }

    hands.sort_by(|a, b| {
        if a.1 == b.1 {
            for i in 0..5 {
                let pow1 = card_mapper(a.0.as_bytes()[i]);
                let pow2 = card_mapper(b.0.as_bytes()[i]);

                let comp = pow1.cmp(&pow2);
                if comp != Ordering::Equal {
                    return comp;
                }
            }
            a.1.cmp(&b.1)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let mut result: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.2 as u32 * (i + 1) as u32;
    }

    println!("result part 2: {}", result);
}
