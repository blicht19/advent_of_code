use std::fs::read_to_string;

use library::get_filename_arg;

fn main() {
    let file_name = get_filename_arg();
    let input: String = read_to_string(file_name).unwrap().parse().unwrap();
    let input: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let input: Vec<&[char]> = input.split(|s| *s == ',').collect();

    println!("Part 1: {}", part_one(&input));
}

fn get_hash_code(word: &[char]) -> u32 {
    let mut current_value = 0;
    for character in word {
        current_value += *character as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn part_one(input: &Vec<&[char]>) -> u32 {
    let mut sum = 0;
    for step in input {
        sum += get_hash_code(step);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash_code() {
        let word = vec!['H', 'A', 'S', 'H'];
        assert_eq!(get_hash_code(&word), 52);
    }
}
