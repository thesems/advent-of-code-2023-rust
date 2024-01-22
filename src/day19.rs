use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Rule {
    greater: bool,
    value: usize,
    part: char,
    next: String,
    special: bool,
}

#[derive(Debug)]
struct Part {
    values: HashMap<char, usize>,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-19").unwrap();
    let lines: Vec<String> = res.split("\n").map(|x| String::from(x.trim())).collect();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let mut parsing_workflows = true;

    for line in lines {
        if line.is_empty() {
            parsing_workflows = false;
            continue;
        }

        if parsing_workflows {
            let mut workflow = Workflow {
                name: String::from(""),
                rules: vec![],
            };

            // example workflow: px{a<2006:qkq,m>2090:A,rfg}
            // work-flow name = px, rules = {a<2006:qkq,m>2090:A,rfg}
            workflow.name = line.get(..line.find("{").unwrap()).unwrap().to_string();
            let rules = line
                .get(line.find("{").unwrap() + 1..line.find("}").unwrap())
                .unwrap();

            let tokens = rules.split(",").collect::<Vec<&str>>();
            for (i, str_rule) in tokens.iter().enumerate() {
                if i == tokens.len() - 1 {
                    workflow.rules.push(Rule {
                        greater: false,
                        value: 0,
                        part: ' ',
                        next: str_rule.to_string(),
                        special: true,
                    });
                    break;
                }
                let rule = str_rule.get(..str_rule.find(":").unwrap()).unwrap();
                let part = rule.get(..1).unwrap();
                let sign = rule.get(1..2).unwrap();
                let value = rule.get(2..).unwrap().parse::<usize>().unwrap();
                let next_workflow_name = str_rule.get(str_rule.find(":").unwrap() + 1..).unwrap();

                workflow.rules.push(Rule {
                    greater: sign == ">",
                    value,
                    part: part.as_bytes()[0] as char,
                    next: next_workflow_name.to_string(),
                    special: false,
                });
            }

            workflows.insert(workflow.name.clone(), workflow);
        } else {
            // example part: {x=787,m=2655,a=1222,s=2876}
            let tokens = line
                .get(1..line.len() - 1)
                .unwrap()
                .split(",")
                .collect::<Vec<&str>>();

            let mut part = Part {
                values: HashMap::new(),
            };

            for token in tokens {
                let part_char = token.get(..1).unwrap();
                let value = token.get(2..).unwrap().parse::<usize>().unwrap();
                part.values.insert(part_char.as_bytes()[0] as char, value);
            }

            parts.push(part);
        }
    }

    part1(&workflows, &parts);
    part2(&workflows);
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) {
    let mut sum = 0;
    for part in parts {
        let mut workflow = workflows.get(&"in".to_string()).unwrap();
        let mut workflow_name = &"in".to_string();

        'wf_loop: loop {
            for rule in workflow.rules.iter() {
                let mut op_result = false;
                if !rule.special {
                    let op_greater = |a: usize, b: usize| -> bool { a > b };
                    let op_less = |a: usize, b: usize| -> bool { a < b };
                    let val = part.values.get(&rule.part).unwrap();
                    let op = if rule.greater { op_greater } else { op_less };
                    op_result = op(*val, rule.value);
                }
                if op_result || rule.special {
                    match rule.next.as_str() {
                        "A" => {
                            sum += part.values.iter().fold(0, |acc, (_, v)| acc + v);
                            break 'wf_loop;
                        }
                        "R" => {
                            break 'wf_loop;
                        }
                        _ => {
                            workflow = workflows.get(&rule.next).unwrap();
                            workflow_name = &rule.next;
                            continue 'wf_loop;
                        }
                    }
                }
            }
        }
    }

    println!("Result part 1: {}", sum);
}

fn search(
    workflows: &HashMap<String, Workflow>,
    name: &String,
    ranges: &mut HashMap<char, (usize, usize)>,
) -> usize {
    let get_combos = |ranges: &HashMap<char, (usize, usize)>| -> usize {
        let mut combos = 1;
        for item in ranges {
            combos *= item.1.1 - item.1.0 + 1;
        }
        combos
    };

    let c = get_combos(ranges);
    if c == 0 || name == "R" {
        return 0;
    }
    if name == "A" {
        return c;
    }

    let mut combos = 0;
    let workflow = &workflows.get(name).unwrap();

    let reduce = |rule: &Rule, range: (usize, usize)| -> ((usize, usize), (usize, usize)) {
        if rule.greater {
            return ((rule.value + 1, range.1), (range.0, rule.value));
        } else {
            return ((range.0, rule.value - 1), (rule.value, range.1));
        }
    };

    for rule in workflow.rules.iter() {
        let mut new_ranges = ranges.clone();
        if !rule.special {
            let range = ranges.get(&rule.part).unwrap();
            let (new_range, left_range) = reduce(rule, *range);
            ranges.insert(rule.part, left_range);
            new_ranges = ranges.clone();
            new_ranges.insert(rule.part, new_range);
        }
        combos += search(workflows, &rule.next, &mut new_ranges);
    }

    combos
}

fn part2(workflows: &HashMap<String, Workflow>) {
    let mut ranges = HashMap::new();
    ranges.insert('x', (1, 4000));
    ranges.insert('m', (1, 4000));
    ranges.insert('a', (1, 4000));
    ranges.insert('s', (1, 4000));

    let result = search(
        workflows,
        &"in".to_string(),
        &mut ranges,
    );
    println!("Result part 2: {}", result);
}
