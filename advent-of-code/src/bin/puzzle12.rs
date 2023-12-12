use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle12/example.txt";
    let file_path = "data/puzzle12/input.txt";

    let ans = puzzle12a(file_path);
    println!("Answer to puzzle 12 a is {ans};");
}

fn puzzle12a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut counts: Vec<i32>;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (missing, counts_str) = ip.split_once(' ').unwrap();
                //println!("\n\nMissing is {missing}, counts is {counts_str}\n\n");
                counts = counts_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                ans += get_possibilities(missing.to_string(), counts);
            }
        }
    }
    ans
}

fn get_possibilities(mut row: String, cnts: Vec<i32>) -> i32 {
    let mut ans = 0;

    if !row.contains('?') {
        //println!("Checking {row}");
        // No unknown values, evaluate!
        let mut obs_cnts: Vec<i32> = vec![];
        let mut current_cnt: i32 = 0;
        for c in row.chars() {
            if c == '#' {
                current_cnt += 1;
            } else if current_cnt != 0 {
                obs_cnts.push(current_cnt);
                current_cnt = 0;
            }
        }
        if current_cnt > 0 {
            obs_cnts.push(current_cnt);
        }

        //println!("Found {:?} for observed, have {:?} for expected", obs_cnts, cnts);
        if obs_cnts == cnts {
            return 1
        } else {
            return 0
        }
    } else {
        let idx = row.find('?').unwrap();
        row.replace_range(idx..(idx + 1), "#");
        ans += get_possibilities(row.clone(), cnts.clone());
        row.replace_range(idx..(idx + 1), ".");
        ans += get_possibilities(row.clone(), cnts.clone());
    }
    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
