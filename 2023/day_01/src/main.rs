use std::u32;

use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();

    let lines = get_lines(file_name.as_str());
    let mut part_one_sum = 0;
    let mut part_two_sum = 0;

    for line in lines {
        part_one_sum += get_number(line.clone());
        let digits = get_digits(line);
        part_two_sum += get_line_number(digits);
    }

    println!("Part 1: {}", part_one_sum);
    println!("Part 2: {}", part_two_sum);
}

fn get_number(line: String) -> u32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\d").unwrap();
    }
    let numbers: Vec<&str> = REGEX.find_iter(line.as_str()).map(|m| m.as_str()).collect();

    let mut calibration = String::new();
    calibration.push_str(
        numbers
            .first()
            .expect("Failed to get first matching string"),
    );
    calibration.push_str(numbers.last().expect("Failed to get last matching string"));

    calibration
        .parse::<u32>()
        .expect("Failed to parse string to number")
}

fn get_digits(line: String) -> Vec<u32> {
    let line_formatted = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    line_formatted
        .chars()
        .filter_map(|character| character.to_digit(10))
        .collect()
}

fn get_line_number(line_digits: Vec<u32>) -> u32 {
    let mut num_string = line_digits.first().expect("Empty line").to_string();
    num_string.push_str(line_digits.last().expect("Empty line").to_string().as_str());

    num_string
        .parse::<u32>()
        .expect("Failed to parse string to number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let number = get_number(String::from("1abc2"));
        assert_eq!(number, 12);
    }

    #[test]
    fn test_get_number_multiple_digits() {
        let number = get_number(String::from("a1b2c3d4e55f"));
        assert_eq!(number, 15);
    }

    #[test]
    fn test_get_number_single_digit() {
        let number = get_number(String::from("treb7uchet"));
        assert_eq!(number, 77);
    }

    #[test]
    fn test_get_digits() {
        let digits = get_digits(String::from("two1nine"));
        assert_eq!(digits.len(), 3);
        assert_eq!(digits[0], 2);
        assert_eq!(digits[1], 1);
        assert_eq!(digits[2], 9);
    }

    #[test]
    fn test_get_digits_overlapping() {
        let digits = get_digits(String::from("sevenine"));
        assert_eq!(digits.len(), 2);
        assert_eq!(digits[0], 7);
        assert_eq!(digits[1], 9);
    }

    #[test]
    fn test_get_line_number() {
        let number = get_line_number(get_digits(String::from("two1nine")));
        assert_eq!(number, 29);
    }

    #[test]
    fn test_get_line_number_overlapping() {
        let number = get_line_number(get_digits(String::from("sevenine")));
        assert_eq!(number, 79);
    }

    #[test]
    fn test_get_line_number_one_digit() {
        let number = get_line_number(get_digits(String::from("four")));
        assert_eq!(number, 44);
    }
}
