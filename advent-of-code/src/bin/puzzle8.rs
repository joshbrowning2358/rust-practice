use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use num::integer::lcm;

fn main() {
    //let file_path = "data/puzzle8/example.txt";
    let file_path = "data/puzzle8/input.txt";
    //let file_path = "data/puzzle8/example2.txt";

    //let ans = part_a(file_path);
    //println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut idx: i32 = 0;
    let mut directions: String = String::new();
    let mut network: HashMap<String, [String; 2]> = HashMap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if idx == 0 {
                    directions = ip.to_string();
                } else if ip.len() > 0 {
                    network.insert(ip[..3].to_string(), [ip[7..10].to_string(), ip[12..15].to_string()]);
                }
                idx += 1;
            }
        }
    }
    //println!("Instructions are {directions} and network is {:?}", network);

    // Traverse the network
    let mut current_node = "AAA".to_string();
    let mut curr_directions = directions.chars();
    loop {
        match curr_directions.next() {
            Some(x) => {
                if x == 'L' {current_node = network.get(&current_node).unwrap()[0].clone();}
                if x == 'R' {current_node = network.get(&current_node).unwrap()[1].clone();}
                ans += 1;
            },
            _ => {
                curr_directions = directions.chars();
            }
        }
        if current_node == "ZZZ" {break;}
    }
    ans
}

fn part_b(file_path: &str) -> i64 {
    let mut ans: i32 = 0;  // Should rename to steps but I'm too lazy
    let mut idx: i32 = 0;
    let mut directions: String = String::new();
    let mut network: HashMap<String, [String; 2]> = HashMap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if idx == 0 {
                    directions = ip.to_string();
                } else if ip.len() > 0 {
                    network.insert(ip[..3].to_string(), [ip[7..10].to_string(), ip[12..15].to_string()]);
                }
                idx += 1;
            }
        }
    }
    //println!("Instructions are {directions} and network is {:?}", network);

    // Traverse the network
    let mut current_nodes: VecDeque<String> = VecDeque::new();
    for k in network.keys() {
        if k.ends_with("A") {current_nodes.push_back(k.to_string())}
    }

    let mut curr_directions = directions.chars();
    let mut cycle_lens: Vec<i32> = vec![-1; current_nodes.len()];
    let mut first_z: Vec<i32> = vec![-1; current_nodes.len()];
    let mut current_z: Vec<i32> = vec![-1; current_nodes.len()];
    loop {
        let mut next_nodes: VecDeque<String> = VecDeque::new();
        match curr_directions.next() {
            Some(x) => {
                if x == 'L' {
                    for current_node in current_nodes {
                       next_nodes.push_back(network.get(&current_node).unwrap()[0].clone());
                    }
                } else {
                    for current_node in current_nodes {
                        next_nodes.push_back(network.get(&current_node).unwrap()[1].clone());
                    }
                }
                ans += 1;
                current_nodes = next_nodes.clone();
                //println!("Current nodes are now {:?} and current_z is {:?}", current_nodes, current_z);
                for (i, next_node) in next_nodes.iter().enumerate() {
                    if next_node.ends_with("Z") {
                        if first_z[i] == -1 {
                            println!("Found Z for node {i} but no current value");
                            first_z[i] = ans;
                            current_z[i] = ans;
                        } else {
                            println!("Found cycle of len {} for node {i}, current_z is {:?}", ans - current_z[i], current_z);
                            cycle_lens[i] = ans - current_z[i];
                            current_z[i] = ans;
                        }
                    }
                }
                if cycle_lens.iter().min().unwrap() > &-1 {
                    break;
                }
            },
            _ => {
                curr_directions = directions.chars();
            }
        }
    }

    // ans is actually # steps, we need least common multiple of all cycle lens
    let mut ans2: i64 = lcm(cycle_lens[0] as i64, cycle_lens[1] as i64);
    for i in 2..cycle_lens.len() {
        ans2 = lcm(ans2, cycle_lens[i] as i64);
    }
    ans2
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
