use std::cmp::{max, min};

use library::{get_filename_arg, get_two_dimensional_vector};

fn main() {
    let file_name = get_filename_arg();
    let input = get_two_dimensional_vector(&file_name);

    let mut empty_columns = vec![];
    for i in 0..input.len() {
        let column = get_column(&input, i);
        if !column.contains(&'#') {
            empty_columns.push(i);
        }
    }

    let mut empty_rows = vec![];
    for i in 0..input.len() {
        if !input[i].contains(&'#') {
            empty_rows.push(i);
        }
    }

    let mut star_locations = vec![];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '#' {
                star_locations.push((i, j));
            }
        }
    }

    let mut part_one_sum = 0;
    let mut part_two_sum = 0;

    for i in 0..star_locations.len() {
        for j in i + 1..star_locations.len() {
            part_one_sum += get_distance(
                star_locations[i],
                star_locations[j],
                &empty_rows,
                &empty_columns,
                2,
            );
            part_two_sum += get_distance(
                star_locations[i],
                star_locations[j],
                &empty_rows,
                &empty_columns,
                1000000,
            );
        }
    }

    println!("Part 1: {}", part_one_sum);
    println!("Part 2: {}", part_two_sum);
}

fn get_column(vector: &Vec<Vec<char>>, index: usize) -> Vec<char> {
    vector.iter().map(|x| x[index]).collect()
}

fn get_distance(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    multiplier: u64,
) -> u64 {
    let (x_a, y_a) = a;
    let (x_b, y_b) = b;
    let mut distance = 0;

    for i in min(x_a, x_b)..max(x_a, x_b) {
        if empty_rows.contains(&i) {
            distance += multiplier;
        } else {
            distance += 1;
        }
    }

    for i in min(y_a, y_b)..max(y_a, y_b) {
        if empty_columns.contains(&i) {
            distance += multiplier;
        } else {
            distance += 1;
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column() {
        let test_input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let column = get_column(&test_input, 1);

        assert_eq!(column.len(), 2);
        assert_eq!(column[0], 'b');
        assert_eq!(column[1], 'e');
    }

    #[test]
    fn test_get_distance() {
        assert_eq!(
            get_distance((0, 3), (1, 7), &vec![3, 7], &vec![2, 5, 8], 2),
            6
        );
    }
}
