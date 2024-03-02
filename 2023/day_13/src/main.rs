use std::{cmp::min, fs::read_to_string};

use library::get_filename_arg;

fn main() {
    let file_name = get_filename_arg();
    let input: String = read_to_string(file_name).unwrap().parse().unwrap();
    let input: Vec<&str> = input.split("\n\n").collect();

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for block in input {
        let grid: Vec<String> = block
            .split("\n")
            .filter(|l| *l != "")
            .map(|l| String::from(l))
            .collect();

        part_one_total += get_reflection_point_part_one(&grid) * 100;
        part_two_total += get_reflection_point_part_two(&grid) * 100;

        let columns_grid = get_columns_grid(&grid);
        part_one_total += get_reflection_point_part_one(&columns_grid);
        part_two_total += get_reflection_point_part_two(&columns_grid);
    }

    println!("Part 1: {part_one_total}");
    println!("Part 1: {part_two_total}");
}

fn get_above_and_below(grid: &Vec<String>, index: usize) -> (Vec<&String>, Vec<&String>) {
    let above: Vec<&String> = grid[..index]
        .into_iter()
        .rev()
        .filter(|s| **s != "")
        .collect();
    let below: Vec<&String> = grid[index..].into_iter().filter(|s| **s != "").collect();

    let min_length = min(above.len(), below.len());
    let above = above[..min_length].to_vec();
    let below = below[..min_length].to_vec();
    (above, below)
}

fn get_reflection_point_part_one(grid: &Vec<String>) -> usize {
    for i in 1..grid.len() {
        let (above, below) = get_above_and_below(grid, i);
        if above == below {
            return i;
        }
    }

    0
}

fn get_reflection_point_part_two(grid: &Vec<String>) -> usize {
    for i in 1..grid.len() {
        let (above, below) = get_above_and_below(grid, i);

        let mut differences = 0;

        for (x, y) in above.iter().zip(below.iter()) {
            for (a, b) in x.chars().zip(y.chars()) {
                if a != b {
                    differences += 1;
                }
            }
        }

        if differences == 1 {
            return i;
        }
    }

    0
}

fn get_columns_grid(grid: &Vec<String>) -> Vec<String> {
    let char_grid: Vec<Vec<char>> = grid.iter().map(|l| l.chars().collect()).collect();
    let mut columns_vec: Vec<String> = vec![];
    for i in 0..char_grid[0].len() {
        let mut char_vec = vec![];
        for j in (0..char_grid.len()).rev() {
            char_vec.push(char_grid[j][i]);
        }
        let char_vec: String = char_vec.into_iter().collect();
        columns_vec.push(char_vec);
    }

    columns_vec
}
