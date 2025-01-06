use std::collections::HashMap;
use std::env;

mod read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    part_a(file_path);
    part_b(file_path);
}

fn part_a(file_path: &str) {
    let mut chars_used: HashMap<char, i32> = HashMap::new();
    let mut char_idx: usize = 0;
    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let mut front_iter = ip.chars();
                let mut back_iter = ip.chars();
                for _ in 0..4 {
                    char_idx += 1;
                    let current_char = front_iter.next().unwrap();
                    let current_cnt = match chars_used.get_mut(&current_char) {
                        Some(x) => {*x},
                        _ => {0}
                    };
                    chars_used.insert(current_char, current_cnt + 1);
                }
                loop {
                    char_idx += 1;

                    // Add front of line
                    let current_char = front_iter.next().unwrap();
                    let current_cnt = match chars_used.get_mut(&current_char) {
                        Some(x) => {*x},
                        _ => {0}
                    };
                    chars_used.insert(current_char, current_cnt + 1);

                    // Drop back of line
                    let current_char = back_iter.next().unwrap();
                    let current_cnt = *chars_used.get_mut(&current_char).unwrap();
                    chars_used.insert(current_char, current_cnt - 1);

                    if *chars_used.values().max().unwrap() == 1 {
                        break;
                    }
                }
            }
        }
    }
    println!("Scanned {char_idx} characters!");
}

fn part_b(file_path: &str) {
    let mut chars_used: HashMap<char, i32> = HashMap::new();
    let mut char_idx: usize = 0;
    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let mut front_iter = ip.chars();
                let mut back_iter = ip.chars();
                for _ in 0..14 {
                    char_idx += 1;
                    let current_char = front_iter.next().unwrap();
                    let current_cnt = match chars_used.get_mut(&current_char) {
                        Some(x) => {*x},
                        _ => {0}
                    };
                    chars_used.insert(current_char, current_cnt + 1);
                }
                loop {
                    char_idx += 1;

                    // Add front of line
                    let current_char = front_iter.next().unwrap();
                    let current_cnt = match chars_used.get_mut(&current_char) {
                        Some(x) => {*x},
                        _ => {0}
                    };
                    chars_used.insert(current_char, current_cnt + 1);

                    // Drop back of line
                    let current_char = back_iter.next().unwrap();
                    let current_cnt = *chars_used.get_mut(&current_char).unwrap();
                    chars_used.insert(current_char, current_cnt - 1);

                    if *chars_used.values().max().unwrap() == 1 {
                        break;
                    }
                }
            }
        }
    }
    println!("Scanned {char_idx} characters!");
}
