use std::{collections::HashMap, u64::MAX};

use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let mut lines = get_lines(file_name.as_str());
    lines.retain(|f| !f.is_empty());

    let mut maps: Vec<HashMap<Range, Range>> = vec![];
    let mut current_map: HashMap<Range, Range> = HashMap::new();
    let line_header_regex = Regex::new(r".*-to.* map:").unwrap();
    for i in 2..lines.len() {
        if line_header_regex.is_match(lines[i].as_str()) || lines[i].is_empty() {
            maps.push(current_map);
            current_map = HashMap::new();
        } else {
            let numbers = get_numbers_from_strings(&split_line(&lines[i]));
            current_map.insert(
                Range {
                    start: numbers[1],
                    end: numbers[1] + numbers[2],
                },
                Range {
                    start: numbers[0],
                    end: numbers[0] + numbers[2],
                },
            );
        }
    }
    maps.push(current_map);

    let part_one_seeds = get_part_one_seeds(&lines[0]);
    let part_two_seeds = get_part_two_seeds(&lines[0]);

    let part_one = get_min_location_part_one(part_one_seeds, &maps);
    println!("Part 1: {}", part_one);

    let part_two = get_min_location_part_two(part_two_seeds, &maps);
    println!("Part 2: {}", part_two);
}

fn get_numbers_from_strings(strings: &Vec<String>) -> Vec<u64> {
    let mut numbers: Vec<u64> = vec![];

    for item in strings {
        numbers.push(item.parse().expect("Failed to parse into number"));
    }

    numbers
}

fn split_line(line: &String) -> Vec<String> {
    line.split(" ").map(|s| s.to_string()).collect()
}

fn get_part_one_seeds(seeds_line: &String) -> Vec<u64> {
    let mut number_strings: Vec<String> = split_line(&seeds_line);
    number_strings.remove(0);
    get_numbers_from_strings(&number_strings)
}

fn get_part_two_seeds(seeds_line: &String) -> Vec<Range> {
    let seed_numbers = get_part_one_seeds(seeds_line);
    let mut seeds: Vec<Range> = vec![];

    for (index, seed) in seed_numbers.iter().enumerate() {
        if index % 2 == 0 {
            seeds.push(Range {
                start: *seed,
                end: seed + seed_numbers[index + 1],
            });
        }
    }

    seeds
}

#[derive(Hash, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

fn get_mapped_value(value: u64, map: &HashMap<Range, Range>) -> u64 {
    let map_keys = map.keys();

    for map_key in map_keys {
        if value >= map_key.start && value < map_key.end {
            let difference = value - map_key.start;
            return map.get(map_key).expect("No value found for key").start + difference;
        }
    }

    value
}

fn get_min_location_part_one(seeds: Vec<u64>, maps: &Vec<HashMap<Range, Range>>) -> u64 {
    let mut min_value: u64 = MAX;

    for seed in seeds {
        let mut value = seed;

        for map in maps.iter() {
            value = get_mapped_value(value, map);
        }

        if value < min_value {
            min_value = value;
        }
    }

    min_value
}

fn get_min_location_part_two(seeds: Vec<Range>, maps: &Vec<HashMap<Range, Range>>) -> u64 {
    let mut min_value: u64 = MAX;

    for seed in seeds {
        for i in seed.start..seed.end {
            let mut value = i;
            for map in maps.iter() {
                value = get_mapped_value(value, map);
            }

            if value < min_value {
                min_value = value;
            }
        }
    }

    min_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_strings() {
        let strings = vec![String::from("3"), String::from("4"), String::from("5")];
        let numbers = get_numbers_from_strings(&strings);

        assert_eq!(numbers.len(), strings.len());
        assert_eq!(numbers[0], 3);
        assert_eq!(numbers[1], 4);
        assert_eq!(numbers[2], 5);
    }

    #[test]
    fn test_split_line() {
        let split_up_line = split_line(&String::from("seeds: 79 14 55 13"));

        assert_eq!(split_up_line.len(), 5);
        assert_eq!(split_up_line[0], "seeds:");
        assert_eq!(split_up_line[1], "79");
        assert_eq!(split_up_line[2], "14");
        assert_eq!(split_up_line[3], "55");
        assert_eq!(split_up_line[4], "13");
    }

    #[test]
    fn test_get_part_one_seeds() {
        let part_one_seeds = get_part_one_seeds(&String::from("seeds: 79 14 55 13"));

        assert_eq!(part_one_seeds.len(), 4);
        assert_eq!(part_one_seeds[0], 79);
        assert_eq!(part_one_seeds[1], 14);
        assert_eq!(part_one_seeds[2], 55);
        assert_eq!(part_one_seeds[3], 13);
    }

    #[test]
    fn test_get_mapped_value() {
        let map = HashMap::from([(Range { start: 1, end: 3 }, Range { start: 20, end: 22 })]);

        assert_eq!(get_mapped_value(0, &map), 0);
        assert_eq!(get_mapped_value(1, &map), 20);
        assert_eq!(get_mapped_value(2, &map), 21);
        assert_eq!(get_mapped_value(3, &map), 3);
    }

    #[test]
    fn test_get_part_two_seeds() {
        let part_two_seeds = get_part_two_seeds(&String::from("seeds: 79 14 55 13"));

        assert_eq!(part_two_seeds.len(), 2);
        assert_eq!(part_two_seeds[0].start, 79);
        assert_eq!(part_two_seeds[0].end, 93);
        assert_eq!(part_two_seeds[1].start, 55);
        assert_eq!(part_two_seeds[1].end, 68);
    }
}
