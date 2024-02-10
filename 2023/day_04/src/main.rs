use std::collections::HashSet;

use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_path = get_filename_arg();
    let lines = get_lines(file_path.as_str());

    println!("Part 1: {}", part_one_sum(lines));
}

fn get_numbers_section(line: String) -> String {
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

fn get_line_score(line: String) -> u32 {
    let numbers_section = get_numbers_section(line);
    let sections: Vec<&str> = numbers_section.split("|").collect();
    let winning_numbers: HashSet<u32> = HashSet::from_iter(get_number_vec(sections[0]).into_iter());
    let player_numbers = get_number_vec(sections[1]);
    let mut score = 0;

    for number in player_numbers {
        if winning_numbers.contains(&number) {
            if score > 0 {
                score *= 2;
            } else {
                score = 1;
            }
        }
    }

    score
}

fn part_one_sum(lines: Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        sum += get_line_score(line);
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
            get_numbers_section(line),
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
            get_line_score(String::from(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            )),
            8
        );

        assert_eq!(get_line_score(String::from("Card 3: 1 2 3 | 4 | 5 | 6")), 0);
    }
}
