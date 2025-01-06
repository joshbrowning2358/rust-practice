mod file_reader;

fn main() {
    let file_name = file!().split('/').last().unwrap();
    let file_path = "data/puzzle1/input.txt";
    // let file_path = "data/puzzle1/easy.txt";

    println!("File path used: {file_path:?}");
    let mut ans = part_a(&file_path);
    println!("Answer to {file_name:?} a is {ans};");

    ans = part_b(&file_path);
    println!("Answer to {file_name:?} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut total: i32 = 0;
    let mut last: i32 = 10000;
    let mut current: i32;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            total += 1;
            if let Ok(ip) = line {
                current = ip.parse::<i32>().unwrap();
                if current > last {
                    ans += 1;
                }
                last = current;
            }
        }
    }
    println!("Parsed a total of {total} lines!");
    return ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut total: i32 = 0;
    let mut last: i32 = 10000;
    let mut current: i32;
    let mut lags: [i32; 3] = [0; 3];
    let mut parsed_val: i32;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            total += 1;
            if let Ok(ip) = line {
                parsed_val = ip.parse::<i32>().unwrap();
                if total == 1 {
                    lags[0] = parsed_val
                } else if total == 2 {
                    lags[1] = parsed_val
                } else if total == 3 {
                    lags[2] = parsed_val;
                    last = lags.iter().sum();
                } else {
                    lags[0] = lags[1];
                    lags[1] = lags[2];
                    lags[2] = parsed_val;
                    current = lags.iter().sum();
                    if current > last {
                        ans += 1;
                    }
                    last = current;
                }
            }
        }
    }
    return ans
}
