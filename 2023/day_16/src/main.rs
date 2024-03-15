use core::fmt;
use std::collections::HashSet;

use lazy_static::lazy_static;
use library::{get_filename_arg, get_two_dimensional_vector};
use regex::Regex;

fn main() {
    let file_name = get_filename_arg();
    let input = get_two_dimensional_vector(&file_name);

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    let mut input = input.clone();
    let mut beams = HashSet::new();
    radiate_right(0, 0, &mut beams, &mut input);
    get_sum(&input)
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    let mut sums = vec![];

    sums.push(get_energized(0, 0, &input, Directions::DOWN));
    sums.push(get_energized(0, 0, &input, Directions::RIGHT));

    sums.push(get_energized(
        0,
        input[0].len() - 1,
        &input,
        Directions::DOWN,
    ));
    sums.push(get_energized(
        0,
        input[0].len() - 1,
        &input,
        Directions::LEFT,
    ));

    sums.push(get_energized(input.len() - 1, 0, &input, Directions::UP));
    sums.push(get_energized(input.len() - 1, 0, &input, Directions::RIGHT));

    sums.push(get_energized(
        input.len() - 1,
        input[0].len() - 1,
        &input,
        Directions::UP,
    ));
    sums.push(get_energized(
        input.len() - 1,
        input[0].len() - 1,
        &input,
        Directions::LEFT,
    ));

    for i in 1..input.len() - 1 {
        sums.push(get_energized(i, 0, &input, Directions::RIGHT));
        sums.push(get_energized(
            i,
            input[i].len() - 1,
            &input,
            Directions::LEFT,
        ));
    }

    for i in 1..input[0].len() - 1 {
        sums.push(get_energized(0, i, &input, Directions::DOWN));
        sums.push(get_energized(input.len() - 1, i, &input, Directions::UP));
    }

    sums.into_iter().max().unwrap()
}

fn get_sum(input: &Vec<Vec<char>>) -> usize {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"[#=?>!]").unwrap();
    }

    let mut sum = 0;
    for line in input {
        let row: String = line.iter().collect();
        let matches: Vec<_> = PATTERN.find_iter(&row).map(|m| m.as_str()).collect();
        if matches.len() > 0 {
            sum += matches.len();
        }
    }

    sum
}

enum Directions {
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

fn beam_string_format(row: usize, column: usize, direction: Directions) -> String {
    format!("{row},{column},{direction}")
}

fn beams_contains(
    row: usize,
    column: usize,
    direction: Directions,
    beams: &mut HashSet<String>,
) -> bool {
    let beam = beam_string_format(row, column, direction);
    if beams.contains(&beam) {
        true
    } else {
        beams.insert(beam);
        false
    }
}

fn can_increment_row(row: usize, input: &Vec<Vec<char>>) -> bool {
    row < input.len() - 1
}

fn can_increment_column(row: usize, column: usize, input: &Vec<Vec<char>>) -> bool {
    column < input[row].len() - 1
}

fn can_decrement(row_or_column: usize) -> bool {
    row_or_column > 0
}

fn radiate_right(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    if beams_contains(row, column, Directions::RIGHT, beams) {
        return;
    }

    for i in column..input[row].len() {
        match input[row][i] {
            '.' | '#' => input[row][i] = '#',
            '-' | '=' => input[row][i] = '=',
            '/' | '?' => {
                input[row][i] = '?';
                if can_decrement(row) {
                    radiate_up(row - 1, i, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[row][i] = '>';
                if can_increment_row(row, input) {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[row][i] = '!';
                if can_decrement(row) {
                    radiate_up(row - 1, i, beams, input);
                }
                if can_increment_row(row, input) {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            invalid_char => panic!("Invalid character {invalid_char}"),
        }
    }
}

fn radiate_left(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    if beams_contains(row, column, Directions::LEFT, beams) {
        return;
    }

    for i in (0..=column).rev() {
        match input[row][i] {
            '.' | '#' => input[row][i] = '#',
            '-' | '=' => input[row][i] = '=',
            '/' | '?' => {
                input[row][i] = '?';
                if can_increment_row(row, input) {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[row][i] = '>';
                if can_decrement(row) {
                    radiate_up(row - 1, i, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[row][i] = '!';
                if can_decrement(row) {
                    radiate_up(row - 1, i, beams, input);
                }
                if can_increment_row(row, input) {
                    radiate_down(row + 1, i, beams, input);
                }
                return;
            }
            invalid_char => panic!("Invalid character {invalid_char}"),
        }
    }
}

fn radiate_down(
    row: usize,
    column: usize,
    beams: &mut HashSet<String>,
    input: &mut Vec<Vec<char>>,
) {
    if beams_contains(row, column, Directions::DOWN, beams) {
        return;
    }

    for i in row..input.len() {
        match input[i][column] {
            '.' | '#' => input[i][column] = '#',
            '-' | '=' => {
                input[i][column] = '=';
                if can_decrement(column) {
                    radiate_left(i, column - 1, beams, input);
                }
                if can_increment_column(row, column, input) {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '/' | '?' => {
                input[i][column] = '?';
                if can_decrement(column) {
                    radiate_left(i, column - 1, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[i][column] = '>';
                if can_increment_column(row, column, input) {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[i][column] = '!';
            }
            invalid_char => panic!("Invalid character {invalid_char}"),
        }
    }
}

fn radiate_up(row: usize, column: usize, beams: &mut HashSet<String>, input: &mut Vec<Vec<char>>) {
    if beams_contains(row, column, Directions::UP, beams) {
        return;
    }

    for i in (0..=row).rev() {
        match input[i][column] {
            '.' | '#' => input[i][column] = '#',
            '-' | '=' => {
                input[i][column] = '=';
                if can_decrement(column) {
                    radiate_left(i, column - 1, beams, input);
                }
                if can_increment_column(row, column, input) {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '/' | '?' => {
                input[i][column] = '?';
                if can_increment_column(row, column, input) {
                    radiate_right(i, column + 1, beams, input);
                }
                return;
            }
            '\\' | '>' => {
                input[i][column] = '>';
                if can_decrement(column) {
                    radiate_left(i, column - 1, beams, input);
                }
                return;
            }
            '|' | '!' => {
                input[i][column] = '!';
            }
            invalid_char => panic!("Invalid character {invalid_char}"),
        }
    }
}

fn get_energized(
    row: usize,
    column: usize,
    input: &Vec<Vec<char>>,
    direction: Directions,
) -> usize {
    let mut input = input.clone();
    let mut beams = HashSet::new();

    match direction {
        Directions::UP => radiate_up(row, column, &mut beams, &mut input),
        Directions::DOWN => radiate_down(row, column, &mut beams, &mut input),
        Directions::LEFT => radiate_left(row, column, &mut beams, &mut input),
        Directions::RIGHT => radiate_right(row, column, &mut beams, &mut input),
    };

    get_sum(&input)
}
