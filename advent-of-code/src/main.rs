use std::fs;
use std::cmp::min;
use std::collections::VecDeque;

const DIGIT_NAMES: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let file_path = "./data/puzzle1a.txt";
    //let file_path = "./data/example.txt";
    //let file_path = "./data/hard.txt";
    let mut ans: i32 = puzzle1a(&file_path);
    
    println!("Answer is {ans}!");

    ans = puzzle1b(&file_path);
    println!("Answer to second puzzle is {ans}");
}

fn puzzle1a(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut ans: i32 = 0;
    let mut first = String::from("");
    let mut last = String::from("");

    for letter in contents.chars() {
        if letter.is_numeric() {
            if first.len() == 0 {
                first.push_str(&letter.to_string());
                last.push_str(&letter.to_string());
            } else {
                last = letter.to_string();
            };
        } else if letter == '\n' {
            ans += (first + &last).parse::<i32>().unwrap();
            first = String::from("");
            last = String::from("");
        }
    }

    ans
}

fn puzzle1b(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Read file");

    let mut ans : i32 = 0;
    let mut first = String::from("");
    let mut last = String::from("");
    let mut match_queue: VecDeque<char> = VecDeque::new();
    let mut partial_match: bool;
    let mut full_match: bool;

    for letter in contents.chars() {
        if letter.is_alphabetic() {
            // Does match_queue + letter match anything?
            // If not, does match_queue[1:] + letter match anything?
            // ...
            loop {
                let mut s: String = (&match_queue).into_iter().collect();
                s.push_str(&letter.to_string());
                (partial_match, full_match) = match_digit(&s);
                if full_match || partial_match {
                    match_queue.push_back(letter);
                    break
                } else if match_queue.len() > 0 {
                    match_queue.pop_front();
                } else {
                    break
                }
            }

            if full_match {
                let s: String = (&match_queue).into_iter().collect();
                let digit_as_letter: char = match s.as_str() {
                    "one" => '1',
                    "two" => '2',
                    "three" => '3',
                    "four" => '4',
                    "five" => '5',
                    "six" => '6',
                    "seven" => '7',
                    "eight" => '8',
                    "nine" => '9',
                    "zero" => '0',
                    _ => {  // Failed to match, this shouldn't happen
                        println!("Failed to find match with {s}!");
                        '0'
                    }
                };
                if first.len() == 0 {
                    first = digit_as_letter.to_string();
                    last = digit_as_letter.to_string();
                } else {
                    last = digit_as_letter.to_string();
                }
                match_queue.pop_front();
            }

        } else if letter.is_numeric() {
            match_queue.clear();
            if first.len() == 0 {
                first.push_str(&letter.to_string());
                last.push_str(&letter.to_string());
            } else {
                last = letter.to_string();
            }
        } else if letter == '\n' {
            match_queue.clear();
            let result: i32 = first.parse::<i32>().unwrap() * 10 + last.parse::<i32>().unwrap();
            ans += result;
            first = String::from("");
            last = String::from("");
        }
    }

    ans
}

fn match_digit(s: &str) -> (bool, bool) {
    // Checks if passed string matches any of the digit names
    // Returns two values:
    // - Does the full string partially match?
    // - Does the full string fully match?
    let mut match_len: usize;
    let mut matched: bool;

    let mut output_partial: bool = false;
    let mut output_full: bool = false;

    for digit_name in DIGIT_NAMES {
        match_len = min(DIGIT_NAMES.len(), s.len());
        matched = true;

        for (c1, c2) in digit_name.chars().zip(s.chars()) {
            if c1 != c2 {
                matched = false;
            }
        }
        output_partial = matched || output_partial;
        if matched && match_len == digit_name.len() {
            output_full = true;
        }
    }
    (output_partial, output_full)
}
