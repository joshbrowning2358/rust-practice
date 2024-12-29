use regex::Regex;

use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let mut result: i32 = 0;

    let memory = parse_input(&file_path);
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for mem in memory {
        for (_, [left, right]) in pattern.captures_iter(&mem).map(|c| c.extract()) {
            result += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
    }
    return result
}

pub fn part_b(file_path: &str) -> i32 {
    // 100875786 is too high
    let mut result: i32 = 0;

    let memory = parse_input(&file_path);
    let outer_pattern = Regex::new(r"(^|do\(\)|don\'t\(\))([^d]*)").unwrap();
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mem = memory.join(".");
    for (_, [cond, mem_subset]) in outer_pattern.captures_iter(&mem).map(|c| c.extract()) {
        if cond != "don't()" {
            for (_, [left, right]) in pattern.captures_iter(&mem_subset).map(|c| c.extract()) {
                result += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
            }
        }
    }

    return result
}

fn parse_input(file_path: &str) -> Vec<String> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(memory) = line {
                result.push(memory)
            }
        }
    }

    return result;
}
