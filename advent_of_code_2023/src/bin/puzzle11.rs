use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle11/easy.txt";
    //let file_path = "data/puzzle11/example.txt"; let ncols = 10;
    let file_path = "data/puzzle11/input.txt"; let ncols = 140;
    //let file_path = "data/puzzle11/hard.txt";

    let ans = puzzle11a(file_path, ncols);
    println!("Answer to puzzle 11a is {ans};");

    let ans = puzzle11b(file_path, ncols);
    println!("Answer to puzzle 11b is {ans};");
}

fn puzzle11a(file_path: &str, ncols: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut galaxies: Vec<[i32; 2]> = Vec::new();
    let mut expand_rows: Vec<i32> = Vec::new();
    let mut expand_cols: Vec<i32> = (0..ncols).collect();

    let mut row: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains('#') {
                    let cols: Vec<i32> = ip.match_indices("#").map(|(i, _)| i as i32).collect();
                    for col in cols.iter() {
                        galaxies.push([row, *col]);
                        // Attempt to remove from expand_cols
                        let index = match expand_cols.iter().position(|x| x == col) {
                            Some(x) => {x},
                            None => {10000}
                        };
                        if index < 10000 {
                            expand_cols.remove(index);
                        }
                    }
                } else {
                    expand_rows.push(row);
                }
                row += 1;
            }
        }
    }

    println!("Initial galaxies are {:?}", galaxies);

    // Gravitational expansion
    let mut corrected_galaxies: Vec<[i32; 2]> = Vec::new();
    while galaxies.len() > 0 {
        let [mut curr_row, mut curr_col] = galaxies.pop().unwrap();
        curr_row += expand_rows.iter().filter(|&x| *x < curr_row).count() as i32;
        curr_col += expand_cols.iter().filter(|&x| *x < curr_col).count() as i32;
        corrected_galaxies.push([curr_row, curr_col]);
    }
    println!("Corrected galaxies are {:?}", corrected_galaxies);

    for i in 0..(corrected_galaxies.len() - 1) {
        for j in (i + 1)..(corrected_galaxies.len()) {
            let x_diff = corrected_galaxies[i][1] - corrected_galaxies[j][1];
            let y_diff = corrected_galaxies[i][0] - corrected_galaxies[j][0];
            ans += x_diff.abs() + y_diff.abs();
        }
    }

    ans
}

fn puzzle11b(file_path: &str, ncols: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut galaxies: Vec<[i32; 2]> = Vec::new();
    let mut expand_rows: Vec<i32> = Vec::new();
    let mut expand_cols: Vec<i32> = (0..ncols).collect();

    let mut row: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains('#') {
                    let cols: Vec<i32> = ip.match_indices("#").map(|(i, _)| i as i32).collect();
                    for col in cols.iter() {
                        galaxies.push([row, *col]);
                        // Attempt to remove from expand_cols
                        let index = match expand_cols.iter().position(|x| x == col) {
                            Some(x) => {x},
                            None => {10000}
                        };
                        if index < 10000 {
                            expand_cols.remove(index);
                        }
                    }
                } else {
                    expand_rows.push(row);
                }
                row += 1;
            }
        }
    }

    println!("Initial galaxies are {:?}", galaxies);

    // Gravitational expansion
    let mut corrected_galaxies: Vec<[i32; 2]> = Vec::new();
    while galaxies.len() > 0 {
        let [mut curr_row, mut curr_col] = galaxies.pop().unwrap();
        curr_row += (expand_rows.iter().filter(|&x| *x < curr_row).count() * 999999) as i32;
        curr_col += (expand_cols.iter().filter(|&x| *x < curr_col).count() * 999999) as i32;
        corrected_galaxies.push([curr_row, curr_col]);
    }
    println!("Corrected galaxies are {:?}", corrected_galaxies);

    for i in 0..(corrected_galaxies.len() - 1) {
        for j in (i + 1)..(corrected_galaxies.len()) {
            let x_diff = corrected_galaxies[i][1] - corrected_galaxies[j][1];
            let y_diff = corrected_galaxies[i][0] - corrected_galaxies[j][0];
            ans += (x_diff.abs() + y_diff.abs()) as i64;
        }
    }

    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
