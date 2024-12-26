use std::collections::HashMap;

use advent_of_code_2024::file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    let ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let adjacency = parse_input(file_path);

    let mut num_triangles = 0;
    for (node, adj_nodes) in adjacency.clone().into_iter() {
        //if !node.starts_with("t") {
        //    continue
        //}
        for adj_node in adj_nodes {
            for third_node in adjacency.get(&adj_node).unwrap() {
                if adjacency.get(third_node).unwrap().contains(&node) {
                    if node.starts_with("t") | adj_node.starts_with("t") | third_node.starts_with("t") {
                        num_triangles += 1;
                    }
                }
            }
        }
    }
    return num_triangles / 6
}

fn part_b(file_path: &str) -> String {
    let adjacency = parse_input(file_path);
    let mut node_names: Vec<String> = vec![];
    for k in adjacency.keys() {
        node_names.push(k.clone());
    }

    let mut largest_set: Vec<usize> = vec![];
    let mut set_indices: Vec<usize> = vec![0];
    let mut next_cand: usize = 1;
    while (set_indices.len() > 0) | (next_cand < node_names.len() - largest_set.len()) {
        if next_cand == node_names.len() {
            next_cand = set_indices.pop().unwrap() + 1;
        } else {
            if is_connected_to_set(&adjacency, &node_names, &set_indices, next_cand) {
                set_indices.push(next_cand);
                if set_indices.len() > largest_set.len() {
                    largest_set = set_indices.clone();
                }
            }
            next_cand += 1;
        }
    }

    return set_to_string(largest_set, &node_names)
}

fn parse_input(file_path: &str) -> HashMap<String, Vec<String>> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let (node1, node2) = row.split_once('-').unwrap();
                let node1_string = node1.to_string();
                let node2_string = node2.to_string();
                adjacency.entry(node1_string.clone()).or_insert(Vec::new()).push(node2_string.clone());
                adjacency.entry(node2_string).or_insert(Vec::new()).push(node1_string);
            }
        }
    }

    return adjacency
}

fn is_connected_to_set(adjacency: &HashMap<String, Vec<String>>, node_names: &Vec<String>, curr_clique: &Vec<usize>, cand: usize) -> bool {
    let cand_name = node_names[cand].clone();
    for node_idx in curr_clique {
        let node_name = node_names[*node_idx].clone();
        if !adjacency.get(&node_name).unwrap().contains(&cand_name) {
            return false
        }
    }
    return true
}

fn set_to_string(ids: Vec<usize>, node_names: &Vec<String>) -> String {
    let mut result: Vec<String> = vec![];
    for node_idx in ids {
        result.push(node_names[node_idx].clone());
    }
    result.sort();
    return result.join(",")
}
