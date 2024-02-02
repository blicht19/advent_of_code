use std::fs::File;
use std::io::{BufRead, BufReader};

// From this StackOverflow answer https://stackoverflow.com/a/35820003
pub fn get_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect()
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
}
