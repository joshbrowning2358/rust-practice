use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "data/puzzle2/input.txt";
    // let file_path = "data/puzzle2/example.txt";
    let ans = puzzle2a(&file_path);
    println!("Answer to puzzle 2a is {ans}");
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
    let mut green: i8 = 0;
    let mut red: i8 = 0;
    let mut blue: i8 = 0;
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
            "blue" => {blue = num_str.parse::<i8>().unwrap();}
            "red" => {red = num_str.parse::<i8>().unwrap();}
            "green" => {green = num_str.parse::<i8>().unwrap();}
            &_ => {println!("Found invalid string in parsing: {color_str}!");}
        }
    }
    red <= 12 && green <= 13 && blue <= 14
}
