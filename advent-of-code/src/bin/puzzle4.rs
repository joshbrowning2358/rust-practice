use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle4/input.txt";
    let file_path = "data/puzzle4/example.txt";
    let ans = puzzle4a(&file_path);
    println!("Answer to puzzle 4a is {ans}");

    let ans = puzzle4b(&file_path);
    println!("Answer to puzzle 4b is {ans}");
}

fn puzzle4a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut winning_nums: Vec<i32> = vec![];
    let mut line_score: i32;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                line_score = 0;
                let (_unused, sub_line) = match ip.split_once(':') {
                    Some((_unused, sub_line)) => {(_unused, sub_line)}
                    None => {
                        println!("Failed to find ':' in line!");
                        ("", "")
                    }
                };

                let (win_str, your_str) = match sub_line.split_once('|') {
                    Some((win, your)) => {(win, your)}
                    None => {
                        println!("Failed to find '|' in line!");
                        ("", "")
                    }
                };
                
                for win_num in win_str.split(" ") {
                    if win_num.len() > 0 { // Handle empty spaces
                        winning_nums.push(win_num.parse::<i32>().unwrap());
                    }
                }

                for your_num in your_str.split(" ") {
                    if your_num.len() > 0 {
                        let your_num = your_num.parse::<i32>().unwrap();
                        if winning_nums.contains(&your_num) {
                            if line_score == 0 {
                                line_score = 1
                            } else {
                                line_score *= 2;
                            }
                        }
                    }
                }
                
                winning_nums = vec![];
                //ans += line_score;

                let n_match = num_matches(&ip);
                println!("Found {n_match} matches!");
                if n_match > 0 {
                    line_score = 2_i32.pow((n_match - 1).try_into().unwrap());
                }
                ans += line_score;
            }
        }
    }
    ans
}

fn puzzle4b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut num_cards: Vec<i32> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                ans += 1;
            }
        }
    }
    ans += 1;
    ans
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//fn split_once_tuple(s: &str, c: char) -> (str, str) {
//    let (left, right) = match s.split_once(c) {
//        Some((lft, rgt)) => {(lft, rgt)}
//        None => {
//            println!("Failed to find {c} in line!");
//            ("", "")
//        }
//    };
//    (left, right)
//}

fn num_matches(line: &str) -> i32 {
    let mut winning_nums: Vec<i32> = vec![];
    let mut n_matches: i32 = 0;
    let (_unused, sub_line) = match line.split_once(':') {
        Some((_unused, sub_line)) => {(_unused, sub_line)}
        None => {
            println!("Failed to find ':' in line!");
            ("", "")
        }
    };

    let (win_str, your_str) = match sub_line.split_once('|') {
        Some((win, your)) => {(win, your)}
        None => {
            println!("Failed to find '|' in line!");
            ("", "")
        }
    };

    for win_num in win_str.split(" ") {
        if win_num.len() > 0 { // Handle empty spaces
            winning_nums.push(win_num.parse::<i32>().unwrap());
        }
    }

    for your_num in your_str.split(" ") {
        if your_num.len() > 0 {
            let your_num = your_num.parse::<i32>().unwrap();
            if winning_nums.contains(&your_num) {
                n_matches += 1;
            }
        }
    }
    n_matches
}
