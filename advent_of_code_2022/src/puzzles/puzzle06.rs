use std::collections::HashMap;

use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle06;

impl puzzle_solver::Solver for Puzzle06 {
    fn part_1(&self, file_path: &str) -> String {
        let mut chars_used: HashMap<char, i32> = HashMap::new();
        let mut char_idx: usize = 0;
        if let Ok(lines) = file_reader::read_lines(file_path) {
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
        return char_idx.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let mut chars_used: HashMap<char, i32> = HashMap::new();
        let mut char_idx: usize = 0;
        if let Ok(lines) = file_reader::read_lines(file_path) {
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
        return char_idx.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1598", "2414")
    }

    fn name(&self) -> &'static str {
        "Day 6: Tuning Trouble"
    }
}
