use std::i32;

use library::{get_filename_arg, get_lines};

fn main() {
    let file_name = get_filename_arg();
    let input = get_lines(&file_name);
    let mut lines = vec![];

    for line in input {
        lines.push(parse_numbers(line));
    }

    println!("Part 1: {}", sum(&lines, |f| get_next_value(f)));
    println!("Part 2: {}", sum(&lines, |f| get_previous_value(f)));
}

fn parse_numbers(input: String) -> Vec<i32> {
    let mut numbers = vec![];
    let line: Vec<&str> = input.split(" ").collect();
    for l in line {
        numbers.push(
            l.parse::<i32>()
                .expect("Failed to parse string into number"),
        );
    }

    numbers
}

fn get_differences(line: &Vec<i32>) -> Vec<i32> {
    let mut differences = vec![];
    for i in 0..line.len() - 1 {
        differences.push(line[i + 1] - line[i]);
    }

    differences
}

fn is_only_zeroes(diffs: &Vec<i32>) -> bool {
    for diff in diffs {
        if *diff != 0 {
            return false;
        }
    }
    true
}

fn get_diff_vecs(line: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut diff_vecs = vec![];
    let mut diffs = get_differences(line);

    while !is_only_zeroes(&diffs) {
        diff_vecs.push(diffs.clone());
        diffs = get_differences(&diffs);
    }

    diff_vecs
}

fn get_next_value(line: &Vec<i32>) -> i32 {
    let diff_vecs = get_diff_vecs(line);

    let mut next_value = 0;
    for i in 0..diff_vecs.len() {
        next_value += diff_vecs[i].last().unwrap();
    }
    next_value += line.last().unwrap();

    next_value
}

fn get_previous_value(line: &Vec<i32>) -> i32 {
    let diff_vecs = get_diff_vecs(line);

    let mut previous_value = 0;
    for i in (0..diff_vecs.len()).rev() {
        previous_value = diff_vecs[i][0] - previous_value;
    }

    line[0] - previous_value
}

fn sum<F>(lines: &Vec<Vec<i32>>, value_fn: F) -> i32
where
    F: Fn(&Vec<i32>) -> i32,
{
    let mut sum = 0;
    for line in lines {
        sum += value_fn(line);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let input = String::from("1 20 42");
        let lines = parse_numbers(input);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], 1);
        assert_eq!(lines[1], 20);
        assert_eq!(lines[2], 42);
    }

    #[test]
    fn test_get_differences() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let differences = get_differences(&input);
        let next_differences = get_differences(&differences);

        assert_eq!(differences.len(), input.len() - 1);
        assert_eq!(next_differences.len(), differences.len() - 1);

        for difference in differences {
            assert_eq!(difference, 3);
        }

        for next_difference in next_differences {
            assert_eq!(next_difference, 0);
        }
    }

    #[test]
    fn test_is_only_zeroes() {
        assert!(!is_only_zeroes(&vec![0, 1, 0, 0]));
        assert!(is_only_zeroes(&vec![0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_get_next_value() {
        assert_eq!(get_next_value(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(get_next_value(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(get_next_value(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_get_previous_value() {
        assert_eq!(get_previous_value(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(get_previous_value(&vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(get_previous_value(&vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
