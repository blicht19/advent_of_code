use std::env;

use lazy_static::lazy_static;
use library::get_lines;
use regex::{Regex, RegexSet};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     println!("Requires single filename as argument");
    // }

    // let lines = get_lines(args[1].as_str());
    // let mut sum = 0;

    // for line in lines {
    //     sum += get_number(line);
    // }

    // println!("Part 1: {}", sum);

    // let regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    // let numbers: Vec<&str> = regex.find_iter("sevenine").map(|m| m.as_str()).collect();
    // println!("{:?}", numbers);

    let patterns = ["seven", "nine", r"\d"];
    let set = RegexSet::new(patterns).unwrap();
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    let matches: Vec<&str> = set
        .matches("sevenine1")
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| re.find("sevenine1").unwrap().as_str())
        .collect();

    println!("{:?}", matches);
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
    lazy_static! {
        static ref SET: RegexSet = RegexSet::new([
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", r"\d"
        ])
        .unwrap();
        static ref REGEXES: Vec<Regex> = SET
            .patterns()
            .iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();
    }

    let line_str = line.as_str();

    let matches: Vec<&str> = SET
        .matches(line_str)
        .into_iter()
        .map(|index| &REGEXES[index])
        .map(|re| re.find(line_str).unwrap().as_str())
        .collect();
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
}
