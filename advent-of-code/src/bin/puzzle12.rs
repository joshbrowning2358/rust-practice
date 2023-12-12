use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle12/example.txt";
    //let file_path = "data/puzzle12/input.txt";
    let file_path = "data/puzzle12/hard.txt";

    let mut ans = puzzle12a(file_path);
    println!("Answer to puzzle 12a is {ans};");

    ans = puzzle12b(file_path);
    println!("Answer to puzzle 12b is {ans};");
}

fn puzzle12a(file_path: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut counts: Vec<i32>;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (missing, counts_str) = ip.split_once(' ').unwrap();
                println!("\n\nMissing is {missing}, counts is {counts_str}\n\n");
                counts = counts_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                ans += get_possibilities(missing.to_string(), counts);
            }
        }
    }
    ans
}

fn puzzle12b(file_path: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut counts: Vec<i32>;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (missing, counts_str) = ip.split_once(' ').unwrap();

                // Duplicate missing
                let mut missing = missing.to_string();
                missing.push_str("?");
                missing = missing.repeat(5);
                missing.pop();

                // Duplicate counts
                counts = counts_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                counts = counts.repeat(5);
                //counts = counts.iter().cycle().take(counts.len() * 5).collect();
                println!("\n\nMissing is {missing}, counts is {:?}\n\n", counts);

                ans += get_possibilities(missing, counts);
            }
        }
    }
    ans
}

fn get_possibilities(mut row: String, cnts: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut contains_question: bool = false;

    //println!("Checking {row}");
    let mut obs_cnts: Vec<i32> = vec![];
    let mut current_cnt: i32 = 0;
    for c in row.chars() {
        if c == '#' {
            current_cnt += 1;
        } else if c == '?' {
            contains_question = true;
            break
        } else if current_cnt > 0 {
            obs_cnts.push(current_cnt);
            current_cnt = 0;
        }
    }
    if (current_cnt > 0) & (!contains_question) {
        obs_cnts.push(current_cnt);
    }

    //println!("Found {:?} for observed, have {:?} for expected", obs_cnts, cnts);
    if contains_question {
        // Don't branch if invalid
        if obs_cnts.len() > cnts.len() {
            return 0
        }
        if obs_cnts != &cnts[..obs_cnts.len()] {
            return 0
        }
        if (current_cnt > 0) & (cnts.len() > obs_cnts.len()) {
            if cnts[obs_cnts.len()] < current_cnt {
                return 0
            }
        }

        let idx = row.find('?').unwrap();
        row.replace_range(idx..(idx + 1), "#");
        ans += get_possibilities(row.clone(), cnts.clone());
        row.replace_range(idx..(idx + 1), ".");
        ans += get_possibilities(row.clone(), cnts.clone());
    } else {
        if obs_cnts == cnts {
            return 1
        } else {
            return 0
        }
    }
    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
