use lazy_static::lazy_static;
use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let input_vec = get_two_dimensional_vector(file_name.as_str());
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
}
