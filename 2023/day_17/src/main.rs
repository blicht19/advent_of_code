use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use library::{get_filename_arg, get_two_dimensional_number_vector, Directions};

fn main() {
    let file_name = get_filename_arg();
    let input = get_two_dimensional_number_vector(&file_name);

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
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

fn get_neighbors(
    node: &Node,
    input: &Vec<Vec<u32>>,
    can_move_straight: fn(&Node) -> bool,
    can_turn: fn(&Node) -> bool,
) -> Vec<Node> {
    let mut neighbors = vec![];

    let node_can_move_straight = can_move_straight(&node);
    let node_can_turn = can_turn(&node);

    if node.direction == Some(Directions::DOWN) {
        if node_can_move_straight && node.row < input.len() - 1 {
            neighbors.push(Node {
                row: node.row + 1,
                column: node.column,
                direction: Some(Directions::DOWN),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.row < input.len() - 1 && node.direction != Some(Directions::UP) && node_can_turn
    {
        neighbors.push(Node {
            row: node.row + 1,
            column: node.column,
            direction: Some(Directions::DOWN),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::UP) {
        if node_can_move_straight && node.row > 0 {
            neighbors.push(Node {
                row: node.row - 1,
                column: node.column,
                direction: Some(Directions::UP),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.row > 0 && node.direction != Some(Directions::DOWN) && node_can_turn {
        neighbors.push(Node {
            row: node.row - 1,
            column: node.column,
            direction: Some(Directions::UP),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::RIGHT) {
        if node_can_move_straight && node.column < input[node.row].len() - 1 {
            neighbors.push(Node {
                row: node.row,
                column: node.column + 1,
                direction: Some(Directions::RIGHT),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.column < input[node.row].len() - 1
        && node.direction != Some(Directions::LEFT)
        && node_can_turn
    {
        neighbors.push(Node {
            row: node.row,
            column: node.column + 1,
            direction: Some(Directions::RIGHT),
            straight_count: 1,
        });
    }

    if node.direction == Some(Directions::LEFT) {
        if node_can_move_straight && node.column > 0 {
            neighbors.push(Node {
                row: node.row,
                column: node.column - 1,
                direction: Some(Directions::LEFT),
                straight_count: node.straight_count + 1,
            });
        }
    } else if node.column > 0 && node.direction != Some(Directions::RIGHT) && node_can_turn {
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
    let a_cost = *cost_so_far.get(&format!("{}", a)).unwrap();
    let b_cost = *cost_so_far.get(&format!("{}", b)).unwrap();

    u32::cmp(&a_cost, &b_cost)
}

fn get_least_cost(
    input: &Vec<Vec<u32>>,
    can_move_straight: fn(&Node) -> bool,
    can_turn: fn(&Node) -> bool,
) -> u32 {
    let mut frontier = vec![];
    let mut cost_so_far = HashMap::new();
    let start_node = Node {
        row: 0,
        column: 0,
        direction: None,
        straight_count: 0,
    };
    frontier.push(start_node.clone());
    let key = format!("{}", start_node);
    cost_so_far.insert(key, 0);

    while frontier.len() > 0 {
        let current = frontier.remove(0);
        let current_key = format!("{}", current);

        if current.row == input.len() - 1
            && current.column == input[current.row].len() - 1
            && can_move_straight(&current)
        {
            return *cost_so_far.get(&current_key).unwrap();
        }

        let neighbors = get_neighbors(&current, input, can_move_straight, can_turn);
        for neighbor in neighbors {
            let neighbor_key = format!("{}", neighbor);
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

    u32::MAX
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    get_least_cost(input, |node| node.straight_count < 3, |_| true)
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    get_least_cost(
        input,
        |node| node.straight_count < 10,
        |node| node.straight_count >= 4 || (node.row == 0 && node.column == 0),
    )
}
