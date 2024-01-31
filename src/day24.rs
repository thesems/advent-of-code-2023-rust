use std::fs;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-24").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut hailstones = vec![];
    for line in lines {
        let tokens = line
            .replace("@", ",")
            .replace(" ", "")
            .split(",")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        hailstones.push(Hailstone {
            x: tokens[0].parse::<f64>().unwrap(),
            y: tokens[1].parse::<f64>().unwrap(),
            z: tokens[2].parse::<f64>().unwrap(),
            vx: tokens[3].parse::<f64>().unwrap(),
            vy: tokens[4].parse::<f64>().unwrap(),
            vz: tokens[5].parse::<f64>().unwrap(),
        });
    }

    part1(&hailstones);
}

fn part1(hailstones: &Vec<Hailstone>) {
    // let range_low = 7f64;
    // let range_high = 27f64;
    let range_low = 200000000000000f64;
    let range_high = 400000000000000f64;

    let mut result = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones[..i].iter() {
            let dx = other.x - hailstone.x;
            let dy = other.y - hailstone.y;
            let det = other.vx * hailstone.vy - other.vy * hailstone.vx;

            if det == 0.0 {
                continue;
            }

            let u = (dy * other.vx - dx * other.vy) / det;
            let v = (dy * hailstone.vx - dx * hailstone.vy) / det;

            if u < 0.0 || v < 0.0 {
                continue;
            }

            let x = other.x + other.vx * v;
            let y = other.y + other.vy * v;

            if x >= range_low && x <= range_high && y >= range_low && y <= range_high {
                if [hailstone, other]
                    .iter()
                    .all(|stone| (x - stone.x) * stone.vx >= 0.0 && (y - stone.y) * stone.vy >= 0.0)
                {
                    result += 1;
                }
            }
        }
    }
    println!("Result part 1: {result}");
}
