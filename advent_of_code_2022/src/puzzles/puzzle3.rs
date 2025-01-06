use std::collections::HashSet;
use std::env;

mod read_lines;

const UPPER_OFFSET: i32 = 65 - 27;
const LOWER_OFFSET: i32 = 97 - 1;
const SPLIT_VAL: i32 = 91;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let mut left: HashSet<char> = HashSet::new();
                let mut right: HashSet<char> = HashSet::new();
                for (i, curr_char) in ip.chars().enumerate() { 
                    if i < ip.len() / 2 {
                        left.insert(curr_char);
                    } else {
                        right.insert(curr_char);
                    }
                }
                let mut overlapping = left.intersection(&right);
                ans += value_letter(overlapping.next().unwrap());
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut first: HashSet<char> = HashSet::new();
    let mut second: HashSet<char> = HashSet::new();
    let mut third: HashSet<char> = HashSet::new();

    let mut row_cnt: i32 = 0;
    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                for curr_char in ip.chars() {
                    match row_cnt % 3 {
                        0 => {first.insert(curr_char);},
                        1 => {second.insert(curr_char);},
                        2 => {third.insert(curr_char);},
                        _ => {panic!("Invalid mod3 value!");}
                    }
                }
                row_cnt += 1;
                if row_cnt % 3 == 0 {
                    let mut sets: Vec<HashSet<char>> = vec![first.clone(), second.clone(), third.clone()];
                    let (intersection, others) = sets.split_at_mut(1);
                    let intersection = &mut intersection[0];
                    for other in others {
                        intersection.retain(|e| other.contains(e));
                    }
                    ans += value_letter(&intersection.drain().next().unwrap());
                    first.clear();
                    second.clear();
                    third.clear();
                }
            }
        }
    }
    ans
}

fn value_letter(c: &char) -> i32 {
    let mut val = *c as i32;
    if val > SPLIT_VAL {
        val -= LOWER_OFFSET;
    } else {
        val -= UPPER_OFFSET;
    }
    val
}
