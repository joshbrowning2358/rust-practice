use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle12/easy.txt";
    //let file_path = "data/puzzle12/example.txt";
    let file_path = "data/puzzle12/input.txt";
    //let file_path = "data/puzzle12/hard.txt";

    let ans = puzzle12a(file_path);
    println!("Answer to puzzle 12a is {ans};");

    // Hard row: ?###???????? 3, 2, 1

    //let row = String::from("..??#??????..??#");
    //let cnts: Vec<i32> = vec![8, 2];
    //let row = String::from("?###????????");
    //let cnts: Vec<i32> = vec![3, 2, 1];
    //let ans = get_possibilities2(row, &cnts);
    //println!("Answer to simple example is {ans}");
    //let row = String::from("?###??????????###????????");
    //let cnts: Vec<i32> = vec![3, 2, 1, 3, 2, 1];
    //let ans = get_possibilities2(row, &cnts);
    //println!("Answer to simple example is {ans}");
    //let row = String::from("?###??????????###??????????###????????");
    //let cnts: Vec<i32> = vec![3, 2, 1, 3, 2, 1, 3, 2, 1];
    //let ans = get_possibilities2(row, &cnts);
    //println!("Answer to simple example is {ans}");
    //let row = String::from("?###??????????###??????????###??????????###??????????###????????");
    //let cnts: Vec<i32> = vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1];
    //let ans = get_possibilities2(row, &cnts);
    //println!("Answer to simple example is {ans}");

    let ans = puzzle12b(file_path);
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

                let new_val = get_possibilities2(missing, &counts);
                //println!("Found {new_val} possibilities for row {ip}");
                ans += new_val;
                row_cnt += 1;
                if row_cnt % 5 == 0 {println!("Finished row {row_cnt}!");}
            }
        }
    }
    ans
}

fn get_possibilities2(mut row: String, cnts: &Vec<i32>) -> i64 {
    //println!("Calling get_possibilities2 with {row} and {:?}", cnts);
    // Handle edge cases
    if row.len() == 0 {return 0}
    if row.len() < (cnts.iter().sum::<i32>() as usize) {return 0}
    if cnts.len() == 0 {
        if row.contains("#") {return 0}
        return 1
    }

    let mut ans: i64 = 0;
    row = simplify_row(&row, &cnts);
    if !row.contains("?") {
        //println!("No ? (row is >{row}< and cnts {:?}), evaluating!", cnts);
        let obs_cnts = get_counts_in_known_str(&row);
        if obs_cnts == *cnts {
            return 1
        } else {
            return 0
        }
    } else if row.contains('.') {
        //println!("Splitting on . in row >{row}< and cnts {:?}", cnts);
        let (left, right) = row.split_once('.').unwrap();
        for i in 0..(cnts.len() + 1) {
            let left_ans = get_possibilities2(left.to_string(), &(cnts[..i].to_vec()));
            if left_ans > 0 {
                let right_ans = get_possibilities2(right.to_string(), &(cnts[i..].to_vec()));
                ans += left_ans * right_ans;
            }
        }
        return ans
    } else {
        //println!("No . in row but ? (row is >{row}<), splitting left/right");
        let idx = row.find('?').unwrap();
        let mut left = row.clone();
        left.replace_range(idx..(idx + 1), &".");
        let mut right = row.clone();
        right.replace_range(idx..(idx + 1), &"#");
        return get_possibilities2(left.to_string(), &cnts) + get_possibilities2(right.to_string(), &cnts)
    }
}

fn simplify_row(row: &str, cnts: &Vec<i32>) -> String {
    // Remove any .'s from start or end as they don't help us fit in cnts
    let mut row = row.trim_matches('.').to_string();

    if cnts.len() == 0 {return row.to_string()}
    if row.len() < (cnts[0] as usize) {return "".to_string()}

    // Use logic to see if we can replace some ?'s with #'s
    let first_pound = match row.find('#') {
        Some(x) => {x},
        None => {1000}
    };
    
    let cnt: usize = cnts[0] as usize;
    if first_pound < cnt {
        // chars from first_pound to cnts[0] must all be #'s
        if row[first_pound..cnt].contains('.') {
            return "".to_string()  // Doesn't fit, this isn't valid.  Empty string should give 0 options later.
        } else {
            let replace_str = "#".repeat(cnt - first_pound);
            row.replace_range(first_pound..cnt, &replace_str);
        }
    }

    row.to_string()
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

fn get_counts_in_known_str(row: &str) -> Vec<i32> {
    let mut obs_cnts: Vec<i32> = vec![];
    let mut current_cnt = 0;
    for c in row.chars() {
        if c == '#' {
            current_cnt += 1;
        } else if c == '?' {
            panic!("Called get_counts_in_known_str with {row} and found '?'");
        } else if current_cnt > 0 {
            obs_cnts.push(current_cnt);
            current_cnt = 0;
        }
    }
    if current_cnt > 0 {
        obs_cnts.push(current_cnt);
    }
    obs_cnts
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
