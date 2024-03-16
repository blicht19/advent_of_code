use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, process::exit};

// From this StackOverflow answer https://stackoverflow.com/a/35820003
pub fn get_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect()
}

pub fn get_two_dimensional_vector(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let lines: Vec<String> = buf_reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect();

    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn get_two_dimensional_number_vector(file_path: &str) -> Vec<Vec<u32>> {
    let file = File::open(file_path).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let lines: Vec<String> = buf_reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect();

    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|character| {
                    character
                        .to_digit(10)
                        .expect("Failed to parse character into digit")
                })
                .collect()
        })
        .collect()
}

pub fn get_filename_arg() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Requires single filename as argument");
        exit(1);
    }
    args[1].clone()
}

#[derive(PartialEq, Clone, Copy)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_rep = match self {
            Self::UP => "UP",
            Self::DOWN => "DOWN",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
        };
        write!(f, "{string_rep}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines() {
        let file_path = "resources/test/get_lines_test_input.txt";
        let lines = get_lines(file_path);

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "1abc2");
        assert_eq!(lines[1], "pqr3stu8vwx");
        assert_eq!(lines[2], "a1b2c3d4e5f");
        assert_eq!(lines[3], "treb7uchet");
    }

    #[test]
    fn test_get_two_dimensional_vector() {
        let file_path = "resources/test/get_lines_test_input.txt";
        let two_dimension_vec = get_two_dimensional_vector(file_path);

        assert_eq!(two_dimension_vec.len(), 4);
        let first_line: String = two_dimension_vec[0].iter().collect();
        assert_eq!(first_line, "1abc2");
    }

    #[test]
    fn test_get_two_dimensional_number_vector() {
        let file_path = "resources/test/get_two_dimensional_number_vector_test_input.txt";
        let expected = vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]];
        let two_dimension_vec = get_two_dimensional_number_vector(file_path);

        assert_eq!(two_dimension_vec, expected);
    }
}
