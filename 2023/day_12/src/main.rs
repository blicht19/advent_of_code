use library::get_lines;
use std::collections::HashMap;
use substring::Substring;

/* This is adapted from a solution by HyperNeutrino https://youtu.be/g3Ms5e7Jdqo?si=FnFBRIBU1ZYgyNc1 */
fn main() {
    let file_name = String::from("resources/input.txt");
    let input = get_lines(&file_name);
    let mut cache: HashMap<String, u64> = HashMap::new();

    let mut part_one_sum = 0;
    let mut part_two_sum = 0;

    for line in input {
        let substrings = line.split(" ").collect::<Vec<&str>>();
        let mut configuration = substrings[0].to_string();
        let part_one_numbers = substrings[1]
            .split(",")
            .map(|n| {
                n.parse::<usize>()
                    .expect("Failed to parse string into number")
            })
            .collect::<Vec<usize>>();
        part_one_sum += count(&configuration, &part_one_numbers, &mut cache);

        configuration.push_str("?");
        configuration = configuration.repeat(5);
        configuration.pop();
        let mut numbers_section = String::from(substrings[1]);
        numbers_section.push_str(",");
        numbers_section = numbers_section.repeat(5);
        numbers_section.pop();
        let part_two_numbers = numbers_section
            .split(",")
            .map(|n| {
                n.parse::<usize>()
                    .expect("Failed to parse string into number")
            })
            .collect::<Vec<usize>>();
        part_two_sum += count(&configuration, &part_two_numbers, &mut cache);
    }

    println!("Part 1: {}", part_one_sum);
    println!("Part 2: {}", part_two_sum);
}

fn count(configuration: &String, numbers: &Vec<usize>, cache: &mut HashMap<String, u64>) -> u64 {
    if configuration == "" {
        return if numbers.len() == 0 { 1 } else { 0 };
    }
    if numbers.len() == 0 {
        return if configuration.contains("#") { 0 } else { 1 };
    }

    let mut key = configuration.clone();
    key.push_str(
        &numbers
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(","),
    );

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;
    let config_chars: Vec<char> = configuration.chars().collect();

    if config_chars[0] == '.' || config_chars[0] == '?' {
        result += count(
            &String::from(configuration.substring(1, configuration.len())),
            numbers,
            cache,
        );
    }

    if (config_chars[0] == '#' || config_chars[0] == '?')
        && numbers[0] <= configuration.len()
        && !configuration.substring(0, numbers[0]).contains('.')
        && (numbers[0] == configuration.len() || config_chars[numbers[0]] != '#')
    {
        result += count(
            &String::from(configuration.substring(numbers[0] + 1, configuration.len())),
            &Vec::from(&numbers[1..numbers.len()]),
            cache,
        );
    }

    cache.insert(key, result);

    result
}
