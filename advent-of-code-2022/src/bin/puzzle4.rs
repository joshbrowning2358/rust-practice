use std::env;

mod read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

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
                let (first_range, second_range) = ip.split_once(",").unwrap();
                let (x0, x1) = first_range.split_once("-").unwrap();
                let (y0, y1) = second_range.split_once("-").unwrap();
                let x0 = x0.parse::<i32>().unwrap();
                let x1 = x1.parse::<i32>().unwrap();
                let y0 = y0.parse::<i32>().unwrap();
                let y1 = y1.parse::<i32>().unwrap();

                if x0 <= y0 && x1 >= y1 {
                    ans += 1;
                }
                else if y0 <= x0 && y1 >= x1 {
                    ans += 1;
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
                let (first_range, second_range) = ip.split_once(",").unwrap();
                let (x0, x1) = first_range.split_once("-").unwrap();
                let (y0, y1) = second_range.split_once("-").unwrap();
                let x0 = x0.parse::<i32>().unwrap();
                let x1 = x1.parse::<i32>().unwrap();
                let y0 = y0.parse::<i32>().unwrap();
                let y1 = y1.parse::<i32>().unwrap();

                if x0 <= y0 && x1 >= y0 {
                    println!("Found match, line is {:?}", ip);
                    ans += 1;
                }
                else if y0 <= x0 && y1 >= x0 {
                    println!("Found match, line is {:?}", ip);
                    ans += 1;
                }
            }
        }
    }
    ans
}
