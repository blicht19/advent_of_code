use std::collections::HashSet;

use json::{object, JsonValue};
use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    // let file_name = String::from("resources/test_input.txt");
    let mut input = get_two_dimensional_vector(&file_name);
    let mut beams = HashSet::new();

    radiate_right(0, 0, &mut beams, &mut input);

    let regex: Regex = Regex::new(r"[#=?>!]").unwrap();
    let mut sum = 0;
    for line in input {
        let row: String = line.iter().collect();
        let matches: Vec<_> = regex.find_iter(&row).map(|m| m.as_str()).collect();
        if matches.len() > 0 {
            sum += matches.len();
        }
    }

    println!("{sum}");
}

enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<Directions> for JsonValue {
    fn from(value: Directions) -> Self {
        match value {
            Directions::UP => JsonValue::String(String::from("UP")),
            Directions::DOWN => JsonValue::String(String::from("DOWN")),
            Directions::LEFT => JsonValue::String(String::from("LEFT")),
            Directions::RIGHT => JsonValue::String(String::from("RIGHT")),
        }
    }
}

fn radiate_right(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    let beam = object! {
        row: row,
        column: column,
        direction: Directions::RIGHT
    };
    let key = json::stringify(beam);
    if beams.contains(&key) {
        return;
    } else {
        beams.insert(key);
    }

    for i in column..input[row].len() {
        match input[row][i] {
            '.' | '#' => input[row][i] = '#',
            '-' | '=' => input[row][i] = '=',
            '/' | '?' => {
                input[row][i] = '?';
                if row > 0 {
                    radiate_up(row - 1, i, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[row][i] = '>';
                if row < input.len() - 1 {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[row][i] = '!';
                if row > 0 {
                    radiate_up(row - 1, i, beams, input);
                }
                if row < input.len() - 1 {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            _ => panic!("Invalid character"),
        }
    }
}

fn radiate_left(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    let beam = object! {
        row: row,
        column: column,
        direction: Directions::LEFT
    };
    let key = json::stringify(beam);
    if beams.contains(&key) {
        return;
    } else {
        beams.insert(key);
    }

    for i in (0..=column).rev() {
        match input[row][i] {
            '.' | '#' => input[row][i] = '#',
            '-' | '=' => input[row][i] = '=',
            '/' | '?' => {
                input[row][i] = '?';
                if row < input.len() - 1 {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[row][i] = '>';
                if row > 0 {
                    radiate_up(row - 1, i, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[row][i] = '!';
                if row > 0 {
                    radiate_up(row - 1, i, beams, input);
                }
                if row < input.len() - 1 {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            _ => panic!("Invalid character"),
        }
    }
}

fn radiate_down(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    let beam = object! {
        row: row,
        column: column,
        direction: Directions::DOWN
    };
    let key = json::stringify(beam);
    if beams.contains(&key) {
        return;
    } else {
        beams.insert(key);
    }

    for i in row..input.len() {
        match input[i][column] {
            '.' | '#' => input[i][column] = '#',
            '-' | '=' => {
                input[i][column] = '=';
                if column > 0 {
                    radiate_left(i, column - 1, beams, input);
                }
                if column < input[row].len() - 1 {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '/' | '?' => {
                input[i][column] = '?';
                if column > 0 {
                    radiate_left(i, column - 1, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[i][column] = '>';
                if column < input[row].len() - 1 {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[i][column] = '!';
            }
            _ => panic!("Invalid character"),
        }
    }
}

fn radiate_up(row: usize, column: usize, beams: &mut HashSet<String>, input: &mut Vec<Vec<char>>) {
    let beam = object! {
        row: row,
        column: column,
        direction: Directions::UP
    };
    let key = json::stringify(beam);
    if beams.contains(&key) {
        return;
    } else {
        beams.insert(key);
    }

    for i in (0..=row).rev() {
        match input[i][column] {
            '.' | '#' => input[i][column] = '#',
            '-' | '=' => {
                input[i][column] = '=';
                if column > 0 {
                    radiate_left(i, column - 1, beams, input);
                }
                if column < input[row].len() - 1 {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '/' | '?' => {
                input[i][column] = '?';
                if column < input[row].len() - 1 {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[i][column] = '>';
                if column > 0 {
                    radiate_left(i, column - 1, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[i][column] = '!';
            }
            _ => panic!("Invalid character"),
        }
    }
}
