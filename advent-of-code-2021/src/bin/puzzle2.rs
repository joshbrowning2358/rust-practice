mod file_reader;

fn main() {
    let file_name = file!();
    let file_path = "data/puzzle2/input.txt";

    let mut ans = part_a(file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut cnt: i32;
    let mut depth: i32 = 0;
    let mut dist: i32 = 0;

    if let Ok(lines) = file_reader::read_lines(file_path) {
       for line in lines {
           if let Ok(command) = line {
               let (dir, cnt_str) = command.split_once(" ").unwrap();
               cnt = cnt_str.parse::<i32>().unwrap();
               match dir {
                   "down" => {
                       depth += cnt;
                    },
                    "up" => {
                        depth -= cnt;
                    },
                    "forward" => {
                        dist += cnt;
                    }
                    _ => {panic!("Invalid char {dir} found in parsing!");}
               }
           }
       }
    }
    return depth * dist
}

fn part_b(file_path: &str) -> i32 {
    let mut cnt: i32;
    let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(lines) = file_reader::read_lines(file_path) {
       for line in lines {
           if let Ok(command) = line {
               let (dir, cnt_str) = command.split_once(" ").unwrap();
               cnt = cnt_str.parse::<i32>().unwrap();
               match dir {
                   "down" => {
                       aim += cnt;
                    },
                    "up" => {
                        aim -= cnt;
                    },
                    "forward" => {
                        dist += cnt;
                        depth += cnt * aim;
                    }
                    _ => {panic!("Invalid char {dir} found in parsing!");}
               }
           }
       }
    }
    return depth * dist
}
