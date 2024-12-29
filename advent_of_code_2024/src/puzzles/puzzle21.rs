use std::collections::VecDeque;

use memoize::memoize;

use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let codes = parse_input(file_path);

    let mut result: i32 = 0;
    for code in codes {
        let first_moves = get_button_moves(&code);
        let mut second_moves: VecDeque<String> = VecDeque::new();
        for code in first_moves {
            second_moves.append(&mut get_dir_moves(&code, true));
        }
        let mut third_moves: VecDeque<String> = VecDeque::new();
        for code in second_moves {
            third_moves.append(&mut get_dir_moves(&code, true));
        }
        let mut min: usize = 99999;
        for code in &third_moves {
            if code.len() < min {
                min = code.len();
            }
        }
        result += (min as i32) * to_num(code);
    }

    return result
}

pub fn part_b(file_path: &str) -> i64 {
    let codes = parse_input(file_path);

    let mut result: i64 = 0;
    for code in codes {
        let first_moves = get_button_moves(&code);
        let mut best: i64 = 0;
        for first_move in &first_moves {
            let mut last_char = 'A';
            let mut curr_total = 0;
            for next_char in first_move.chars() {
                let increment = count_pushes(last_char, next_char, 24);
                curr_total += increment;
                last_char = next_char;
            }
            if (curr_total < best) | (best == 0) {
                best = curr_total;
            }
        }
        result += best * (to_num(code) as i64);
    }

    return result
}

#[memoize]
fn count_pushes(start: char, end: char, depth: usize) -> i64 {
    // Find path from start to end and then recurse
    // Example: start = "<", end = "A", depth = 3.  Paths are:
    // >^> => count_pushes('>', '^', 2) + count_pushes('^', '>', 2)
    // >>^ => count_pushes('>', '>', 2) + count_pushes('>', '^', 2)
    let mut code = String::from(start);
    code.push(end);
    let pushes = get_dir_moves(&code, false);

    if depth == 0 {
        return *pushes.iter().map(|x| x.len() as i64).collect::<Vec<i64>>().iter().min().unwrap()
    }

    let mut best: i64 = 0;
    for push in pushes {
        let mut last_char = 'A';
        let mut curr_total = 0;
        for next_char in push.chars() {
            curr_total += count_pushes(last_char, next_char, depth - 1);
            last_char = next_char;
        }
        if (curr_total < best) | (best == 0) {
            best = curr_total;
        }
    }

    return best
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
                if !((x_delta < 0) & (end_loc.0 == 0) & (start_loc.1 == 3)) {
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
                if !((y_delta > 0) & (end_loc.1 == 3) & (start_loc.0 == 0)) {
                    button_moves.push_back(alt_seq);
                }
            }
        }

        start = dest;
    }
    return button_moves
}

fn get_dir_moves(code: &String, start_a: bool) -> VecDeque<String> {
    let mut button_moves: VecDeque<String> = VecDeque::from([String::new()]);
    let mut start = 'A';
    if !start_a {
        start = code.chars().next().unwrap();
    }
    
    let mut slice = &code[1..];
    if start_a {
        slice = &code[0..];
    }
    for dest in slice.chars() {
        let start_loc = get_dir_position(start);
        let end_loc = get_dir_position(dest);

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
                if !((x_delta < 0) & (end_loc.0 == 0) & (start_loc.1 == 0)) {
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
                if !((y_delta < 0) & (end_loc.1 == 0) & (start_loc.0 == 0)) {
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

fn get_dir_position(button: char) -> (i32, i32) {
    return match button {
        'A' => (2, 0),
        '^' => (1, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("Invalid button code {button}!")
    }
}

fn to_num(code: String) -> i32 {
    if code.starts_with('0') {
        return code[1..3].to_string().parse::<i32>().unwrap()
    } else {
        return code[0..3].to_string().parse::<i32>().unwrap()
    }
}
