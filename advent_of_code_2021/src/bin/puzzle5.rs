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
    // let tuples = parse_input(file_path, false);
    let tuples = parse_input(file_path, true);

    let mut ans: i32 = 0;

    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for tuple in tuples {
        let mut left_x = tuple[0];
        let mut left_y = tuple[1];
        let right_x = tuple[2];
        let right_y = tuple[3];
        let delta_x = if left_x == right_x {0} else {if left_x > right_x {-1} else {1}}; 
        let delta_y = if left_y == right_y {0} else {if left_y > right_y {-1} else {1}};

        // Move left tuple to right tuple, one step at a time, adding to hashmap
        while (left_x != right_x) | (left_y != right_y) {
            *counts.entry((left_x, left_y)).or_insert(0) += 1;
            left_x += delta_x;
            left_y += delta_y;
        }
        *counts.entry((right_x, right_y)).or_insert(0) += 1;
    }

    for (_k, v) in &counts {
        if v > &1 {
            ans += 1;
        }
    }

    return ans
}

fn part_b(file_path: &str) -> i32 {
    println!("FIle path is {:?}", file_path);
    return 0
}

fn parse_input(file_path: &str, include_diag: bool) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(pipes) = line {
                let (left_tup, right_tup) = pipes.split_once(" -> ").unwrap();
                let (left_x, left_y) = left_tup.split_once(",").unwrap();
                let (right_x, right_y) = right_tup.split_once(",").unwrap();

                if include_diag | ((*left_x == *right_x) | (left_y == right_y)) {
                    result.push(
                        vec![
                            left_x.parse::<i32>().unwrap(),
                            left_y.parse::<i32>().unwrap(),
                            right_x.parse::<i32>().unwrap(),
                            right_y.parse::<i32>().unwrap()
                        ]
                    );
                }
            }
        }
    }

    return result;
}
