use std::collections::HashSet;

use library::{get_filename_arg, get_two_dimensional_vector};

fn main() {
    let file_name = get_filename_arg();
    let input = get_two_dimensional_vector(&file_name);
    println!(
        "Part 1: {}",
        get_longest_path(&input, get_neighbors_part_one)
    );
    println!(
        "Part 2: {}",
        get_longest_path(&input, get_neighbors_part_two)
    );
}

fn get_longest_path(
    input: &Vec<Vec<char>>,
    get_neighbors: fn((usize, usize), &Vec<Vec<char>>) -> Vec<(usize, usize)>,
) -> u32 {
    let start_location = (0, 1);
    let mut visited = HashSet::new();
    visited.insert(start_location);
    get_longest((1, 1), &mut visited, input, 1, get_neighbors)
}

fn get_longest(
    current_location: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>,
    current_value: u32,
    get_neighbors: fn((usize, usize), &Vec<Vec<char>>) -> Vec<(usize, usize)>,
) -> u32 {
    if visited.contains(&current_location) || map[current_location.0][current_location.1] == '#' {
        return 0;
    }

    if current_location == (map.len() - 1, map[0].len() - 2) {
        return current_value;
    }

    visited.insert(current_location);
    let neighbors = get_neighbors(current_location, map);
    let max = neighbors
        .iter()
        .map(|&neighbor| get_longest(neighbor, visited, map, current_value + 1, get_neighbors))
        .max()
        .unwrap();
    visited.remove(&current_location);

    max
}

fn get_neighbors_part_one(location: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let current_tile = map[location.0][location.1];
    let neighbors = if current_tile == '.' {
        vec![
            (location.0 + 1, location.1),
            (location.0, location.1 + 1),
            (location.0 - 1, location.1),
            (location.0, location.1 - 1),
        ]
    } else if current_tile == '>' {
        vec![(location.0, location.1 + 1)]
    } else {
        vec![(location.0 + 1, location.1)]
    }
    .into_iter()
    .filter(|neighbor| map[neighbor.0][neighbor.1] != '#')
    .collect();

    neighbors
}

fn get_neighbors_part_two(location: (usize, usize), _: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    vec![
        (location.0 + 1, location.1),
        (location.0, location.1 + 1),
        (location.0 - 1, location.1),
        (location.0, location.1 - 1),
    ]
}
