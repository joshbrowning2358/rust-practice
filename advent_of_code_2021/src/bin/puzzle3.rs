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
    let mut cnts: [i32; 12] = [0; 12];
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(binary_str) = line {
                let mut idx = 0;
                for digit in binary_str.chars() {
                    match digit {
                        '0' => {
                            cnts[idx] -= 1;
                        },
                        '1' => {
                            cnts[idx] += 1;
                        }
                        _ => {panic!("Found invalid digit {digit}");}
                    }
                    idx += 1;
                }
            }
        }
    }

    // Compute gamma and epsilon based on how many bits are 0/1
    let mut gamma: i32 = 0;
    let mut eps: i32 = 0;
    let mut idx: u8 = 12;
    let two: i32 = 2;
    for digit in cnts.iter() {
        idx -= 1;
        if digit > &0 {
            gamma += two.pow(idx as u32);
        } else {
            eps += two.pow(idx as u32)
        }
    }
    // println!("Found gamma {gamma} and epsilon {eps}");
    return gamma * eps
}

fn part_b(file_path: &str) -> i32 {
    let mut num_candidates: i32 = 100000;
    let mut pattern = String::from("");
    let mut cnt: i32;
    let mut last_candidate = String::from("");

    while (num_candidates > 1) & (pattern.len() < 12) {
        num_candidates = 0;
        cnt = 0;
        if let Ok(lines) = file_reader::read_lines(file_path) {
            for line in lines {
                if let Ok(binary_str) = line {
                    if binary_str.starts_with(&pattern) {
                        match binary_str.chars().nth(pattern.len()).unwrap() {
                            '0' => {cnt -= 1;},
                            '1' => {cnt += 1;}
                            _ => {panic!("Found invalid digit!");}
                        }
                        num_candidates += 1;
                        last_candidate = binary_str;
                    }
                }
            }
            if cnt >= 0 {
                pattern.push_str(&"1");
            } else {
                pattern.push_str(&"0");
            }
        }
        // println!("Pattern is {pattern}, found {last_candidate}, count is {cnt}, num_candidates is {num_candidates}");
    }
    let a = i32::from_str_radix(&last_candidate, 2).unwrap();

    pattern = String::from("");
    num_candidates = 10000;
    while (num_candidates > 1) & (pattern.len() < 12) {
        num_candidates = 0;
        cnt = 0;
        if let Ok(lines) = file_reader::read_lines(file_path) {
            for line in lines {
                if let Ok(binary_str) = line {
                    if binary_str.starts_with(&pattern) {
                        match binary_str.chars().nth(pattern.len()).unwrap() {
                            '0' => {cnt -= 1;},
                            '1' => {cnt += 1;}
                            _ => {panic!("Found invalid digit!");}
                        }
                        num_candidates += 1;
                        last_candidate = binary_str;
                    }
                }
            }
            if cnt >= 0 {
                pattern.push_str(&"0");
            } else {
                pattern.push_str(&"1");
            }
        }
        // println!("Pattern is {pattern}, found {last_candidate}, count is {cnt}, num_candidates is {num_candidates}");
    }
    let b = i32::from_str_radix(&last_candidate, 2).unwrap();

    return a * b
}
