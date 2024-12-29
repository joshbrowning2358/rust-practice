use num::abs;

use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let mut result: i32 = 0;
    let mut vals: Vec<i32>;
    let mut failed;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(report) = line {
                vals = report.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                failed = is_unsafe(vals);
                if !failed {
                    result += 1;
                }
            }
        }
    }
    return result
}

pub fn part_b(file_path: &str) -> i32 {
    let mut result: i32 = 0;
    let mut vals: Vec<i32>;
    let mut failed: bool;
    let mut any_passed: bool = false;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(report) = line {
                vals = report.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                let mut i: usize = 0;
                while !any_passed && i < vals.len() {
                    let new_vals = [&vals[0..i], &vals[(i + 1)..]].concat();
                    failed = is_unsafe(new_vals);
                    if !failed {
                        any_passed = true;
                    }
                    i += 1;
                }
                if any_passed {
                    result += 1;
                }
                any_passed = false;
            }
        }
    }
    return result
}

fn is_unsafe(report: Vec<i32>) -> bool {
    let mut inc_cnt = 0;
    let mut failed = false;
    for diffs in report.windows(2) {
        if diffs[0] < diffs[1] {
            if inc_cnt < 0 {
                failed = true;
                break
            } else {
                inc_cnt += 1;
            }
        }
        if diffs[0] > diffs[1] {
            if inc_cnt > 0 {
                failed = true;
                break
            } else {
                inc_cnt -= 1;
            }
        }
        if abs(diffs[1] - diffs[0]) > 3 || abs(diffs[1] - diffs[0]) < 1 {
            failed = true;
            break
        }
    } 
    return failed
}
