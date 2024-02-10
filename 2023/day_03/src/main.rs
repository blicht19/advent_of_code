use lazy_static::lazy_static;
use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let input_vec = get_two_dimensional_vector(file_name.as_str());

    let part_one_sum = part_one_sum(&input_vec);

    println!("Part 1: {}", part_one_sum);
}

fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
}

/* isValidPosition and getAdjacentElements based on example from from https://www.geeksforgeeks.org/find-all-adjacent-elements-of-given-element-in-a-2d-array-or-matrix/ */
fn is_valid_position(y: i32, x: i32, y_length: i32, x_length: i32) -> bool {
    y >= 0 && x >= 0 && y < y_length && x < x_length
}

fn get_adjacent_elements(characters: &Vec<Vec<char>>, y: i32, x: i32) -> Vec<char> {
    let mut adjacent: Vec<char> = vec![];
    let y_length = characters.len() as i32;
    let x_length = characters.first().expect("empty 2d array").len() as i32;

    if is_valid_position(y - 1, x - 1, y_length, x_length) {
        adjacent.push(characters[y as usize - 1][x as usize - 1]);
    }
    if is_valid_position(y, x - 1, y_length, x_length) {
        adjacent.push(characters[y as usize][x as usize - 1]);
    }
    if is_valid_position(y, x + 1, y_length, x_length) {
        adjacent.push(characters[y as usize][x as usize + 1]);
    }
    if is_valid_position(y + 1, x, y_length, x_length) {
        adjacent.push(characters[y as usize + 1][x as usize]);
    }
    if is_valid_position(y + 1, x + 1, y_length, x_length) {
        adjacent.push(characters[y as usize + 1][x as usize + 1]);
    }
    if is_valid_position(y + 1, x - 1, y_length, x_length) {
        adjacent.push(characters[y as usize + 1][x as usize - 1]);
    }
    if is_valid_position(y - 1, x + 1, y_length, x_length) {
        adjacent.push(characters[y as usize - 1][x as usize + 1]);
    }
    adjacent
}

fn is_part_number_digit(input_vec: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    let adjacent_elements = get_adjacent_elements(input_vec, y, x);

    for character in adjacent_elements {
        if is_symbol(character) {
            return true;
        }
    }
    false
}

struct Location {
    start: usize,
    end: usize,
}

fn get_number_locations(line: &Vec<char>) -> Vec<Location> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\d+").unwrap();
    }

    let line_str: String = line.iter().collect();
    let mut locations: Vec<Location> = Vec::new();

    for field in REGEX.find_iter(&line_str) {
        locations.push(Location {
            start: field.start(),
            end: field.end(),
        });
    }

    locations
}

fn is_valid_part_number(
    number_location: &Location,
    line_number: usize,
    input_vec: &Vec<Vec<char>>,
) -> bool {
    for i in number_location.start..number_location.end {
        if is_part_number_digit(input_vec, line_number as i32, i as i32) {
            return true;
        }
    }

    false
}

fn part_one_sum(input_vec: &Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;
    for line_number in 0..input_vec.len() {
        let number_locations = get_number_locations(&input_vec[line_number]);
        for location in number_locations {
            if is_valid_part_number(&location, line_number, &input_vec) {
                let line_string: String = input_vec[line_number].iter().collect();
                let number_string: String = line_string
                    .chars()
                    .skip(location.start)
                    .take(location.end - location.start)
                    .collect();

                sum += number_string
                    .parse::<u32>()
                    .expect("Failed to parse number to string");
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('+'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('9'));
    }

    #[test]
    fn test_is_valid_position() {
        assert!(is_valid_position(0, 0, 5, 5));
        assert!(!is_valid_position(-1, 0, 5, 5));
        assert!(!is_valid_position(0, -1, 5, 5));
        assert!(!is_valid_position(5, 0, 5, 5));
        assert!(!is_valid_position(0, 5, 5, 5));
    }

    #[test]
    fn test_get_adjacent_elements() {
        let mut char_vec: Vec<Vec<char>> = vec![];
        char_vec.push(vec!['a', 'b']);
        char_vec.push(vec!['f', 'g']);

        let adjacent_elements = get_adjacent_elements(&char_vec, 0, 0);

        assert_eq!(adjacent_elements.len(), 3);
        assert!(adjacent_elements.contains(&'b'));
        assert!(adjacent_elements.contains(&'g'));
        assert!(adjacent_elements.contains(&'f'));
    }

    #[test]
    fn test_is_part_number_digit() {
        let first_line = vec!['4', '6', '7', '.', '.', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '*', '.', '.', '.', '.'];
        let input_vec = vec![first_line, second_line];

        assert!(is_part_number_digit(&input_vec, 0, 2));
        assert!(!is_part_number_digit(&input_vec, 0, 5));
    }

    #[test]
    fn test_get_number_locations() {
        let line = vec!['4', '6', '7', '.', '.', '1', '1'];
        let number_locations = get_number_locations(&line);

        assert_eq!(number_locations.len(), 2);
        assert_eq!(number_locations[0].start, 0);
        assert_eq!(number_locations[0].end, 3);
        assert_eq!(number_locations[1].start, 5);
        assert_eq!(number_locations[1].end, 7);
    }

    #[test]
    fn test_is_valid_part_number() {
        let first_line = vec!['4', '6', '7', '.', '.', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '*', '.', '.', '.', '.'];
        let input_vec = vec![first_line, second_line];

        assert!(is_valid_part_number(
            &Location { start: 0, end: 3 },
            0,
            &input_vec
        ));
        assert!(!is_valid_part_number(
            &Location { start: 5, end: 8 },
            0,
            &input_vec
        ));
    }
}
