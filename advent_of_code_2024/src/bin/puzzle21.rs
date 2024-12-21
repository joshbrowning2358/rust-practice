use std::collections::VecDeque;

use advent_of_code_2024::file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/easy1.txt");

    let ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    let ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let codes = parse_input(file_path);

    let mut result: i32 = 0;
    for code in codes {
        println!("Processing code {code:?}");
        let first_moves = get_button_moves(&code);
        println!("First robot options {first_moves:?}");
    }

    return result
}

fn part_b(file_path: &str) -> i32 {
    return 0
}

fn parse_input(file_path: &str) -> Vec<String> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                result.push(row);
            }
        }
    }

    return result
}

fn get_button_moves(code: &String) -> VecDeque<String> {
    let mut button_moves: VecDeque<String> = VecDeque::from([String::new()]);
    let mut start = 'A';
    
    for dest in code.chars() {
        let start_loc = get_numeric_position(start);
        let end_loc = get_numeric_position(dest);

        let x_delta = end_loc.0 - start_loc.0 as i32;
        let y_delta = end_loc.1 - start_loc.1 as i32;

        for _ in 0..(button_moves.len()) {
            let mut curr_seq = button_moves.pop_front().unwrap();
            if y_delta == 0 {
                for _ in 0..x_delta.abs() {
                    if x_delta > 0 {curr_seq.push_str(">");}
                    if x_delta < 0 {curr_seq.push_str("<");}
                }
                curr_seq.push_str("A");
                button_moves.push_back(curr_seq);
            } else if x_delta == 0 {
                for _ in 0..y_delta.abs() {
                    if y_delta > 0 {curr_seq.push_str("v");}
                    if y_delta < 0 {curr_seq.push_str("^");}
                }
                curr_seq.push_str("A");
                button_moves.push_back(curr_seq);
            } else if (x_delta != 0) & (y_delta != 0) {
                // Hard case, may have two paths but need to ensure we don't go over (0, 3)
                // Ignoring (0, 3) constraint first
                let mut alt_seq = curr_seq.clone();
                
                // Update curr_seq
                for _ in 0..x_delta.abs() {
                    if x_delta > 0 {curr_seq.push_str(">")}
                    if x_delta < 0 {curr_seq.push_str("<")}
                }
                for _ in 0..y_delta.abs() {
                    if y_delta > 0 {curr_seq.push_str("v");}
                    if y_delta < 0 {curr_seq.push_str("^");}
                }
                curr_seq.push_str("A");
                if (x_delta < 0) & (end_loc.0 == 0) & (start_loc.1 == 3) {
                    println!("Skipping as overlaps bad button");
                } else {
                    button_moves.push_back(curr_seq);
                }
                
                // Update alt_seq
                for _ in 0..y_delta.abs() {
                    if y_delta > 0 {alt_seq.push_str("v");}
                    if y_delta < 0 {alt_seq.push_str("^");}
                }
                for _ in 0..x_delta.abs() {
                    if x_delta > 0 {alt_seq.push_str(">")}
                    if x_delta < 0 {alt_seq.push_str("<")}
                }
                alt_seq.push_str("A");
                if (y_delta > 0) & (end_loc.1 == 3) & (start_loc.0 == 0) {
                    println!("Skipping as overlaps bad button");
                } else {
                    button_moves.push_back(alt_seq);
                }
            }
        }

        start = dest;
    }
    return button_moves
}

fn get_numeric_position(button: char) -> (i32, i32) {
    return match button {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!("Invalid button code {button}!")
    }
}
