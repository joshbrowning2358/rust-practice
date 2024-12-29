use num::abs;

// use advent_of_code_2024::file_reader;
use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let (mut left, mut right) = parse_input(&file_path);
    left.sort();
    right.sort();

    let mut total = 0;
    for (left_val, right_val) in left.iter().zip(right.iter_mut()) {
        total += abs(*left_val - *right_val);
    }
    return total
}

pub fn part_b(file_path: &str) -> i32 {
    let (mut left, mut right) = parse_input(&file_path);
    left.sort();
    right.sort();

    let mut total = 0;
    for l in left.iter() {
        // let cnt = l * (right.iter().filter(|&n| *n == *l).count() as i32);
        // println!("Val is {l} and count is {cnt}");
        total += l * (right.iter().filter(|&n| *n == *l).count() as i32);
    }
    return total
}

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(pipes) = line {
                let (left_val, right_val) = pipes.split_once("   ").unwrap();
                left.push(left_val.parse::<i32>().unwrap());
                right.push(right_val.parse::<i32>().unwrap());
            }
        }
    }

    return (left, right)
}
