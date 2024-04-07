use library::{get_filename_arg, get_lines};
use std::collections::HashMap;

fn main() {
    let file_name = get_filename_arg();
    let lines = get_lines(&file_name);
    let right_left: Vec<char> = lines[0].replace("\n", "").chars().collect();
    let map_lines: Vec<String> = lines[2..lines.len()]
        .into_iter()
        .map(|f| f.replace("\n", ""))
        .collect();

    let mappings = get_mappings(map_lines);
    println!(
        "Part 1: {}",
        get_distance_to_end(&String::from("AAA"), &right_left, &mappings, |f| f == "ZZZ")
    );

    let mut distances = vec![];
    for key in mappings.keys() {
        if key.chars().collect::<Vec<char>>()[2] == 'A' {
            distances.push(get_distance_to_end(key, &right_left, &mappings, |f| {
                f.contains("Z")
            }));
        }
    }

    println!("Part 2: {}", least_common_multiple(distances));
}

struct Node {
    l: String,
    r: String,
}

fn get_mappings(lines: Vec<String>) -> HashMap<String, Node> {
    let mut mappings = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split(" = (").collect();
        let key = split[0].to_string();
        let values: Vec<&str> = split[1].split(", ").collect();
        mappings.insert(
            key,
            Node {
                l: values[0].to_string(),
                r: values[1].to_string().replace(")", ""),
            },
        );
    }

    mappings
}

fn get_distance_to_end<F>(
    key: &String,
    right_left: &Vec<char>,
    mappings: &HashMap<String, Node>,
    end_condition: F,
) -> usize
where
    F: Fn(&String) -> bool,
{
    let mut count = 0;
    let mut current_key: &String = key;

    loop {
        if end_condition(current_key) {
            break;
        }

        let direction = right_left[count % right_left.len()];
        let next_node = mappings.get(current_key).expect("No node found for key");

        current_key = if direction == 'L' {
            &next_node.l
        } else if direction == 'R' {
            &next_node.r
        } else {
            panic!("No node found for key");
        };

        count += 1;
    }

    count
}

/* least_common_multiple based on this StackOverflow answer:
 * https://stackoverflow.com/a/31302607 */

fn greatest_common_denominator(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_denominator(b, a % b)
    }
}

fn lcm_helper(a: usize, b: usize) -> usize {
    a * b / greatest_common_denominator(a, b)
}

fn least_common_multiple(values: Vec<usize>) -> usize {
    let mut multiple = values[0];

    for value in values {
        multiple = lcm_helper(multiple, value);
    }

    multiple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mappings() {
        let test_input = vec![
            String::from("AAA = (BBB, BBB)"),
            String::from("BBB = (AAA, ZZZ)"),
        ];
        let mappings = get_mappings(test_input);

        let a = mappings.get(&String::from("AAA")).unwrap();
        let b = mappings.get(&String::from("BBB")).unwrap();

        assert_eq!(a.l, "BBB");
        assert_eq!(a.r, "BBB");
        assert_eq!(b.l, "AAA");
        assert_eq!(b.r, "ZZZ");
    }

    #[test]
    fn test_greatest_common_denominator() {
        assert_eq!(greatest_common_denominator(1, 2), 1);
        assert_eq!(greatest_common_denominator(2, 2), 2);
        assert_eq!(greatest_common_denominator(4, 20), 4);
        assert_eq!(greatest_common_denominator(15, 20), 5);
    }

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(least_common_multiple(vec![5, 5]), 5);
        assert_eq!(least_common_multiple(vec![4, 5]), 20);
        assert_eq!(least_common_multiple(vec![10, 15]), 30);
    }
}
