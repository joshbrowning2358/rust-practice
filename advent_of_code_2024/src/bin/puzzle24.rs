use std::collections::{HashMap, VecDeque};

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
        } else {
            equations.push_back(equation);
        }
    }

    return compute_result(&initial_vals, "z")
}

fn part_b(file_path: &str) -> i64 {
    let (initial_vals, equations) = parse_input(file_path);
    summarize(&initial_vals, &equations);

    for equation in equations {
        // To perform addition, we need
        // X(i-1) AND Y(i-1) = A    // Carry digit
        // Xi OR Yi = B
        // B OR A = Zi
    }
    return 0
}

fn summarize(initial_vals: &HashMap<String, bool>, equations: &VecDeque<Equation>) -> i32 {
    let x = compute_result(&initial_vals, "x");
    let y = compute_result(&initial_vals, "y");
    let target = x + y;
    let actual = compute_addition(&equations, &initial_vals);
    println!("x is {x}, y is {y}, target is {target}, actual is {actual}");
    println!("input x is: {:#048b}", x);
    println!("input y is: {:#048b}", y);
    println!("result is:  {:#048b}", actual);
    println!("target is:  {:#048b}", target);
    return 0
}

fn compute_addition(equations: &VecDeque<Equation>, initial_vals: &HashMap<String, bool>) -> i64{
    let mut equations = equations.clone();
    let mut initial_vals = initial_vals.clone();
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
        } else {
            equations.push_back(equation);
        }
    }
    return compute_result(&initial_vals, "z")
}

fn parse_input(file_path: &str) -> (HashMap<String, bool>, VecDeque<Equation>) {
    let mut initial_vals: HashMap<String, bool> = HashMap::new();
    let mut equations: VecDeque<Equation> = VecDeque::new();

    let mut on_equations: bool = false;
    println!("Cheated!  Found answer in excel by manually swapping nodes!");
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    on_equations = true
                } else if on_equations {
                    let (expr, mut out_node) = row.split_once(" -> ").unwrap();
                    let vals: Vec<&str> = expr.split(" ").collect();
                    let operator_id = match vals[1] {
                        "AND" => 0,
                        "OR" => 1,
                        "XOR" => 2,
                        _ => panic!("Invalid value!"),
                    };
                    // Check my answer my swapping these
                    //out_node = match out_node {
                    //    "dqr" => "z33",
                    //    "z33" => "dqr",
                    //    "pfw" => "z39",
                    //    "z39" => "pfw",
                    //    "shh" => "z21",
                    //    "z21" => "shh",
                    //    "vgs" => "dtk",
                    //    "dtk" => "vgs",
                    //    _ => out_node,
                    //};
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
