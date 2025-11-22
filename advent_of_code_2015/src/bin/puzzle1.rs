use std::fs;

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
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut floor: i32 = 0;
    for c in contents.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }
    return floor
}

fn part_b(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut idx: i32 = 0;
    let mut floor: i32 = 0;
    for c in contents.chars() {
        idx += 1;
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            break
        }
    }
    return idx
}

