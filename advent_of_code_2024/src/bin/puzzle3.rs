use regex::Regex;

use advent_of_code_2024::file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let mut ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
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

fn part_b(file_path: &str) -> i32 {
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
