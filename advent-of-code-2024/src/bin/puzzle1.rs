use num::abs;

mod file_reader;

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
    let (mut left, mut right) = parse_input(&file_path);
    left.sort();
    right.sort();

    let mut total = 0;
    for (left_val, right_val) in left.iter().zip(right.iter_mut()) {
        total += abs(*left_val - *right_val);
    }
    return total
}

fn part_b(file_path: &str) -> i32 {
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
