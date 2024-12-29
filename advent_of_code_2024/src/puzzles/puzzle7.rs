use crate::file_reader;

pub fn part_a(file_path: &str) -> i64 {
    // 27516147946705 is too low
    // 28730327770375
    let equations = parse_input(file_path);
    let mut result: i64 = 0;
    for equation in equations.iter() {
        if check_eq(equation.0, &equation.1) {
            result += equation.0; 
        }
    }

    return result
}

pub fn part_b(file_path: &str) -> i64 {
    let equations = parse_input(file_path);
    let mut result: i64 = 0;
    for equation in equations.iter() {
        if check_eq_b(equation.0, &equation.1) {
            result += equation.0;
        }
    }

    return result
}

fn parse_input(file_path: &str) -> Vec<(i64, Vec<i64>)> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(equation) = line {
                let (ans_str, nums_str) = equation.split_once(": ").unwrap();
                let ans = ans_str.parse::<i64>().unwrap();
                let nums = nums_str.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
                result.push((ans, nums));
            }
        }
    }

    return result;
}

fn check_eq(ans: i64, vals: &Vec<i64>) -> bool {
    for digit_check in (0..2_i32.pow((vals.len() - 1) as u32)).rev() {
        let mut possible_ans = vals[0];
        for i in 1..vals.len() {
            if digit_check / 2_i32.pow((i - 1) as u32) % 2 == 1 {
                possible_ans *= vals[i];
            } else {
                possible_ans += vals[i];
            }
        }
        if possible_ans == ans {
            return true
        }
    }
    return false
}

fn check_eq_b(ans: i64, vals: &Vec<i64>) -> bool {
    for digit_check in (0..3_i32.pow((vals.len() - 1) as u32)).rev() {
        let mut possible_ans = vals[0];
        for i in 1..vals.len() {
            if digit_check / 3_i32.pow((i - 1) as u32) % 3 == 1 {
                possible_ans *= vals[i];
            } else if digit_check / 3_i32.pow((i - 1) as u32) % 3 == 2 {
                let n_digits = vals[i].checked_ilog10().unwrap_or(0) + 1;
                possible_ans = possible_ans * 10_i64.pow(n_digits as u32) + vals[i];
            } else {
                possible_ans += vals[i];
            }
        }
        if possible_ans == ans {
            return true
        }
    }
    return false
}
