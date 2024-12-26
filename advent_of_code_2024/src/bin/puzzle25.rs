use advent_of_code_2024::file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let (keys, locks) = parse_input(file_path);

    let mut result: i32 = 0;
    for key in &keys {
        for lock in &locks {
            let mut valid: bool = true;
            for idx in 0..5 {
                if key[idx] + lock[idx] > 7 {
                    valid = false;
                }
            }
            if valid {
                result += 1;
            }
        }
    }

    return result
}

fn parse_input(file_path: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys: Vec<Vec<u8>> = Vec::new();
    let mut locks: Vec<Vec<u8>> = Vec::new();

    let mut new_key_or_lock = vec![0; 5];
    let mut is_key: bool = false;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for (line_idx, line) in lines.enumerate() {
            if let Ok(row) = line {
                if line_idx % 8 == 7 {
                    if is_key {
                        keys.push(new_key_or_lock);
                    } else {
                        locks.push(new_key_or_lock);
                    }
                    new_key_or_lock = vec![0; 5];
                    continue
                }
                if line_idx % 8 == 0 {
                    is_key = row != "#####";
                }
                for (idx, c) in row.chars().enumerate() {
                    if c == '#' {
                        new_key_or_lock[idx] += 1;
                    }
                }
            }
        }
    }

    return (keys, locks)
}
