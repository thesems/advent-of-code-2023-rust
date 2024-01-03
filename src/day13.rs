use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-13").unwrap();
    let lines: Vec<String> = res.split("\n").map(|x| String::from(x)).collect();

    solve(lines.clone());
}

fn reflection_row(block: &Vec<String>, distance: usize) -> usize {
    let diff = |x: &String, y: &String| -> usize {
        let mut cnt = 0;
        for idx in 0..x.len() {
            if x.as_bytes()[idx] != y.as_bytes()[idx] {
                cnt += 1;
            }
        }
        cnt
    };

    for idx in 1..block.len() {
        if block[..idx]
            .iter()
            .rev()
            .zip(block[idx..].iter())
            .fold(0, |acc: usize, x: (&String, &String)| acc + diff(x.0, x.1))
            == distance
        {
            return idx;
        }
    }
    return 0;
}

fn solve(lines: Vec<String>) {
    let mut block: Vec<String> = vec![];
    let mut result_1 = 0;
    let mut result_2 = 0;

    for (i, line) in lines.iter().enumerate() {
        if line == "" {
            result_1 += 100 * reflection_row(&block, 0);
            result_2 += 100 * reflection_row(&block, 1);

            let mut rotated_block: Vec<String> = vec![];
            for i in 0..block[0].len() {
                let mut str = String::new();
                for j in 0..block.len() {
                    str.push(block[j].as_bytes()[i] as char);
                }
                rotated_block.push(str);
            }

            result_1 += reflection_row(&rotated_block, 0);
            result_2 += reflection_row(&rotated_block, 1);

            // new block
            block.clear();
            continue;
        }
        block.push(line.clone());
    }

    println!("Result part 1: {result_1}");
    println!("Result part 2: {result_2}");
}
