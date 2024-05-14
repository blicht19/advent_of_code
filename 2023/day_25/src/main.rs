use library::{get_filename_arg, get_lines};
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet, VecDeque};

/* Based on comment by LtHummus
* https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/keu45a6/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button */
fn main() {
    let file_name = get_filename_arg();
    let input = get_lines(&file_name);

    let mut connections = get_connections(input);

    let mut counts = HashMap::new();
    let nodes: Vec<String> = connections.keys().map(|a| a.clone()).collect();

    for _ in 0..100 {
        let mut random_nodes = nodes.choose_multiple(&mut rand::thread_rng(), 2);

        let path = get_path(
            &connections,
            &random_nodes.next().unwrap(),
            &random_nodes.next().unwrap(),
        );

        for entry in path {
            let count = counts.get(&entry).unwrap_or(&0);
            counts.insert(entry, count + 1);
        }
    }

    let mut counts: Vec<(&String, &i32)> = counts.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1));

    disconnect(&mut connections, &counts[0].0, &counts[1].0);
    disconnect(&mut connections, &counts[2].0, &counts[3].0);
    disconnect(&mut connections, &counts[4].0, &counts[5].0);

    println!(
        "{}",
        count_reachable_nodes(&connections, &counts[0].0)
            * count_reachable_nodes(&connections, &counts[1].0)
    );
}

fn get_connections(input: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::new();

    for line in input.iter() {
        let (node, connected) = line.split_once(": ").unwrap();
        let node = node.to_string();
        let connected_nodes: Vec<String> = connected.split(" ").map(|s| s.to_string()).collect();

        if !connections.contains_key(&node) {
            connections.insert(node.clone(), HashSet::new());
        }

        for connected_node in connected_nodes {
            connections
                .get_mut(&node)
                .unwrap()
                .insert(connected_node.clone());
            if !connections.contains_key(&connected_node) {
                connections.insert(connected_node.clone(), HashSet::new());
            }
            connections
                .get_mut(&connected_node)
                .unwrap()
                .insert(node.clone());
        }
    }

    connections
}

fn get_path(connections: &HashMap<String, HashSet<String>>, a: &String, b: &String) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((a, vec![a.to_string()]));

    while let Some((current, path_traveled)) = queue.pop_front() {
        if current == b {
            return path_traveled;
        }

        for connected_node in connections.get(current).unwrap() {
            if !path_traveled.contains(connected_node) {
                let mut visited = path_traveled.clone();
                visited.push(connected_node.clone());
                queue.push_back((connected_node, visited));
            }
        }
    }

    vec![]
}

fn disconnect(connnections: &mut HashMap<String, HashSet<String>>, a: &String, b: &String) {
    connnections.get_mut(a).unwrap().remove(b);
    connnections.get_mut(b).unwrap().remove(a);
}

fn count_reachable_nodes(connections: &HashMap<String, HashSet<String>>, node: &String) -> usize {
    let mut visited = HashSet::new();
    visited.insert(node.clone());
    find_reachable_nodes(connections, node, &mut visited);
    visited.len()
}

fn find_reachable_nodes(
    connections: &HashMap<String, HashSet<String>>,
    node: &String,
    visited: &mut HashSet<String>,
) {
    for reachable in connections.get(node).unwrap() {
        if !visited.contains(reachable) {
            visited.insert(reachable.clone());
            find_reachable_nodes(connections, reachable, visited);
        }
    }
}
