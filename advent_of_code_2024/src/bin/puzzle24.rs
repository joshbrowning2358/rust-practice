use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;

use advent_of_code_2024::file_reader;

#[derive(Debug, Clone)]
pub struct Equation {
    pub node1: String,
    pub node2: String,
    pub operator: i32,  // AND -> 0, OR -> 1, XOR -> 2
    pub out_node: String,
}

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

fn part_a(file_path: &str) -> i64 {
    let (mut initial_vals, mut equations) = parse_input(file_path);
    forward_propogate(&mut initial_vals, &mut equations);
    return compute_result(&initial_vals, "z")
}

fn part_b(file_path: &str) -> String {
    let (init_vals, mut init_equations) = parse_input(file_path);
    
    let mut swapped_wires: Vec<(String, String)> = vec![];

    for _ in 0..4 {
    // loop {
        let mut equations = init_equations.clone();
        let mut vals = init_vals.clone();
        forward_propogate(&mut vals, &mut equations);

        let mut idx = find_first_incorrect(&vals);
        if idx == 100 {
            let mut all_work = true;
            // Ensure it works for 3 other inputs
            for _ in 0..3 {
                let mut val_copy = init_vals.clone();
                let mut equations_copy = init_equations.clone();
                // Randomly update 20 x values
                for _ in 0..20 {
                    let idx = rand::thread_rng().gen_range(0..45);
                    val_copy.insert(format!("x{:0>2}", idx), rand::random());
                }
                forward_propogate(&mut val_copy, &mut equations_copy);
                idx = find_first_incorrect(&val_copy);
                if idx < 100 {
                    all_work = false;
                    break
                }
            }
            
            if all_work {
                break
           }
        }

        let candidates = Vec::from_iter(find_candidates(&init_equations, idx));

        'swapping: for idx1 in 0..(candidates.len() - 1) {
            for idx2 in (idx1 + 1)..candidates.len() {
                let mut test_equations = init_equations.clone();
                let success = swap_wires(&mut test_equations, &candidates[idx1], &candidates[idx2]);
                if success {
                    let mut vals = init_vals.clone();
                    let success2 = forward_propogate(&mut vals, &mut test_equations.clone());
                    if success2 {
                        let test_idx = find_first_incorrect(&vals);
                        if test_idx > idx {
                            if test_cand(&mut init_vals.clone(), &test_equations, idx) {
                                swapped_wires.push((candidates[idx1].clone(), candidates[idx2].clone()));
                                init_equations = test_equations;
                                break 'swapping
                            }
                        }
                    }
                }
            }
        }
        // break
    }
    return sort_wires_alphabetically(swapped_wires)
}

fn sort_wires_alphabetically(wires: Vec<(String, String)>) -> String {
    let mut wire_list = vec![];
    for (left, right) in wires.iter() {
        wire_list.push(left.to_string());
        wire_list.push(right.to_string());
    }
    wire_list.sort();
    return wire_list.join(",")
}

fn test_cand(vals: &mut HashMap<String, bool>, equations: &VecDeque<Equation>, idx: usize) -> bool {
    let mut n_success = 0;
    for x in vec![true, false] {
        for y in vec![true, false] {
            for x2 in vec![true, false] {
                for y2 in vec![true, false] {
                    let mut curr_vals = vals.clone();
                    curr_vals.insert(format!("x{:0>2}", idx), x);
                    curr_vals.insert(format!("y{:0>2}", idx), y);
                    curr_vals.insert(format!("x{:0>2}", idx - 1), x2);
                    curr_vals.insert(format!("y{:0>2}", idx - 1), y2);
                    forward_propogate(&mut curr_vals, &mut equations.clone());
                    if find_first_incorrect(&curr_vals) > idx {
                        n_success += 1;
                    }
                }
            }
        }
    }
    if n_success == 16 {
        return true
    }
    return false
}

fn swap_wires(equations: &mut VecDeque<Equation>, left: &String, right: &String) -> bool {
    let mut n_swaps = 0;
    let mut n_checks = 0;
    while (n_swaps < 2) & (n_checks < equations.len()) {
        let mut eq = equations.pop_front().unwrap();
        if eq.out_node == left.to_string() {
            eq.out_node = right.to_string();
            n_swaps += 1;
        } else if eq.out_node == right.to_string() {
            eq.out_node = left.to_string();
            n_swaps += 1;
        }
        n_checks += 1;
        equations.push_back(eq);
    }
    if n_swaps == 2 {
        return true
    }
    return false
}

