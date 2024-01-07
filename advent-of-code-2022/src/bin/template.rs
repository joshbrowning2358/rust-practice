use std::env;

mod read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    part_a(file_path);
    part_b(file_path);
}

fn part_a(file_path: &str) {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                ans += ip.len() as i32;
            }
        }
    }
    println!("Answer to part 1 is {ans}");
}

fn part_b(file_path: &str) {
    println!("Answer to part 2 is {}", file_path.len());
}
