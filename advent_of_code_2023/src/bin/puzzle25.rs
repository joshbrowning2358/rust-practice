use std::cmp::{min, max};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use rand::{seq::IteratorRandom, thread_rng};

fn main() {
    //let file_path = "data/puzzle25/example.txt"; //let n_nodes = 16;
    let file_path = "data/puzzle25/input.txt"; //let n_nodes = 1494;

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> usize {
    let (nodes, edges) = parse_file(file_path);
    println!("Got nodes {:?} and edges {:?}", nodes, edges);
    
    let mut edge_counts: HashMap<[i32; 2], i32> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    
    for _rand_idx in 0..100 {
        let mut rng = thread_rng();
        visited.clear();
        let sampled = nodes.iter().choose_multiple(&mut rng, 2);

        // Find shortest path from sampled[0] to sampled[1]
        let mut to_explore: VecDeque<Vec<i32>> = VecDeque::new();
        for next_node in edges.get(sampled[0]).unwrap() {
            to_explore.push_back(vec![*sampled[0], *next_node]);
        }
        visited.insert(*sampled[0]);
        loop {
            let curr_path = to_explore.pop_front().unwrap();
            //let old = match edge_counts.get_mut(&[last_node, curr_node]) {
            //    Some(x) => {*x},
            //    None => {0}
            //};
            //edge_counts.insert([last_node, curr_node], old + 1);

            let curr_node = curr_path[curr_path.len() - 1];
            for next_node in edges.get(&curr_node).unwrap() {
                if !visited.contains(next_node) {
                    let mut next_path = curr_path.clone();
                    next_path.push(*next_node);
                    to_explore.push_back(next_path);
                    visited.insert(*next_node);
                }
            }

            if curr_node == *sampled[1] {
                for i in 0..(curr_path.len() - 2) {
                    let s = min(curr_path[i], curr_path[i + 1]);
                    let e = max(curr_path[i], curr_path[i + 1]);
                    let old = match edge_counts.get_mut(&[s, e]) {
                        Some(x) => {*x},
                        None => {0}
                    };
                    edge_counts.insert([s, e], old + 1);
                }
                break;
            }
        }
    }
    let mut sorted_counts: Vec<i32> = edge_counts.values().map(|x| *x).collect();
    sorted_counts.sort();
    let cut_value = sorted_counts[sorted_counts.len() - 3];
    let edges_to_remove: Vec<[i32; 2]> = edge_counts.iter().filter(|&(k, v)| v >= &cut_value).map(|(k, v)| *k).collect();
    println!("Edges to remove are {:?}", edges_to_remove);

    let mut cut_edges = edges.clone();
    // Remove cut edges
    for [s, e] in edges_to_remove.iter() {
        let mut old = cut_edges.get_mut(s).unwrap().to_vec();
        old.retain(|&x| x != *e);
        cut_edges.insert(*s, old);
        
        old = cut_edges.get_mut(e).unwrap().to_vec();
        old.retain(|&x| x != *s);
        cut_edges.insert(*e, old);
    }

    println!("Cut edges are {:?}", cut_edges);
    let comp1 = connected_nodes(edges_to_remove[0][0], &cut_edges);
    let comp2 = connected_nodes(edges_to_remove[0][1], &cut_edges);
    println!("Components have sizes {comp1} and {comp2}");
    comp1 * comp2
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                ans += ip.len() as i32;
            }
        }
    }
    ans
}

fn connected_nodes(start: i32, edges: &HashMap<i32, Vec<i32>>) -> usize {
    let mut connected: HashSet<i32> = HashSet::new();
    connected.insert(start);
    let mut to_explore: VecDeque<i32> = VecDeque::new();
    for next_node in edges.get(&start).unwrap() {
        to_explore.push_back(*next_node);
    }
    while to_explore.len() > 0 {
        let curr_node = to_explore.pop_front().unwrap();
        connected.insert(curr_node);

        for next_node in edges.get(&curr_node).unwrap() {
            if !connected.contains(&next_node) {
                to_explore.push_back(*next_node);
            }
        }
    }
    connected.len()
}

fn parse_file(file_path: &str) -> (HashSet<i32>, HashMap<i32, Vec<i32>>) {
    let mut nodes: HashSet<i32> = HashSet::new();
    let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut node_to_id: HashMap<&str, i32> = HashMap::new();

    let mut node_id = 0;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    for ip in contents.split('\n') {
        if ip.len() < 3 {continue;}
        let (node, edge_str) = ip.split_once(':').unwrap();
        let curr_node_id: i32;
        
        let curr_edges: Vec<&str> = edge_str.split(' ').map(|x| x.trim()).filter(|x| x.len() > 0).collect();
        if !node_to_id.contains_key(node) {
            curr_node_id = node_id;
            node_to_id.insert(node, curr_node_id);
            node_id += 1;
                nodes.insert(curr_node_id);
        } else {
            curr_node_id = *node_to_id.get(node).unwrap();
        }

        // Add to edges
        let mut edge_ids: Vec<i32> = Vec::new();
        let mut edge_id: i32;
        for edge_node in curr_edges.iter() {
            if !node_to_id.contains_key(edge_node) {
                edge_id = node_id;
                node_to_id.insert(edge_node, edge_id);
                node_id += 1;
                nodes.insert(edge_id);
            } else {
                edge_id = *node_to_id.get(edge_node).unwrap();
            }
            edge_ids.push(edge_id);

            let mut old: Vec<i32>;
            if edges.contains_key(&edge_id) {
                old = edges.get_mut(&edge_id).unwrap().to_vec();
            } else {
                old = Vec::new();
            }
            old.push(curr_node_id);
            edges.insert(edge_id, old);
        }

        let mut old: Vec<i32> = match edges.get_mut(&curr_node_id) {
            Some(x) => {x.to_vec()},
            None => {Vec::new()}
        };
        old.append(&mut edge_ids);
        edges.insert(curr_node_id, old);
    }
    println!("Node mapping: {:?}", node_to_id);
    (nodes, edges)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
