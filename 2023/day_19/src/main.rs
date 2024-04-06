use std::collections::HashMap;

use library::{get_filename_arg, get_lines};

/* Based on solution by Reddit user Polaric_Spiral https://www.reddit.com/r/adventofcode/comments/18ltr8m/comment/ke48wv2/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button&rdt=35843 */
fn main() {
    let file_name = get_filename_arg();
    let mut workflow_strings = get_lines(&file_name);
    let mut parts =
        workflow_strings.split_off(workflow_strings.iter().position(|i| i == "").unwrap());
    parts.remove(0);

    let mut workflows = HashMap::new();
    for workflow_string in workflow_strings {
        let replaced = workflow_string.replace("}", "");
        let splits: Vec<&str> = replaced.split("{").collect();
        let (key, rule) = (String::from(splits[0]), String::from(splits[1]));
        let rules: Vec<Rule> = rule.split(",").map(|r| parse_rule(r)).collect();
        workflows.insert(key, rules);
    }
    println!("Part 1: {}", part_one(&workflows, &parts));
    println!("Part 2: {}", part_two(&workflows));
}

fn part_one(workflows: &HashMap<String, Vec<Rule>>, part_strings: &Vec<String>) -> i64 {
    let mut parts = vec![];
    for part in part_strings {
        parts.push(parse_part(part));
    }

    parts
        .into_iter()
        .map(|mut part| {
            get_accepted_count(&mut part, workflows, None) * part.values().map(|p| p.0).sum::<i64>()
        })
        .sum()
}

fn part_two(workflows: &HashMap<String, Vec<Rule>>) -> i64 {
    let mut ratings = HashMap::from([
        (String::from("x"), (1, 4000)),
        (String::from("m"), (1, 4000)),
        (String::from("a"), (1, 4000)),
        (String::from("s"), (1, 4000)),
    ]);
    let x = get_accepted_count(&mut ratings, workflows, None);
    x
}

#[derive(Debug)]
struct Rule {
    value: i64,
    operation: i64,
    category: String,
    goto: String,
}

fn parse_rule(rule: &str) -> Rule {
    if !rule.contains(":") {
        return Rule {
            value: 0,
            operation: 1,
            category: String::from("x"),
            goto: String::from(rule),
        };
    }

    let split: Vec<&str> = rule.split(":").collect();
    let (evaluation, goto) = (split[0], split[1]);
    let evaluation_chars: Vec<char> = evaluation.chars().collect();
    let category = String::from(evaluation_chars[0]);
    let comparison = evaluation_chars[1];
    let operation = if comparison == '<' { -1 } else { 1 };
    let value = evaluation[2..evaluation.len()].parse::<i64>().unwrap();

    Rule {
        value,
        operation,
        category,
        goto: String::from(goto),
    }
}

fn parse_part(part: &str) -> HashMap<String, (i64, i64)> {
    let mut current_part = HashMap::new();
    let line: Vec<&str> = part[1..part.len() - 1].split(",").collect();
    for pair in line {
        let split: Vec<&str> = pair.split("=").collect();
        let rating = split[1].parse().unwrap();
        current_part.insert(String::from(split[0]), (rating, rating));
    }

    current_part
}

fn get_accepted_count(
    ratings: &mut HashMap<String, (i64, i64)>,
    workflows: &HashMap<String, Vec<Rule>>,
    workflow: Option<&String>,
) -> i64 {
    let workflow = match workflow {
        Some(x) => x,
        None => "in",
    };

    if workflow == "R" {
        return 0;
    }

    if workflow == "A" {
        let mut product = 1;
        let values: Vec<(i64, i64)> = ratings.values().cloned().collect();
        for (low, high) in values {
            product *= high - low + 1;
        }
        return product;
    }

    let mut combinations = 0;

    for Rule {
        value,
        operation,
        category,
        goto,
    } in workflows.get(workflow).unwrap()
    {
        let (low, high) = ratings.get(category).unwrap();
        if (low - value) * operation > 0 {
            if (high - value) * operation > 0 {
                return combinations + get_accepted_count(ratings, workflows, Some(goto));
            } else {
                let mut next_part = ratings.clone();
                next_part.insert(String::from(category), (*low, value - 1));
                combinations += get_accepted_count(&mut next_part, workflows, None);
                ratings.insert(String::from(category), (*value, *high));
            }
        } else if (high - value) * operation > 0 {
            let mut next_part = ratings.clone();
            next_part.insert(String::from(category), (value + 1, *high));
            combinations += get_accepted_count(&mut next_part, workflows, None);
            ratings.insert(String::from(category), (*low, *value));
        }
    }

    0
}
