use std::env;

mod read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    //let file_path = "data/puzzle/input.txt";

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
                ans += ip.len() as i32;
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    0
}
