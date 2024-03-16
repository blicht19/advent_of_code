use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use library::{get_filename_arg, get_two_dimensional_number_vector, Directions};

fn main() {
    let file_name = get_filename_arg();
    // let file_name = String::from("resources/test_input.txt");
    let input = get_two_dimensional_number_vector(&file_name);

    println!("Part 1: {}", part_one(&input));
}

#[derive(Clone)]
struct Node {
    row: usize,
    column: usize,
    direction: Option<Directions>,
    straight_count: usize,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction_string = match self.direction {
            Some(x) => format!("{x}"),
            None => String::from("NONE"),
        };
        write!(
            f,
            "{},{},{},{}",
            self.row, self.column, direction_string, self.straight_count
        )
    }
}

struct FrontierNode {
    row: usize,
    column: usize,
    direction: Option<Directions>,
    can_move_straight: bool,
}

impl Display for FrontierNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction_string = match self.direction {
            Some(x) => format!("{x}"),
            None => String::from("NONE"),
        };
        write!(
            f,
            "{},{},{},{}",
            self.row, self.column, direction_string, self.can_move_straight
        )
    }
}

fn get_neighbors(node: &Node, input: &Vec<Vec<u32>>) -> Vec<Node> {
    let mut neighbors = vec![];

    if node.direction == Some(Directions::DOWN) {
        if node.straight_count < 3 && node.row < input.len() - 1 {
            neighbors.push(Node {
                row: node.row + 1,
                column: node.column,
                direction: Some(Directions::DOWN),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.row < input.len() - 1 && node.direction != Some(Directions::UP) {
        neighbors.push(Node {
            row: node.row + 1,
            column: node.column,
            direction: Some(Directions::DOWN),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::UP) {
        if node.straight_count < 3 && node.row > 0 {
            neighbors.push(Node {
                row: node.row - 1,
                column: node.column,
                direction: Some(Directions::UP),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.row > 0 && node.direction != Some(Directions::DOWN) {
        neighbors.push(Node {
            row: node.row - 1,
            column: node.column,
            direction: Some(Directions::UP),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::RIGHT) {
        if node.straight_count < 3 && node.column < input[node.row].len() - 1 {
            neighbors.push(Node {
                row: node.row,
                column: node.column + 1,
                direction: Some(Directions::RIGHT),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.column < input[node.row].len() - 1 && node.direction != Some(Directions::LEFT) {
        neighbors.push(Node {
            row: node.row,
            column: node.column + 1,
            direction: Some(Directions::RIGHT),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::LEFT) {
        if node.straight_count < 3 && node.column > 0 {
            neighbors.push(Node {
                row: node.row,
                column: node.column - 1,
                direction: Some(Directions::LEFT),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.column > 0 && node.direction != Some(Directions::RIGHT) {
        neighbors.push(Node {
            row: node.row,
            column: node.column - 1,
            direction: Some(Directions::LEFT),
            straight_count: 1,
        });
    }

    neighbors
}

fn compare_nodes(a: &Node, b: &Node, cost_so_far: &HashMap<String, u32>) -> Ordering {
    let a_cost = *cost_so_far
        .get(&format!(
            "{}",
            FrontierNode {
                row: a.row,
                column: a.column,
                direction: a.direction,
                can_move_straight: a.straight_count < 3
            }
        ))
        .unwrap();
    let b_cost = *cost_so_far
        .get(&format!(
            "{}",
            FrontierNode {
                row: b.row,
                column: b.column,
                direction: b.direction,
                can_move_straight: b.straight_count < 3
            }
        ))
        .unwrap();

    u32::cmp(&a_cost, &b_cost)
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    let mut frontier: Vec<Node> = vec![];
    let mut cost_so_far = HashMap::new();
    let start_node = Node {
        row: 0,
        column: 0,
        direction: None,
        straight_count: 0,
    };
    frontier.push(start_node);
    let key = format!(
        "{}",
        FrontierNode {
            row: 0,
            column: 0,
            direction: None,
            can_move_straight: true
        }
    );
    cost_so_far.insert(key, 0);

    while frontier.len() > 0 {
        let current = frontier.remove(0);
        let current_key = format!(
            "{}",
            FrontierNode {
                row: current.row,
                column: current.column,
                direction: current.direction,
                can_move_straight: current.straight_count < 3
            }
        );

        if current.row == input.len() - 1 && current.column == input[current.row].len() - 1 {
            return *cost_so_far.get(&current_key).unwrap();
        }

        let neighbors = get_neighbors(&current, input);
        for neighbor in neighbors {
            let neighbor_key = format!(
                "{}",
                FrontierNode {
                    row: neighbor.row,
                    column: neighbor.column,
                    direction: neighbor.direction,
                    can_move_straight: neighbor.straight_count < 3,
                },
            );
            let new_cost =
                cost_so_far.get(&current_key).unwrap() + input[neighbor.row][neighbor.column];
            if !cost_so_far.contains_key(&neighbor_key)
                || new_cost < *cost_so_far.get(&neighbor_key).unwrap()
            {
                cost_so_far.insert(neighbor_key, new_cost);
                frontier.push(neighbor);
                frontier.sort_by(|a, b| compare_nodes(a, b, &cost_so_far));
            }
        }
    }

    0
}
