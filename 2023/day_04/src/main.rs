use std::collections::HashSet;

use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_path = get_filename_arg();
    let lines = get_lines(file_path.as_str());

    println!("Part 1: {}", part_one_sum(&lines));
    println!("Part 2: {}", part_two_sum(&lines));
}

fn get_numbers_section(line: &String) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"  +").unwrap();
    }
    let numbers_section: &str = line.split(":").collect::<Vec<&str>>()[1];
    REGEX.replace_all(&numbers_section, " ").to_string()
}

fn get_number_vec(numbers: &str) -> Vec<u32> {
    let mut parsed_numbers: Vec<u32> = Vec::new();
    let number_strings: Vec<&str> = numbers
        .split(" ")
        .filter(|element| *element != "")
        .collect();

    for number_string in number_strings {
        parsed_numbers.push(
            number_string
                .parse()
                .expect("Failed to parse string to number"),
        );
    }

    parsed_numbers
}

fn get_match_count(line: &String) -> u32 {
    let numbers_section = get_numbers_section(&line);
    let sections: Vec<&str> = numbers_section.split("|").collect();
    let winning_numbers: HashSet<u32> = HashSet::from_iter(get_number_vec(sections[0]).into_iter());
    let player_numbers = get_number_vec(sections[1]);
    let mut matches = 0;

    for number in player_numbers {
        if winning_numbers.contains(&number) {
            matches += 1;
        }
    }

    matches
}

fn get_line_score(line: &String) -> u32 {
    let match_count = get_match_count(&line);
    if match_count == 0 {
        return 0;
    }

    let base: u32 = 2;
    base.pow(match_count - 1)
}

fn part_one_sum(lines: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        sum += get_line_score(&line);
    }

    sum
}

fn part_two_sum(lines: &Vec<String>) -> u32 {
    let mut copies_histogram = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let match_count = get_match_count(line);
        let copies = copies_histogram[i];

        let mut j = i + 1;
        while j <= i + match_count as usize && i < lines.len() {
            copies_histogram[j] += copies;
            j += 1;
        }
    }

    let mut sum = 0;

    for count in copies_histogram {
        sum += count;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_section() {
        let line = String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");

        assert_eq!(
            get_numbers_section(&line),
            " 1 21 53 59 44 | 69 82 63 72 16 21 14 1"
        )
    }

    #[test]
    fn test_get_number_vec() {
        let input = " 1 21 ";

        let numbers = get_number_vec(input);

        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 21);
    }

    #[test]
    fn test_get_line_score() {
        assert_eq!(
            get_line_score(&String::from(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            )),
            8
        );

        assert_eq!(
            get_line_score(&String::from("Card 3: 1 2 3 | 4 | 5 | 6")),
            0
        );
    }
}
