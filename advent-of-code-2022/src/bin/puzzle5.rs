use std::collections::VecDeque;
use std::env;

mod read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    part_a(file_path);

    part_b(file_path);
}

fn part_a(file_path: &str) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut n_stacks: usize;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if stacks.len() == 0 {
                    n_stacks = (ip.len() + 1) / 4;
                    stacks = initialize_stacks(n_stacks);
                }
                if ip.starts_with("move") {
                    let (to_move, from, to) = parse_move_line(&ip);
                    for _ in 0..to_move {
                        let moved_element = stacks[from - 1].pop_back().unwrap();
                        stacks[to - 1].push_back(moved_element);
                    }
                } else {
                    for (i, c) in ip.chars().enumerate() {
                        if i % 4 == 1 && c != ' ' {stacks[(i - 1) / 4 as usize].push_front(c)}
                    }
                }
            }
        }
    }
    println!("Final stacks {:?}", stacks);
}

fn initialize_stacks(n_stacks: usize) -> Vec<VecDeque<char>> {
    let mut output: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..n_stacks {
        output.push(VecDeque::new());
    }
    output
}

fn parse_move_line(line: &str) -> (usize, usize, usize) {
    let (s1, s2) = line.split_once(" from ").unwrap();
    let (s2, s3) = s2.split_once(" to ").unwrap();
    let s1 = s1.replace("move ", "");
    (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap(), s3.parse::<usize>().unwrap())
}

fn part_b(file_path: &str) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut n_stacks: usize;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if stacks.len() == 0 {
                    n_stacks = (ip.len() + 1) / 4;
                    stacks = initialize_stacks(n_stacks);
                }
                if ip.starts_with("move") {
                    let (to_move, from, to) = parse_move_line(&ip);
                    let split_pt = stacks[from - 1].len() - to_move;
                    let mut moved_elements = stacks[from - 1].split_off(split_pt);
                    stacks[to - 1].append(&mut moved_elements);
                } else {
                    for (i, c) in ip.chars().enumerate() {
                        if i % 4 == 1 && c != ' ' {stacks[(i - 1) / 4 as usize].push_front(c)}
                    }
                }
            }
        }
    }
    println!("Final stacks {:?}", stacks);
}
