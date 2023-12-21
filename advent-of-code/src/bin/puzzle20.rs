use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "data/puzzle20/example.txt";
    //let file_path = "data/puzzle20/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

#[derive(Clone, Debug)]
struct FlipFlop {
    outputs: Vec<String>,
    state: bool,
}

#[derive(Clone, Debug)]
struct Conjunction {
    outputs: Vec<String>,
    input_states: HashMap<String, bool>,
}

fn part_a(file_path: &str) -> i64 {
    let (mut flip_flops, mut conjunctions, broadcaster) = parse_file(file_path);

    let mut n_low: i64 = 0;
    let mut n_high: i64 = 0;

    for _ in 0..1 {
        let mut to_process: VecDeque<(&str, bool)> = VecDeque::new();
        let output: &str;
        //let mut flip_flop: FlipFlop;
        let mut conjunction: Conjunction;
        to_process.push_back(("broadcast", false));
        while to_process.len() > 0 {
            println!("To process is {:?}", to_process);
            let (curr_node, curr_sig) = to_process.pop_front().unwrap();
            if curr_sig {
                n_high += 1;
            } else {
                n_low += 1;
            }

            if (&flip_flops).contains_key(curr_node) {
                if !curr_sig {  // Flip flops only activate if they receive low
                    let mut flip_flop = flip_flops.remove(curr_node).unwrap().clone();
                    for i in 0..flip_flop.outputs.len() {
                        to_process.push_back((&flip_flop.outputs[i], !(flip_flop.state.clone())));
                    }
                    flip_flop.state = !flip_flop.state;
                    flip_flops.insert(curr_node, flip_flop);
                }
            } else if conjunctions.contains_key(curr_node) {
                // TODO
            } else {
                println!("Broadcaster node, key is {curr_node}");
                for output in broadcaster.get(curr_node).unwrap() {
                    to_process.push_back((&output, false));
                }
            }
        }
    }

    n_high * n_low
}

fn parse_file(file_path: &str) -> (HashMap<&str, FlipFlop>, HashMap<String, Conjunction>, HashMap<String, Vec<String>>) {
    let mut flip_flops: HashMap<&str, FlipFlop> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();
    let mut broadcaster: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (src, dest_str) = ip.split_once("->").unwrap();
                let dest: Vec<String> = dest_str.split(",").map(|x| x.trim().to_string()).collect();

                let key: String; 
                match src.chars().next().unwrap() {
                    '%' => {
                        key = src.trim().replace("%", "");
                        println!("Inserting key >{key}<");
                        flip_flops.insert(&key.clone(), FlipFlop{outputs: dest.clone(), state: false});
                    },
                    'b' => {
                        key = "broadcast".to_string();
                        broadcaster.insert(key.clone(), dest.clone());
                    },
                    '&' => {
                        key = src.trim().replace("&", "");
                        println!("Inserting key >{key}<");
                        conjunctions.insert(key.clone(), Conjunction{outputs: dest.clone(), input_states: HashMap::new()});
                    }
                    _ => {panic!("Invalid character in parsing!");}
                }

                for dest_val in dest {
                    if conjunctions.contains_key(&dest_val) {
                        conjunctions.get_mut(&dest_val).unwrap().input_states.insert(key.clone(), false);
                    }
                }
            }
        }
    }
    println!("Flip flops are {:?}", flip_flops);
    println!("Broadcaster is {:?}", broadcaster);
    println!("Conjunctions are {:?}\n\n", conjunctions);
    (flip_flops, conjunctions, broadcaster)
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
