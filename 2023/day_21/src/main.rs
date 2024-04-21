use std::collections::{HashSet, VecDeque};

use library::{get_filename_arg, get_two_dimensional_vector};

fn main() {
    let file_name = get_filename_arg();
    let input = get_two_dimensional_vector(&file_name);

    let mut s_location = None;
    for i in 0..input.len() {
        let s_column = input[i].iter().position(|&j| j == 'S');
        if s_column.is_some() {
            s_location = Some((i, s_column.unwrap()));
            break;
        }
    }

    let s_location = s_location.expect("No starting position found");

    println!("Part 1: {}", part_one(&input, s_location));
    println!("Part 2: {}", part_two(&input, s_location));
}

fn get_adjacent_locations(
    coordinate: (usize, usize),
    input: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let mut adjacent = vec![];
    let (row, column) = coordinate;

    if row > 0 && input[row - 1][column] != '#' {
        adjacent.push((row - 1, column));
    }
    if column > 0 && input[row][column - 1] != '#' {
        adjacent.push((row, column - 1));
    }
    if row < input.len() - 1 && input[row + 1][column] != '#' {
        adjacent.push((row + 1, column));
    }
    if column < input[0].len() - 1 && input[row][column + 1] != '#' {
        adjacent.push((row, column + 1));
    }

    adjacent
}

fn part_one(input: &Vec<Vec<char>>, start_location: (usize, usize)) -> usize {
    let steps = 64;
    let mut current_locations = HashSet::new();
    current_locations.insert(start_location);

    for _i in 0..steps {
        let locations = current_locations.clone();
        current_locations = HashSet::new();

        for location in locations {
            let adjacent = get_adjacent_locations(location, &input);
            for adjacent_location in adjacent {
                current_locations.insert(adjacent_location);
            }
        }
    }

    current_locations.len()
}

// I was stumped on part 2. Thanks to HyperNeutrino for the explanation https://youtu.be/9UOMZSL0JTg?si=DBJz-S_An1uKAtY1

fn fill(start_location: (usize, usize), steps: usize, input: &Vec<Vec<char>>) -> usize {
    let mut answers = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert(start_location);
    let mut queue = VecDeque::new();
    queue.push_back((start_location.0, start_location.1, steps));

    while !queue.is_empty() {
        let (row, column, steps) = queue.pop_front().unwrap();

        if steps % 2 == 0 {
            answers.insert((row, column));
        }

        if steps == 0 {
            continue;
        }

        'inner: for (next_row, next_column) in get_adjacent_locations((row, column), input) {
            if seen.contains(&(next_row, next_column)) {
                continue 'inner;
            }

            seen.insert((next_row, next_column));
            queue.push_back((next_row, next_column, steps - 1));
        }
    }

    answers.len()
}

fn part_two(input: &Vec<Vec<char>>, start_location: (usize, usize)) -> usize {
    let size = input.len();
    let steps = 26501365;
    let grid_width = (steps / size) - 1;

    let odd_grids = ((grid_width / 2) * 2 + 1).pow(2);
    let even_grids = (((grid_width + 1) / 2) * 2).pow(2);

    let top_corner = fill((size - 1, start_location.1), size - 1, input);
    let right_corner = fill((start_location.0, 0), size - 1, input);
    let bottom_corner = fill((0, start_location.1), size - 1, input);
    let left_corner = fill((start_location.0, size - 1), size - 1, input);

    let floor_divide = (size / 2) - 1;
    let small_top_right = fill((size - 1, 0), floor_divide, input);
    let small_bottom_right = fill((0, 0), floor_divide, input);
    let small_top_left = fill((size - 1, size - 1), floor_divide, input);
    let small_bottom_left = fill((0, size - 1), floor_divide, input);

    let floor_divide_big = ((size * 3) / 2) - 1;
    let big_top_right = fill((size - 1, 0), floor_divide_big, input);
    let big_bottom_right = fill((0, 0), floor_divide_big, input);
    let big_top_left = fill((size - 1, size - 1), floor_divide_big, input);
    let big_bottom_left = fill((0, size - 1), floor_divide_big, input);

    let odd_points = fill(start_location, size * 2 + 1, input);
    let even_points = fill(start_location, size * 2, input);

    odd_grids * odd_points
        + even_grids * even_points
        + top_corner
        + right_corner
        + bottom_corner
        + left_corner
        + (grid_width + 1)
            * (small_top_right + small_top_left + small_bottom_right + small_bottom_left)
        + grid_width * (big_bottom_left + big_bottom_right + big_top_left + big_top_right)
}