fn find_candidates(equations: &VecDeque<Equation>, idx: usize) -> HashSet<String> {
    let mut candidates: HashSet<String> = HashSet::new();
    candidates.insert(format!("z{:0>2}", idx));
    for _ in 0..3 {  // Only backpropogate three times, should be enough to find issue but could increase
        let mut update = HashSet::new();
        for eq in equations {
            if candidates.contains(&eq.node1) | candidates.contains(&eq.node2) | candidates.contains(&eq.out_node) {
                update.insert(eq.node1.clone());
                update.insert(eq.node2.clone());
                update.insert(eq.out_node.clone());
            }
        }
        candidates.extend(update);
    }
    return candidates
}

fn find_first_incorrect(initial_vals: &HashMap<String, bool>) -> usize {
    let mut carry = 0;
    for i in 0..45 {
        let summation = *initial_vals.get(&format!("x{:0>2}", i)).unwrap() as i32 + *initial_vals.get(&format!("y{:0>2}", i)).unwrap() as i32 + carry;
        let expected_z = (summation % 2) == 1;
        if summation >= 2 {
            carry = 1;
        } else {
            carry = 0;
        }
        if expected_z != *initial_vals.get(&format!("z{:0>2}", i)).unwrap() {
            return i
        }
    }
    return 100
}

fn forward_propogate(initial_vals: &mut HashMap<String, bool>, equations: &mut VecDeque<Equation>) -> bool {
    let mut iterations_without_update = 0;
    while equations.len() > 0 {
        let equation = equations.pop_front().unwrap();
        if initial_vals.contains_key(&equation.node1) & initial_vals.contains_key(&equation.node2) {
            let new_val = match equation.operator {
                0 => initial_vals.get(&equation.node1).unwrap() & initial_vals.get(&equation.node2).unwrap(),
                1 => initial_vals.get(&equation.node1).unwrap() | initial_vals.get(&equation.node2).unwrap(),
                2 => initial_vals.get(&equation.node1).unwrap() ^ initial_vals.get(&equation.node2).unwrap(),
                _ => {panic!("Invalid value!");},
            };
            initial_vals.insert(equation.out_node, new_val);
            iterations_without_update = 0;
        } else {
            equations.push_back(equation);
            iterations_without_update += 1;
        }
        if iterations_without_update > 200 {
            return false
        }
    }
    return true
}

fn parse_input(file_path: &str) -> (HashMap<String, bool>, VecDeque<Equation>) {
    let mut initial_vals: HashMap<String, bool> = HashMap::new();
    let mut equations: VecDeque<Equation> = VecDeque::new();

    let mut on_equations: bool = false;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    on_equations = true
                } else if on_equations {
                    let (expr, out_node) = row.split_once(" -> ").unwrap();
                    let vals: Vec<&str> = expr.split(" ").collect();
                    let operator_id = match vals[1] {
                        "AND" => 0,
                        "OR" => 1,
                        "XOR" => 2,
                        _ => panic!("Invalid value!"),
                    };
                    let eq = Equation{
                        node1: vals[0].to_string(),
                        node2: vals[2].to_string(),
                        operator: operator_id,
                        out_node: out_node.to_string()
                    };
                    if row.starts_with("x") | row.starts_with("y") {
                        equations.push_front(eq);
                    } else {
                        equations.push_back(eq);
                    }
                } else {
                    let (key, val_str) = row.split_once(": ").unwrap();
                    initial_vals.insert(key.to_string(), val_str.parse::<i32>().unwrap() != 0);
                }
            }
        }
    }

    return (initial_vals, equations)
}

fn compute_result(vals: &HashMap<String, bool>, start_letter: &str) -> i64 {
    let mut result: i64 = 0;
    for (key, val) in vals {
        if key.starts_with(start_letter) {
            let exponent = key[1..].parse::<u32>().unwrap();
            if *val {
                result += 2_i64.pow(exponent);
            }
        }
    }
    return result
}
