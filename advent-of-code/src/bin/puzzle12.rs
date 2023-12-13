use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle12/easy.txt";
    //let file_path = "data/puzzle12/example.txt";
    let file_path = "data/puzzle12/input.txt";
    //let file_path = "data/puzzle12/hard.txt";

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
                //println!("\n\nMissing is {missing}, counts is {counts_str}\n\n");
                counts = counts_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

                let new_val = get_possibilities(missing.to_string(), counts);
                //println!("Found {new_val} possibilities for row {ip}");
                ans += new_val;
            }
        }
    }
    ans
}

fn puzzle12b(file_path: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut counts: Vec<i32>;

    let mut row_cnt = 0;

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
                //println!("\n\nMissing is {missing}, counts is {:?}\n\n", counts);

                let new_val = get_possibilities(missing, counts);
                //println!("Found {new_val} possibilities for row {ip}");
                ans += new_val;
                row_cnt += 1;
                if row_cnt % 25 == 0 {println!("Finished row {row_cnt}!");}
            }
        }
    }
    ans
}

fn get_possibilities(mut row: String, cnts: Vec<i32>) -> i64 {
    //println!("Checking line {row} with counts {:?}", cnts);
    let mut ans = 0;
    let mut contains_question: bool = false;

    let mut obs_cnts: Vec<i32> = vec![];
    let mut current_cnt = 0;
    let mut chars = row.chars();
    loop {
        match chars.next() {
            Some(c) => {
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
            None => {break}
        };
    }
    if (current_cnt > 0) & (!contains_question) {
        obs_cnts.push(current_cnt);
    }

    if obs_cnts.len() > cnts.len() {
        return 0
    } else if obs_cnts != &cnts[..obs_cnts.len()] {  // Doesn't match required so far
        return 0
    } else if !contains_question {
        if obs_cnts == cnts {
            return 1
        } else {
            return 0
        }
    } else if (obs_cnts.len() < cnts.len()) & contains_question {
        let idx = row.find('?').unwrap();

        // Current streak is current_cnt
        let streak = current_cnt as usize;
        //println!("Counts are {:?} and obs_cnts are {:?}, current is {current_cnt}", cnts, obs_cnts);
        let next_cnt = cnts[obs_cnts.len()] as usize;
        if next_cnt == streak { // ? must be a .
            return get_possibilities(
                row.clone()[(idx + 1)..].to_string(),
                cnts[(obs_cnts.len() + 1)..].to_vec()
            );
        }

        // Can we fit a #### in the current position?
        let mut fits: bool = true;
        let mut future_streak = 0;
        while (future_streak + streak + 1) < next_cnt {
            match chars.next() {
                Some(x) => {
                    if x == '.' { // Having a # or ? is ok, a . is not
                        fits = false;
                        break
                    }
                    future_streak += 1;
                }
                None => {
                    fits = false;
                    break
                }
            };
        }
        // Next character must be a ? or . now (or end of string)
        match chars.next() {
            Some(x) => {
                if x== '#' {
                    fits = false;
                }
            }
            None => {}
        };

        if fits {
            let mut repl_str = "#".to_string().repeat(future_streak + 1);
            let mut new_row = row.clone();
            if (idx + repl_str.len()) < new_row.len() { // Append a . only if there's space
                repl_str.push_str(".");
            }
            new_row.replace_range(idx..(idx + repl_str.len()), &repl_str);
            //println!("Replacing with #, orig_row {}, new_row {} new cnts {:?}", new_row, new_row[(idx + repl_str.len())..].to_string(), cnts[(obs_cnts.len() + 1)..].to_vec());
            ans += get_possibilities(
                new_row[(idx + repl_str.len())..].to_string(),
                cnts[(obs_cnts.len() + 1)..].to_vec()
            );
        }
        let mut new_row = row.clone();
        new_row.replace_range(idx..(idx + 1), ".");
        if current_cnt > 0 {
            if current_cnt == cnts[obs_cnts.len()] {
                //println!("Replacing with a ., new_row {}, cnts {:?}", new_row[(idx + 1)..].to_string(), cnts[(obs_cnts.len() + 1)..].to_vec());
                ans += get_possibilities(
                    new_row[(idx + 1)..].to_string(),
                    cnts[(obs_cnts.len() + 1)..].to_vec()
                );
            }
        } else {
            ans += get_possibilities(
                new_row[(idx + 1)..].to_string(),
                cnts[obs_cnts.len()..].to_vec()
            );
        }
    } else { // Still contains ? but obs_cnts.len() == cnts.len().  If we have ##..#.??? this could work, for example, but not ##..#.?#?
        if (current_cnt > 0) | (obs_cnts != cnts) {
            return 0
        }
        loop { // Ensure no # after ?
            match chars.next() {
                Some(x) => {
                    if x == '#' {return 0}
                }
                None => break
            };
        }
        //println!("Success!  obs_cnts {:?} cnt {:?}\n", obs_cnts, cnts);
        return 1
    }
    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
