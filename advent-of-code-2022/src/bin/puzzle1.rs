use std::collections::BinaryHeap;

mod read_lines;

fn main() {
    //let file_path = "data/puzzle1/example.txt";
    let file_path = "data/puzzle1/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut curr_cnt: i32 = 0;

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    if curr_cnt > ans {ans = curr_cnt;}
                    curr_cnt = 0;
                } else {
                    curr_cnt += ip.parse::<i32>().unwrap();
                }
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut curr_cnt: i32 = 0;
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();

    if let Ok(lines) = read_lines::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    max_heap.push(curr_cnt);
                    curr_cnt = 0;
                } else {
                    curr_cnt += ip.parse::<i32>().unwrap();
                }
            }
        }
    }

    let mut ans: i32 = 0;
    for _ in 0..3 {
        ans += max_heap.pop().unwrap();
    }
    ans
}
