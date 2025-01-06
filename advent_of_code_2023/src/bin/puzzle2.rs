use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "data/puzzle2/input.txt";
    //let file_path = "data/puzzle2/example.txt";
    let ans = puzzle2a(&file_path);
    println!("Answer to puzzle 2a is {ans}");

    let ans = puzzle2b(&file_path);
    println!("Answer to puzzle 2b is {ans}");
}

fn puzzle2b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut red_max: i32;
    let mut green_max: i32;
    let mut blue_max: i32;
    let mut red: i32;
    let mut green: i32;
    let mut blue: i32;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            red_max = 0;
            blue_max = 0;
            green_max = 0;
            if let Ok(ip) = line {
                for game_txt in ip.split([':', ';']) {
                    if !game_txt.starts_with("Game") {
                        (red, green, blue) = split_game(game_txt);
                        red_max = max(red, red_max);
                        blue_max = max(blue, blue_max);
                        green_max = max(green, green_max);
                    }
                }
            }
            ans += red_max * blue_max * green_max;
        }
    }

    ans
}

fn puzzle2a(file_path: &str) -> i32 {
    let mut ans = 0;
    let mut valid_game: bool;
    let mut game_id = String::from("");

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                valid_game = true;
                for game_txt in ip.split([':', ';']) {
                    if game_txt.starts_with("Game") {
                        game_id = game_txt.replace("Game ", "");
                    } else {
                        valid_game = valid_game && is_valid_game(game_txt);

                    }
                }
                if valid_game {
                    ans += game_id.parse::<i32>().unwrap();
                }
            }
        }
    }

    ans
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_valid_game(game_txt: &str) -> bool {
    let (red, green, blue) = split_game(game_txt);
    red <= 12 && green <= 13 && blue <= 14
}

fn split_game(game_txt: &str) -> (i32, i32, i32) {
    let mut green: i32 = 0;
    let mut red: i32 = 0;
    let mut blue: i32 = 0;
    let mut color_str: String;
    let mut num_str: String;
    let mut found_space: bool;

    for color_txt in game_txt.split(',') {
        color_str = String::from("");
        num_str = String::from("");
        found_space = false;

        for letter in color_txt.chars() {
            if letter == ' ' {
                if num_str.len() > 0 {  // Don't update otherwise, this might be the starting space
                    found_space = true
                }
            } else if found_space {
                color_str.push_str(&letter.to_string());
            } else {
                num_str.push_str(&letter.to_string());
            }
        }
            
        match color_str.as_str() {
            "blue" => {blue = num_str.parse::<i32>().unwrap();}
            "red" => {red = num_str.parse::<i32>().unwrap();}
            "green" => {green = num_str.parse::<i32>().unwrap();}
            &_ => {println!("Found invalid string in parsing: {color_str}!");}
        }
    }
    (red, green, blue)
}
