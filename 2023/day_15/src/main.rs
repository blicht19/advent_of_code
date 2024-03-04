use library::get_filename_arg;
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_name = get_filename_arg();
    let input: String = read_to_string(file_name).unwrap().parse().unwrap();
    let input: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let input: Vec<&[char]> = input.split(|s| *s == ',').collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn get_hash_code(word: &[char]) -> usize {
    let mut current_value = 0;
    for character in word {
        current_value += *character as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn part_one(input: &Vec<&[char]>) -> usize {
    let mut sum = 0;
    for step in input {
        sum += get_hash_code(step);
    }

    sum
}

fn part_two(input: &Vec<&[char]>) -> usize {
    let operation_regex = Regex::new(r"=|-").unwrap();
    let value_regex = Regex::new(r"\d+").unwrap();
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for line in input {
        let line_string: String = line.iter().collect();
        let operation = operation_regex.find(&line_string).unwrap();
        let label = &line[0..operation.start()];
        let hash_code = get_hash_code(label);
        let label: String = label.iter().collect();
        let operation = operation.as_str();
        let value = value_regex.find(&line_string);

        let index = boxes[hash_code].iter().position(|x| x.0 == label);
        if operation == "-" {
            if index.is_some() {
                boxes[hash_code].remove(index.unwrap());
            }
        } else {
            let value = value.unwrap().as_str().parse().unwrap();
            if index.is_some() {
                boxes[hash_code][index.unwrap()].1 = value;
            } else {
                boxes[hash_code].push((label, value));
            }
        }
    }

    let mut sum = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let slot = 1 + j;
            let focal_length = boxes[i][j].1;
            let box_num = 1 + i;
            sum += slot * focal_length * box_num;
        }
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
