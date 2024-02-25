use lazy_static::lazy_static;
use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let input_vec = get_two_dimensional_vector(&file_name);

    let part_one_sum = part_one_sum(&input_vec);
    println!("Part 1: {}", part_one_sum);

    let part_two_sum = part_two_sum(&input_vec);
    println!("Part 2: {}", part_two_sum);
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

#[derive(Clone)]
struct Location {
    start: usize,
    end: usize,
}

fn get_number_locations_in_line(line: &Vec<char>) -> Vec<Location> {
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
        let number_locations = get_number_locations_in_line(&input_vec[line_number]);
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

#[derive(Clone)]
struct TwoDimensionalLocation {
    row_location: Location,
    y: usize,
}

fn get_number_locations(input_vec: &Vec<Vec<char>>) -> Vec<TwoDimensionalLocation> {
    let mut number_locations: Vec<TwoDimensionalLocation> = Vec::new();
    for i in 0..input_vec.len() {
        let locations = get_number_locations_in_line(&input_vec[i]);
        for location in locations {
            number_locations.push(TwoDimensionalLocation {
                y: i,
                row_location: location,
            })
        }
    }

    number_locations
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn get_asterisk_coordinates(input_vec: &Vec<Vec<char>>) -> Vec<Coordinate> {
    let mut locations: Vec<Coordinate> = Vec::new();
    for y in 0..input_vec.len() {
        for x in 0..input_vec[y].len() {
            if input_vec[y][x] == '*' {
                locations.push(Coordinate { x, y });
            }
        }
    }

    locations
}

fn get_adjacent_number_locations(
    coord: Coordinate,
    number_locations: &Vec<TwoDimensionalLocation>,
    input_vec: &Vec<Vec<char>>,
) -> Vec<TwoDimensionalLocation> {
    let mut adjacent_numbers: Vec<TwoDimensionalLocation> = Vec::new();

    if coord.y >= 1 {
        let mut above_numbers: Vec<TwoDimensionalLocation> = number_locations
            .iter()
            .filter(|location| {
                location.y == coord.y - 1
                    && ((location.row_location.start >= coord.x - 1
                        && location.row_location.start <= coord.x + 1)
                        || (location.row_location.end > coord.x - 1
                            && location.row_location.end < coord.x + 2))
            })
            .cloned()
            .collect();
        adjacent_numbers.append(&mut above_numbers);
    }

    if coord.x + 1 < input_vec[coord.y].len() {
        let mut right_numbers: Vec<TwoDimensionalLocation> = number_locations
            .iter()
            .filter(|location| location.y == coord.y && location.row_location.start == coord.x + 1)
            .cloned()
            .collect();
        adjacent_numbers.append(&mut right_numbers);
    }

    if coord.y + 1 < input_vec.len() {
        let mut below_numbers: Vec<TwoDimensionalLocation> = number_locations
            .iter()
            .filter(|location| {
                location.y == coord.y + 1
                    && ((location.row_location.start >= coord.x - 1
                        && location.row_location.start <= coord.x + 1)
                        || (location.row_location.end > coord.x - 1
                            && location.row_location.end < coord.x + 2))
            })
            .cloned()
            .collect();
        adjacent_numbers.append(&mut below_numbers);
    }

    if coord.x >= 1 {
        let mut left_numbers: Vec<TwoDimensionalLocation> = number_locations
            .iter()
            .filter(|location| location.y == coord.y && location.row_location.end == coord.x)
            .cloned()
            .collect();
        adjacent_numbers.append(&mut left_numbers);
    }

    adjacent_numbers
}

fn get_number_from_two_dimensional_location(
    input_vec: &Vec<Vec<char>>,
    location: &TwoDimensionalLocation,
) -> u32 {
    let line_string: String = input_vec[location.y].iter().collect();
    let number_string: String = line_string
        .chars()
        .skip(location.row_location.start)
        .take(location.row_location.end - location.row_location.start)
        .collect();

    number_string
        .parse::<u32>()
        .expect("Failed to parse number to string")
}

fn part_two_sum(input_vec: &Vec<Vec<char>>) -> u32 {
    let asterisks = get_asterisk_coordinates(&input_vec);
    let numbers = get_number_locations(&input_vec);

    let mut sum = 0;

    for asterisk in asterisks {
        let adjacent_numbers = get_adjacent_number_locations(asterisk, &numbers, input_vec);

        if adjacent_numbers.len() == 2 {
            sum += get_number_from_two_dimensional_location(&input_vec, &adjacent_numbers[0])
                * get_number_from_two_dimensional_location(&input_vec, &adjacent_numbers[1]);
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
    fn test_get_number_locations_in_line() {
        let line = vec!['4', '6', '7', '.', '.', '1', '1'];
        let number_locations = get_number_locations_in_line(&line);

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

    #[test]
    fn test_get_number_locations() {
        let first_line = vec!['4', '6', '7', '.', '.', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '2', '2', '.', '.', '.'];
        let input_vec = vec![first_line, second_line];
        let number_locations = get_number_locations(&input_vec);

        assert_eq!(number_locations.len(), 3);

        assert_eq!(number_locations[0].y, 0);
        assert_eq!(number_locations[0].row_location.start, 0);
        assert_eq!(number_locations[0].row_location.end, 3);

        assert_eq!(number_locations[1].y, 0);
        assert_eq!(number_locations[1].row_location.start, 5);
        assert_eq!(number_locations[1].row_location.end, 8);

        assert_eq!(number_locations[2].y, 1);
        assert_eq!(number_locations[2].row_location.start, 3);
        assert_eq!(number_locations[2].row_location.end, 5);
    }

    #[test]
    fn test_get_asterisk_coordinates() {
        let first_line = vec!['4', '6', '7', '.', '*', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '*', '.', '.', '.', '.'];
        let input_vec = vec![first_line, second_line];
        let coordinates = get_asterisk_coordinates(&input_vec);

        assert_eq!(coordinates.len(), 2);

        assert_eq!(coordinates[0].y, 0);
        assert_eq!(coordinates[0].x, 4);

        assert_eq!(coordinates[1].y, 1);
        assert_eq!(coordinates[1].x, 3);
    }

    #[test]
    fn test_get_adjacent_number_locations() {
        let first_line = vec!['4', '6', '7', '.', '*', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '*', '.', '.', '.', '.'];
        let third_line = vec!['.', '.', '3', '5', '.', '.', '.', '.'];
        let input_vec = vec![first_line, second_line, third_line];
        let number_locations = get_number_locations(&input_vec);

        let adjacent_numbers =
            get_adjacent_number_locations(Coordinate { x: 3, y: 1 }, &number_locations, &input_vec);

        assert_eq!(adjacent_numbers.len(), 2);

        assert_eq!(adjacent_numbers[0].y, 0);
        assert_eq!(adjacent_numbers[0].row_location.start, 0);
        assert_eq!(adjacent_numbers[0].row_location.end, 3);

        assert_eq!(adjacent_numbers[1].y, 2);
        assert_eq!(adjacent_numbers[1].row_location.start, 2);
        assert_eq!(adjacent_numbers[1].row_location.end, 4);
    }

    #[test]
    fn test_get_number_from_two_dimensional_location() {
        let first_line = vec!['4', '6', '7', '.', '*', '1', '1', '4'];
        let second_line = vec!['.', '.', '.', '*', '.', '.', '.', '.'];
        let input_vec = vec![first_line, second_line];
        let location = TwoDimensionalLocation {
            y: 0,
            row_location: Location { start: 0, end: 3 },
        };

        assert_eq!(
            get_number_from_two_dimensional_location(&input_vec, &location),
            467
        );
    }
}
