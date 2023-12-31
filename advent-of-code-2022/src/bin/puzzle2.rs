use std::env;

mod read_lines;

const OFFSET: i32 = 23;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    //let file_path = "data/puzzle2/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (theirs, yours) = ip.split_once(' ').unwrap();
                let yours = yours.chars().next().unwrap() as i32 - OFFSET;
                let theirs = theirs.chars().next().unwrap() as i32;
                if yours == theirs {
                    ans += 3  // draw
                } else if yours == theirs + 1 || yours == theirs - 2 {
                    ans += 6 // win
                }
                ans += match yours {
                    65 => {1},  // rock
                    66 => {2},  // paper
                    67 => {3},  //scissors
                    _ => {panic!("Invalid value!");}
                }
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (theirs, yours) = ip.split_once(' ').unwrap();
                let yours = yours.chars().next().unwrap() as i32;
                let theirs = theirs.chars().next().unwrap() as i32;
                ans += match yours {
                    88 => {0},  // lose
                    89 => {3},  // draw
                    90 => {6},  // win
                    _ => {panic!("Invalid value!");}
                };
                
                let mut your_shape = theirs + (yours - 89);
                if your_shape < 65 {your_shape += 3;}
                if your_shape > 67 {your_shape -= 3;}
                ans += match your_shape {
                    65 => {1},  // rock
                    66 => {2},  // scissors
                    67 => {3},  // paper
                    _ => {panic!("Found invalid value for your_shape!");}
                };
            }
        }
    }
    ans
}
