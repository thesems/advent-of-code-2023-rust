use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let res = fs::read_to_string("./inputs/input-12").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    solve(lines.clone());
}

fn search(
    mut springs: VecDeque<char>,
    mut sizes: VecDeque<isize>,
    states: &mut HashMap<String, usize>,
) -> usize {
    let mut combinations = 0;

    let stringify_state = |sps: &VecDeque<char>, szs: &VecDeque<isize>| -> String {
        let mut state = String::new();
        for &s in sps {
            state.push(s);
            state.push('-');
        }
        state.push('-');
        for &s in szs {
            for ch in s.to_string().chars() {
                state.push(ch);
                state.push('-');
            }
        }
        state
    };

    let state = stringify_state(&springs, &sizes);
    combinations += match states.get(&state) {
        Some(x) => *x,
        None => {
            let mut mutating = false;
            let mut valid = true;
            let org_sizes = sizes.clone();
            let mut combos = 0;

            while let Some(mut curr_ch) = springs.pop_front() {
                if curr_ch == '?' {
                    if mutating {
                        if sizes[0] == 0 {
                            curr_ch = '.';
                        } else {
                            curr_ch = '#';
                        }
                    } else {
                        // explore as .
                        combos += search(springs.clone(), sizes.clone(), states);

                        // explore as #
                        curr_ch = '#';
                    }
                }

                if curr_ch == '#' {
                    if sizes.len() == 0 || sizes[0] == 0 {
                        valid = false;
                        break;
                    }

                    sizes[0] -= 1;
                    mutating = true;
                } else if curr_ch == '.' && mutating {
                    mutating = false;
                    if sizes[0] == 0 {
                        sizes.pop_front();
                    } else if sizes[0] != org_sizes[org_sizes.len() - sizes.len()] {
                        valid = false;
                        break;
                    }
                }
            }

            if valid && sizes.iter().sum::<isize>() == 0 {
                combos += 1;
            }
            states.insert(state, combos);
            combos 
        }
    };

    combinations
}

fn solve(lines: Vec<String>) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut states = HashMap::new();

    for line in lines.iter() {
        let tokens: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
        let springs: VecDeque<char> = tokens[0].chars().into_iter().collect();
        let sizes: VecDeque<isize> = tokens[1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        {
            // part 1
            result_1 += search(springs.clone(), sizes.clone(), &mut HashMap::new());
        }
        {
            // part 2
            let mut new_springs: VecDeque<char> = VecDeque::from([]);
            (0..5).for_each(|i| {
                for ch in &springs {
                    new_springs.push_back(*ch);
                }
                if i < 4 {
                    new_springs.push_back('?');
                }
            });
            let mut new_sizes: VecDeque<isize> = VecDeque::from([]);
            (0..5).for_each(|_| {
                for n in &sizes {
                    new_sizes.push_back(*n);
                }
            });
            result_2 += search(new_springs.clone(), new_sizes.clone(), &mut states);
        }
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
    println!("Result part 1: {}", result_1);
    println!("Result part 2: {}", result_2);
}
