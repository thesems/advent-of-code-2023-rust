use std::fs;

pub fn run() {
    let res = fs::read_to_string("./inputs/input-10").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x))
        .collect();

    part12(lines.clone());
}

fn shoelace(coords: &Vec<(usize, usize)>) -> usize {
    let mut area: isize = 0;
    for i in 0..coords.len() {
        let p1 = (coords[i].0 as isize, coords[i].1 as isize);
        let p2 = (
            coords[(i + 1) % coords.len()].0 as isize,
            coords[(i + 1) % coords.len()].1 as isize,
        );
        area += p1.0 * p2.1 - p1.1 * p2.0;
    }
    return (area.abs() / 2) as usize;
}

fn part12(lines: Vec<String>) {
    let height = lines.len();
    let width = lines.iter().next().unwrap().len();

    let mut field = vec![vec!['.'; width]; height];
    let mut start: (usize, usize) = (0, 0); // format tuple(x,y)

    for (j, line) in lines.iter().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            field[j][i] = ch;

            if ch == 'S' {
                start = (i, j);
            }
        }
    }

    let mut last = start;
    let mut current = start;
    let mut steps = 0;

    let mut main_pipe: Vec<(usize, usize)> = vec![];

    loop {
        let current_char = field[current.1][current.0];
        if current_char == 'S' && steps != 0 {
            break;
        }

        main_pipe.push(current);

        // neighbours coordinates
        let upper = (current.0, std::cmp::max(0, current.1 as isize - 1) as usize);
        let lower = (current.0, std::cmp::min(height - 1, current.1 + 1));
        let left = (std::cmp::max(0, current.0 as isize - 1) as usize, current.1);
        let right = (std::cmp::min(width - 1, current.0 + 1), current.1);

        if current != upper
            && last != upper
            && "SL|J".contains(current_char)
            && "SF|7".contains(field[upper.1][upper.0])
        {
            last = current;
            current = upper;
        } else if current != lower
            && last != lower
            && "SF|7".contains(current_char)
            && "SL|J".contains(field[lower.1][lower.0])
        {
            last = current;
            current = lower;
        } else if current != left
            && last != left
            && "S7-J".contains(current_char)
            && "SL-F".contains(field[left.1][left.0])
        {
            last = current;
            current = left;
        } else if current != right
            && last != right
            && "SF-L".contains(current_char)
            && "S7-J".contains(field[right.1][right.0])
        {
            last = current;
            current = right;
        } else {
            panic!("Disconnected pipe found!");
        }

        steps += 1;
    }

    println!("Result part 1: {}", steps / 2);

    let area = shoelace(&main_pipe);
    let result = area + 1 - (main_pipe.len() as f32 / 2.).floor() as usize;
    println!("Result part 2: {}", result);
}
