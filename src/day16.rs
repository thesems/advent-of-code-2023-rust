use std::{collections::HashSet, fs};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-16").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut mirrors: Vec<(isize, isize, char)> = vec![];
    let (mut width, mut height): (isize, isize) = (0, 0);

    for (j, line) in lines.iter().enumerate() {
        if j > height as usize {
            height = j as isize;
        }
        for (i, ch) in line.chars().enumerate() {
            if i > width as usize {
                width = i as isize;
            }
            if ch != '.' {
                mirrors.push((i as isize, j as isize, ch));
            }
        }
    }

    let mut queue: Vec<(isize, isize, char)> = vec![(0, 0, 'r')];
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize, char)> = HashSet::new();

    while !queue.is_empty() {
        let mut item = queue.pop().unwrap();

        loop {
            if item.0 < 0 || item.0 > width || item.1 < 0 || item.1 > height {
                break;
            }

            if visited.contains(&item) {
                break;
            }

            set.insert((item.0, item.1));
            visited.insert(item);

            let mirror = mirrors.iter().find(|x| x.0 == item.0 && x.1 == item.1);

            if mirror.is_some() {
                let mirror_sign = mirror.unwrap().2;
                match mirror_sign {
                    '/' => match item.2 {
                        'r' => {
                            item.1 -= 1;
                            item.2 = 'u';
                        }
                        'l' => {
                            item.1 += 1;
                            item.2 = 'd';
                        }
                        'u' => {
                            item.0 += 1;
                            item.2 = 'r';
                        }
                        'd' => {
                            item.0 -= 1;
                            item.2 = 'l';
                        }
                        _ => panic!(),
                    },
                    '\\' => match item.2 {
                        'l' => {
                            item.1 -= 1;
                            item.2 = 'u';
                        }
                        'r' => {
                            item.1 += 1;
                            item.2 = 'd';
                        }
                        'd' => {
                            item.0 += 1;
                            item.2 = 'r';
                        }
                        'u' => {
                            item.0 -= 1;
                            item.2 = 'l';
                        }
                        _ => panic!(),
                    },
                    '-' => match item.2 {
                        'u' | 'd' => {
                            // put left on the queue
                            queue.push((item.0 - 1, item.1, 'l'));

                            // move right
                            item.0 += 1;
                            item.2 = 'r';
                        }
                        'r' => {
                            item.0 += 1;
                        }
                        'l' => {
                            item.0 -= 1;
                        }
                        _ => panic!(),
                    },
                    '|' => match item.2 {
                        'l' | 'r' => {
                            // put up on the queue
                            queue.push((item.0, item.1 - 1, 'u'));

                            // move down
                            item.1 += 1;
                            item.2 = 'd';
                        }
                        'u' => {
                            item.1 -= 1;
                        }
                        'd' => {
                            item.1 += 1;
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            } else {
                match item.2 {
                    'r' => {
                        item.0 += 1;
                    }
                    'l' => {
                        item.0 -= 1;
                    }
                    'u' => {
                        item.1 -= 1;
                    }
                    'd' => {
                        item.1 += 1;
                    }
                    _ => panic!(),
                }
            }
        }
    }

    println!("Result part 1: {}", set.len());
}

fn part2(lines: Vec<String>) {
    let mut mirrors: Vec<(isize, isize, char)> = vec![];
    let (mut width, mut height): (isize, isize) = (0, 0);

    for (j, line) in lines.iter().enumerate() {
        if j > height as usize {
            height = j as isize;
        }
        for (i, ch) in line.chars().enumerate() {
            if i > width as usize {
                width = i as isize;
            }
            if ch != '.' {
                mirrors.push((i as isize, j as isize, ch));
            }
        }
    }

    let mut initial_beams: Vec<(isize, isize, char)> = vec![];
    let mut queue: Vec<(isize, isize, char)> = vec![];

    for y in 0..height + 1 {
        if y == 0 || y == height {
            for x in 0..width + 1 {
                if x == 0 {
                    initial_beams.push((x, y, 'r'));
                } else if x == width {
                    initial_beams.push((x, y, 'l'));
                }

                if y == 0 {
                    initial_beams.push((x, y, 'd'));
                } else if y == height {
                    initial_beams.push((x, y, 'u'));
                }
            }
        } else {
            initial_beams.push((0, y, 'r'));
            initial_beams.push((width, y, 'l'));
        }
    }

    let mut max_energized = 0;
    while !initial_beams.is_empty() {
        let beam = initial_beams.pop().unwrap();
        queue.push(beam);

        let mut visited: HashSet<(isize, isize, char)> = HashSet::new();
        let mut set: HashSet<(isize, isize)> = HashSet::new();
        while !queue.is_empty() {
            let mut item = queue.pop().unwrap();

            loop {
                if item.0 < 0 || item.0 > width || item.1 < 0 || item.1 > height {
                    break;
                }

                if visited.contains(&item) {
                    break;
                }

                set.insert((item.0, item.1));
                visited.insert(item);

                let mirror = mirrors.iter().find(|x| x.0 == item.0 && x.1 == item.1);

                if mirror.is_some() {
                    let mirror_sign = mirror.unwrap().2;
                    match mirror_sign {
                        '/' => match item.2 {
                            'r' => {
                                item.1 -= 1;
                                item.2 = 'u';
                            }
                            'l' => {
                                item.1 += 1;
                                item.2 = 'd';
                            }
                            'u' => {
                                item.0 += 1;
                                item.2 = 'r';
                            }
                            'd' => {
                                item.0 -= 1;
                                item.2 = 'l';
                            }
                            _ => panic!(),
                        },
                        '\\' => match item.2 {
                            'l' => {
                                item.1 -= 1;
                                item.2 = 'u';
                            }
                            'r' => {
                                item.1 += 1;
                                item.2 = 'd';
                            }
                            'd' => {
                                item.0 += 1;
                                item.2 = 'r';
                            }
                            'u' => {
                                item.0 -= 1;
                                item.2 = 'l';
                            }
                            _ => panic!(),
                        },
                        '-' => match item.2 {
                            'u' | 'd' => {
                                // put left on the queue
                                queue.push((item.0 - 1, item.1, 'l'));

                                // move right
                                item.0 += 1;
                                item.2 = 'r';
                            }
                            'r' => {
                                item.0 += 1;
                            }
                            'l' => {
                                item.0 -= 1;
                            }
                            _ => panic!(),
                        },
                        '|' => match item.2 {
                            'l' | 'r' => {
                                // put up on the queue
                                queue.push((item.0, item.1 - 1, 'u'));

                                // move down
                                item.1 += 1;
                                item.2 = 'd';
                            }
                            'u' => {
                                item.1 -= 1;
                            }
                            'd' => {
                                item.1 += 1;
                            }
                            _ => panic!(),
                        },
                        _ => panic!(),
                    }
                } else {
                    match item.2 {
                        'r' => {
                            item.0 += 1;
                        }
                        'l' => {
                            item.0 -= 1;
                        }
                        'u' => {
                            item.1 -= 1;
                        }
                        'd' => {
                            item.1 += 1;
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        if set.len() > max_energized {
            max_energized = set.len();
        }
    }

    println!("Result part 2: {}", max_energized);
}
