use std::collections::HashMap;

use library::{get_filename_arg, get_two_dimensional_vector};

fn main() {
    let file_name = get_filename_arg();
    let mut input = get_two_dimensional_vector(&file_name);
    println!("Part 1: {}", part_one(&mut input));
    println!("Part 2: {}", part_two(&mut input));
}

// Maybe if I spent more time on this I could come up with cleaner solution,
// but I think the redundancy is better than have a bunch of repeated match statements
fn tilt_north(input: &mut Vec<Vec<char>>, row: usize, column: usize) {
    for i in (1..=row).rev() {
        if input[i - 1][column] != '.' {
            return;
        }
        input[i - 1][column] = 'O';
        input[i][column] = '.';
    }
}

fn tilt_south(input: &mut Vec<Vec<char>>, row: usize, column: usize) {
    for i in row..input.len() - 1 {
        if input[i + 1][column] != '.' {
            return;
        }
        input[i + 1][column] = 'O';
        input[i][column] = '.';
    }
}

fn tilt_east(input: &mut Vec<Vec<char>>, row: usize, column: usize) {
    for i in column..input[row].len() - 1 {
        if input[row][i + 1] != '.' {
            return;
        }
        input[row][i + 1] = 'O';
        input[row][i] = '.';
    }
}

fn tilt_west(input: &mut Vec<Vec<char>>, row: usize, column: usize) {
    for i in (1..=column).rev() {
        if input[row][i - 1] != '.' {
            return;
        }
        input[row][i - 1] = 'O';
        input[row][i] = '.';
    }
}

fn north_cycle(input: &mut Vec<Vec<char>>) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'O' {
                tilt_north(input, i, j);
            }
        }
    }
}

fn south_cycle(input: &mut Vec<Vec<char>>) {
    for i in (0..input.len()).rev() {
        for j in 0..input[i].len() {
            if input[i][j] == 'O' {
                tilt_south(input, i, j);
            }
        }
    }
}

fn east_cycle(input: &mut Vec<Vec<char>>) {
    for i in (0..input[0].len()).rev() {
        for j in 0..input.len() {
            if input[j][i] == 'O' {
                tilt_east(input, j, i);
            }
        }
    }
}

fn west_cycle(input: &mut Vec<Vec<char>>) {
    for i in 0..input[0].len() {
        for j in 0..input.len() {
            if input[j][i] == 'O' {
                tilt_west(input, j, i);
            }
        }
    }
}

fn cycle(input: &mut Vec<Vec<char>>) {
    north_cycle(input);
    west_cycle(input);
    south_cycle(input);
    east_cycle(input);
}

// It's probably possible to make this work with part 2 as well, but Rust tried to fight me the whole way
fn calculate_load(input: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for i in 0..input.len() {
        let rock_count = input[i]
            .iter()
            .filter(|x| **x == 'O')
            .collect::<Vec<&char>>()
            .len();
        sum += (input.len() - i) * rock_count;
    }

    sum
}

fn part_one(input: &mut Vec<Vec<char>>) -> usize {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'O' {
                tilt_north(input, i, j);
            }
        }
    }

    calculate_load(input)
}

// Initially part 2 by observing that 1000 cycles gets you the right answer. This more general soultion
// is based on kwshi's soluition
// See https://github.com/kwshi/advent-of-code/blob/main/python/2023/14.py

const NUM_CYCLES: usize = 1000000000;

fn part_two(input: &mut Vec<Vec<char>>) -> usize {
    let mut state_map: HashMap<String, usize> = HashMap::new();
    let mut start = 0;
    let mut length = 0;

    for i in 1..NUM_CYCLES {
        cycle(input);
        let current_input: String = input.iter().map(|l| l.iter().collect::<String>()).collect();
        if state_map.contains_key(&current_input) {
            start = *state_map.get(&current_input).unwrap();
            length = i - start;
            break;
        } else {
            state_map.insert(current_input, i);
        }
    }

    let current_lines: Vec<char> = state_map
        .keys()
        .find(|key| *state_map.get(*key).unwrap() == start + ((NUM_CYCLES - start) % length))
        .unwrap()
        .chars()
        .collect();

    let current_lines: Vec<&[char]> = current_lines.chunks(input[0].len()).collect();

    let mut sum = 0;
    for i in 0..current_lines.len() {
        let rock_count = current_lines[i]
            .iter()
            .filter(|x| **x == 'O')
            .collect::<Vec<&char>>()
            .len();
        sum += (input.len() - i) * rock_count;
    }

    sum
}
