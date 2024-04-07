use std::collections::{HashMap, VecDeque};

use library::{get_filename_arg, get_lines};

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);

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

    println!("Part 1: {}", part_one(&mut modules, &destinations)); // Part 2 done by hand :)
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
