use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    next_modules: Vec<String>,
}
impl FlipFlop {
    fn new(name: String, next_device: Vec<String>) -> Self {
        Self {
            name,
            state: false,
            next_modules: next_device,
        }
    }
}

#[derive(Debug, Clone)]
struct Conjuction {
    name: String,
    next_modules: Vec<String>,
    last_states: HashMap<String, Pulse>,
}
impl Conjuction {
    fn new(name: String, next_devices: Vec<String>) -> Self {
        Self {
            name,
            next_modules: next_devices,
            last_states: HashMap::new(),
        }
    }

    fn update_state(&mut self, in_module: &String, pulse: Pulse) {
        self.last_states.insert(in_module.clone(), pulse);
    }

    fn pulse(&self) -> Pulse {
        if self
            .last_states
            .iter()
            .any(|(_, pulse)| pulse == &Pulse::Low)
        {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Debug)]
struct Broadcast {
    next_modules: Vec<String>,
}

pub fn run() {
    let res = fs::read_to_string("./inputs/input-20").unwrap();
    let lines: Vec<String> = res
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x.trim()))
        .collect();

    let mut flip_flops: HashMap<String, FlipFlop> = HashMap::new();
    let mut conjuctions: HashMap<String, Conjuction> = HashMap::new();
    let mut broadcast = Broadcast {
        next_modules: vec![],
    };

    for line in lines {
        let tokens = line
            .split("->")
            .map(|x| String::from(x.trim()))
            .collect::<Vec<String>>();

        let next_modules: Vec<String> = tokens[1]
            .replace(" ", "")
            .split(",")
            .map(|x| String::from(x))
            .collect();

        if tokens[0] == "broadcaster" {
            broadcast.next_modules = next_modules;
        } else if tokens[0].as_bytes()[0] == b'%' {
            let ff = FlipFlop::new(String::from(&tokens[0][1..]), next_modules);
            flip_flops.insert(ff.name.clone(), ff);
        } else if tokens[0].as_bytes()[0] == b'&' {
            let conjuction = Conjuction::new(String::from(&tokens[0][1..]), next_modules);
            conjuctions.insert(conjuction.name.clone(), conjuction);
        }
    }

    let conjuctions_clone: Vec<Conjuction> = conjuctions.values().map(|x| x.clone()).collect();
    for (name, conjuction) in &mut conjuctions {
        for (in_name, ff) in &flip_flops {
            if ff.next_modules.contains(name) {
                conjuction.last_states.insert(in_name.clone(), Pulse::Low);
            }
        }

        for other_conjuction in conjuctions_clone.iter() {
            if other_conjuction.next_modules.contains(name) {
                conjuction
                    .last_states
                    .insert(other_conjuction.name.clone(), Pulse::Low);
            }
        }
    }

    part1(&broadcast, &mut flip_flops, &mut conjuctions);
}

fn part1(
    broadcast: &Broadcast,
    flip_flops: &mut HashMap<String, FlipFlop>,
    conjuctions: &mut HashMap<String, Conjuction>,
) {
    let mut memory: HashMap<String, HashMap<String, Pulse>> = HashMap::new();
    for (conj_name, _) in conjuctions.iter() {
        let mut states = HashMap::new();
        for (name, _) in flip_flops.iter() {
            states.insert(name.clone(), Pulse::Low);
        }
        memory.insert(conj_name.clone(), states);
    }

    let mut queue: VecDeque<(String, Pulse, String)>;
    let mut high_pulses = 0;
    let mut low_pulses = 0; // button press

    for _ in 0..1000 {
        queue = broadcast
            .next_modules
            .iter()
            .map(|module| (String::from("broadcaster"), Pulse::Low, module.clone()))
            .collect();

        low_pulses += 1;
        while !queue.is_empty() {
            let (from_module, pulse, to_module) = queue.pop_front().unwrap();

            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            
            // println!("{} -{:?}-> {}", &from_module, &pulse, &to_module);

            if flip_flops.contains_key(&to_module) && pulse == Pulse::Low {
                let ff = flip_flops.get_mut(&to_module).unwrap();
                let next_pulse = match ff.state {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                ff.state = !ff.state;

                for next_module in &ff.next_modules {
                    queue.push_back((to_module.clone(), next_pulse.clone(), next_module.clone()));
                }
            } else if conjuctions.contains_key(&to_module) {
                let conjuction = conjuctions.get_mut(&to_module).unwrap();
                conjuction.update_state(&from_module, pulse);

                let next_pulse = conjuction.pulse();
                for next_module in &conjuction.next_modules {
                    queue.push_back((to_module.clone(), next_pulse.clone(), next_module.clone()));
                }
            }
        }
    }

    println!("Low/high pulses: {low_pulses}/{high_pulses}.");
    println!("Result part1: {}", high_pulses * low_pulses);
}
