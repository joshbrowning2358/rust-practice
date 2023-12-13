use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle13/easy.txt";
    //let file_path = "data/puzzle13/example.txt";
    let file_path = "data/puzzle13/input.txt";
    //let file_path = "data/puzzle13/hard.txt";

    let mut ans = puzzle13a(file_path);
    println!("Answer to puzzle 13a is {ans};");

    ans = puzzle13b(file_path);
    println!("Answer to puzzle 13b is {ans};");
}

fn puzzle13a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut current_pattern = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 1 {
                    current_pattern.push_str(&ip.to_string());
                    nrows += 1;
                } else {
                    ans += find_reflections(&current_pattern, nrows, 0);
                    current_pattern.clear();
                    nrows = 0;
                }
            }
        }
    }
    ans += find_reflections(&current_pattern, nrows, 0);
    ans
}

fn puzzle13b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut current_pattern = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 1 {
                    current_pattern.push_str(&ip.to_string());
                    nrows += 1;
                } else {
                    ans += find_reflections(&current_pattern, nrows, 1);
                    current_pattern.clear();
                    nrows = 0;
                }
            }
        }
    }
    ans += find_reflections(&current_pattern, nrows, 1);
    ans
}

fn find_reflections(pattern: &str, nrows: usize, target_diff: i32) -> i32 {
    println!("Pattern is {pattern}");
    let mut ans: i32 = 0;
    let arr: Vec<char> = pattern.chars().collect();
    let ncols: usize = arr.len() / nrows;
    let mut n_mismatch: i32 = 0;

    for reflect_row in 1..nrows {
        let max_offset = min(reflect_row, nrows - reflect_row);
        'row_offsets: for offset_row in 1..(max_offset + 1) {
            for col in 0..ncols {
                let idx_1 = (reflect_row - offset_row) * ncols + col;
                let idx_2 = (reflect_row + offset_row - 1) * ncols + col;
                //println!("Checking reflect_row {reflect_row} offset_row {offset_row} col {col} idx1 {idx_1} idx2 {idx_2} nrows {nrows} ncols {ncols}");
                if arr[idx_1] != arr[idx_2] {
                    n_mismatch += 1;
                }
                if n_mismatch > target_diff {
                    break 'row_offsets
                }
            }
        }
        if n_mismatch == target_diff {println!("Found reflection on row {reflect_row}");}
        if n_mismatch == target_diff {ans += 100 * reflect_row as i32;}
        n_mismatch = 0;
    }

    for reflect_col in 1..ncols {
        let max_offset = min(reflect_col, ncols - reflect_col);
        'col_offsets: for offset_col in 1..(max_offset + 1) {
            for row in 0..nrows {
                let idx_1 = row * ncols + (reflect_col - offset_col);
                let idx_2 = row * ncols + (reflect_col + offset_col - 1);
                //println!("Checking reflect_row {reflect_row} offset_row {offset_row} col {col} idx1 {idx_1} idx2 {idx_2} nrows {nrows} ncols {ncols}");
                if arr[idx_1] != arr[idx_2] {
                    n_mismatch += 1;
                }
                if n_mismatch > target_diff {
                    break 'col_offsets;
                }
            }
        }
        if n_mismatch == target_diff {println!("Found reflection on col {reflect_col}");}
        if n_mismatch == target_diff {ans += reflect_col as i32;}
        n_mismatch = 0;
    }

    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
