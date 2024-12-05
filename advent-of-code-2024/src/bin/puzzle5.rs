use std::collections::HashMap;

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
    let (later, updates) = parse_input(file_path);

    let mut result = 0;
    'update: for update in updates {
        for left_ind in 0..update.len() - 1 {
            for right_ind in left_ind + 1..update.len() {
                let left_val = update[left_ind];
                let right_val = update[right_ind];
                if let Some(invalids) = later.get(&right_val) {
                    if invalids.contains(&left_val) {
                        continue 'update;
                    }
                }
            }
        }

        // Got here, must be valid!
        let middle_ind = (update.len() - 1) / 2;
        result += update[middle_ind]
    }

    return result
}

fn part_b(file_path: &str) -> i32 {
    let (later, updates) = parse_input(file_path);

    let mut result = 0;
    for mut update in updates {
        let valid = is_valid(&update, &later);

        if !valid {
            let mut fixed = false;
            while !fixed {
                'fixing: for left_ind in 0..update.len() - 1 {
                    for right_ind in left_ind + 1..update.len() {
                        let left_val = update[left_ind];
                        let right_val = update[right_ind];
                        if let Some(invalids) = later.get(&right_val) {
                            if invalids.contains(&left_val) {
                                update[left_ind] = right_val;
                                update[right_ind] = left_val;
                                break 'fixing;
                            }
                        }
                    }
                }
                fixed = is_valid(&update, &later);
            }

            let middle_ind = (update.len() - 1) / 2;
            result += update[middle_ind]
        }
    }

    return result
}

fn parse_input(file_path: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut later = HashMap::new();
    let mut updates = Vec::new();


    let mut reading_updates: bool = false;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    reading_updates = true;
                    continue
                }
                if reading_updates {
                    updates.push(row.split(',').map(|x| x.parse::<i32>().unwrap()).collect());
                }
                else {
                    let (left, right) = row.split_once('|').unwrap();
                    let left_int = left.parse::<i32>().unwrap();
                    let right_int = right.parse::<i32>().unwrap();
                    later.entry(left_int).and_modify(|val: &mut Vec<i32>| val.push(right_int)).or_insert(vec![right_int]);
                }
            }
        }
    }

    return (later, updates)
}

fn is_valid(update: &Vec<i32>, later: &HashMap<i32, Vec<i32>>) -> bool {
    let mut valid = true;
    for left_ind in 0..update.len() - 1 {
        for right_ind in left_ind + 1..update.len() {
            let left_val = update[left_ind];
            let right_val = update[right_ind];
            if let Some(invalids) = (*later).get(&right_val) {
                if invalids.contains(&left_val) {
                    valid = false;
                }
            }
        }
    }
    return valid
}
