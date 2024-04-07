use std::collections::{HashMap, VecDeque};

use library::{get_filename_arg, get_lines};

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);

    let (mut part_one_modules, destinations) = get_modules_and_destinations(&lines);
    println!("Part 1: {}", part_one(&mut part_one_modules, &destinations));

    let (mut part_two_modules, _) = get_modules_and_destinations(&lines);
    println!("Part 2: {}", part_two(&mut part_two_modules, &destinations));
}

fn get_modules_and_destinations(
    lines: &Vec<String>,
) -> (
    HashMap<String, Box<dyn Module>>,
    HashMap<String, Vec<String>>,
) {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut destinations: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let (source, destination) = line.split_once(" -> ").unwrap();
        let destination_names: Vec<String> =
            destination.split(", ").map(|s| String::from(s)).collect();
        let prefix = source.chars().collect::<Vec<char>>()[0];
        let mut name = source[1..source.len()].to_string();
        let module: Box<dyn Module> = match prefix {
            '%' => Box::new(FlipFlop::default()),
            '&' => Box::new(Conjunction::default()),
            _ => {
                name = String::from("broadcaster");
                Box::new(Broadcaster::default())
            }
        };
        modules.insert(name.clone(), module);
        destinations.insert(name, destination_names);
    }

    for (name, destination_names) in destinations.iter() {
        for destination_name in destination_names {
            modules
                .get_mut(destination_name)
                .map(|n| n.add_input(name.clone()));
        }
    }
    (modules, destinations)
}

fn part_one(
    modules: &mut HashMap<String, Box<dyn Module>>,
    destinations: &HashMap<String, Vec<String>>,
) -> u64 {
    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        low_count += 1;
        queue.push_back((String::from("broadcaster"), false));
        while queue.len() > 0 {
            let (name, pulse_value) = queue.pop_front().unwrap();
            for destination in destinations.get(&name).unwrap() {
                if pulse_value {
                    high_count += 1;
                } else {
                    low_count += 1;
                }

                let next_module = modules.get_mut(destination);

                if next_module.is_none() {
                    continue;
                }

                let next_module = next_module.unwrap();
                if !next_module.continue_pulsing(pulse_value) {
                    continue;
                }
                next_module.pulse(pulse_value, name.clone());
                queue.push_back((destination.to_string(), next_module.get_state()));
            }
        }
    }

    high_count * low_count
}

fn part_two(
    modules: &mut HashMap<String, Box<dyn Module>>,
    destinations: &HashMap<String, Vec<String>>,
) -> u64 {
    let mut i = 1;
    let mut cycle_counts = vec![];
    let mut inputs_to_inputs_to_rx = vec![
        String::from("pq"),
        String::from("fg"),
        String::from("dk"),
        String::from("fm"),
    ];

    while inputs_to_inputs_to_rx.len() > 0 {
        let mut queue = VecDeque::new();
        queue.push_back((String::from("broadcaster"), false));
        while queue.len() > 0 {
            let (name, pulse_value) = queue.pop_front().unwrap();
            for destination in destinations.get(&name).unwrap() {
                if inputs_to_inputs_to_rx.contains(destination) && !pulse_value {
                    inputs_to_inputs_to_rx.remove(
                        inputs_to_inputs_to_rx
                            .iter()
                            .position(|r| r == destination)
                            .unwrap(),
                    );
                    cycle_counts.push(i);
                }

                let next_module = modules.get_mut(destination);

                if next_module.is_none() {
                    continue;
                }

                let next_module = next_module.unwrap();
                if !next_module.continue_pulsing(pulse_value) {
                    continue;
                }
                next_module.pulse(pulse_value, name.clone());
                queue.push_back((destination.to_string(), next_module.get_state()));
            }
        }
        i += 1;
    }
    least_common_multiple(cycle_counts)
}

fn greatest_common_denominator(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_denominator(b, a % b)
    }
}

fn lcm_helper(a: u64, b: u64) -> u64 {
    a * b / greatest_common_denominator(a, b)
}

fn least_common_multiple(values: Vec<u64>) -> u64 {
    let mut multiple = values[0];

    for value in values {
        multiple = lcm_helper(multiple, value);
    }

    multiple
}

trait Module {
    fn pulse(&mut self, pulse_value: bool, from: String);
    fn get_state(&self) -> bool;
    fn add_input(&mut self, _: String) {}
    fn continue_pulsing(&self, _: bool) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug)]
struct FlipFlop {
    state: bool,
}

impl Module for FlipFlop {
    fn pulse(&mut self, pulse_value: bool, _: String) {
        if !pulse_value {
            self.state = !self.state;
        }
    }

    fn get_state(&self) -> bool {
        self.state
    }

    fn continue_pulsing(&self, pulse_value: bool) -> bool {
        if pulse_value {
            return false;
        }
        true
    }
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, bool>,
    state: bool,
}

impl Module for Conjunction {
    fn pulse(&mut self, pulse_value: bool, from: String) {
        self.inputs.insert(from, pulse_value);
        let mut new_pulse_value = false;
        for value in self.inputs.values() {
            if !value {
                new_pulse_value = true
            }
        }
        self.state = new_pulse_value;
    }

    fn get_state(&self) -> bool {
        self.state
    }

    fn add_input(&mut self, from: String) {
        self.inputs.insert(from, false);
    }
}

#[derive(Default, Clone, Debug)]
struct Broadcaster {
    state: bool,
}

impl Module for Broadcaster {
    fn pulse(&mut self, pulse_value: bool, _: String) {
        self.state = pulse_value;
    }

    fn get_state(&self) -> bool {
        self.state
    }
}
