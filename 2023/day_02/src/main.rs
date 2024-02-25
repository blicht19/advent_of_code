use lazy_static::lazy_static;
use library::{get_filename_arg, get_lines};
use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Counts {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);

    let mut part_one_sum: u32 = 0;
    let mut part_two_sum: u32 = 0;
    for line in lines {
        part_one_sum += get_id_value(line.clone());
        part_two_sum += get_power(line);
    }
    println!("Part one: {}", part_one_sum);
    println!("Part two: {}", part_two_sum);
}

fn get_number(numeric_string: &str) -> u32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\d{1,3}").unwrap();
    }
    let first_match = REGEX
        .find(&numeric_string)
        .map(|x| x.as_str())
        .expect("Failed to find game number");

    first_match
        .parse::<u32>()
        .expect("Failed to parse game number")
}

fn get_color_counts(counts_string: &str) -> Counts {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\d{1,2} (red|green|blue)").unwrap();
    }

    let mut counts = Counts {
        red: 0,
        green: 0,
        blue: 0,
    };
    let count_matches: Vec<&str> = REGEX
        .find_iter(&counts_string)
        .map(|m| m.as_str())
        .collect();

    for count_match in count_matches {
        let count = get_number(count_match);

        if count_match.contains("red") {
            counts.red = count;
        } else if count_match.contains("green") {
            counts.green = count;
        } else if count_match.contains("blue") {
            counts.blue = count;
        }
    }

    counts
}

fn game_is_valid(game: &str) -> bool {
    let substrings = game.split(";").collect::<Vec<&str>>();

    for substring in substrings {
        let counts = get_color_counts(substring);
        if counts.red > MAX_RED || counts.green > MAX_GREEN || counts.blue > MAX_BLUE {
            return false;
        }
    }

    true
}

fn get_id_value(line: String) -> u32 {
    let substrings = line.split(":").collect::<Vec<&str>>();
    if game_is_valid(substrings[1]) {
        get_number(substrings[0])
    } else {
        0
    }
}

fn get_power(line: String) -> u32 {
    let substrings = line.split(";").collect::<Vec<&str>>();
    let mut counts = vec![];
    for substring in substrings {
        counts.push(get_color_counts(substring));
    }
    let red = counts
        .iter()
        .map(|count| count.red)
        .max()
        .expect("Failed to get max red value");
    let green = counts
        .iter()
        .map(|count| count.green)
        .max()
        .expect("Failed to get max green value");
    let blue = counts
        .iter()
        .map(|count| count.blue)
        .max()
        .expect("Failed to get max blue value");

    red * green * blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_number() {
        let game_number = get_number("Game 1");
        assert_eq!(game_number, 1);
    }

    #[test]
    fn test_get_game_number_three_digits() {
        let game_number = get_number("100 blue");
        assert_eq!(game_number, 100);
    }

    #[test]
    fn test_get_color_counts() {
        let color_counts = get_color_counts("1 red, 2 green, 6 blue");
        assert_eq!(color_counts.red, 1);
        assert_eq!(color_counts.green, 2);
        assert_eq!(color_counts.blue, 6);
    }

    #[test]
    fn test_get_color_counts_partial() {
        let color_counts = get_color_counts("1 red, 2 green");
        assert_eq!(color_counts.red, 1);
        assert_eq!(color_counts.green, 2);
        assert_eq!(color_counts.blue, 0);
    }

    #[test]
    fn test_game_is_valid() {
        assert!(game_is_valid(
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        ));
        assert!(!game_is_valid(
            "15 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        ));
        assert!(!game_is_valid(
            "3 blue, 13 red; 1 red, 2 green, 6 blue; 2 green"
        ));
        assert!(!game_is_valid(
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 14 green"
        ));
    }

    #[test]
    fn test_get_id_value() {
        assert_eq!(
            get_id_value(String::from(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            5
        );
        assert_eq!(
            get_id_value(String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )),
            0
        );
    }

    #[test]
    fn test_get_power() {
        assert_eq!(
            get_power(String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )),
            1560
        );
    }
}
