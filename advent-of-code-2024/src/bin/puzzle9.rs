mod file_reader;

fn main() {
    //let file_path = "data/puzzle9/easy.txt";
    //let file_path = "data/puzzle9/example.txt";
    let file_path = "data/puzzle9/input.txt";
    //let file_path = "data/puzzle9/hard.txt";

    let mut ans = puzzle9a(file_path);
    println!("Answer to puzzle 9a is {ans};");

    ans = puzzle9b(file_path);
    println!("Answer to puzzle 9b is {ans};");
}

fn puzzle9a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let nums: Vec<i32> = ip.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                let mut last_diffs: [i32; 32] = [0; 32];  // 32 should be plenty
                last_diffs[0] = nums[nums.len() - 1];
                let mut curr_diffs: Vec<i32> = nums;
                let mut idx: usize = 1;
                loop {
                    let mut new_curr_diffs: Vec<i32> = vec![];
                    for i in 0..(curr_diffs.len() - 1) {
                        new_curr_diffs.push(curr_diffs[i + 1] - curr_diffs[i])
                    }
                    //println!("New curr_diffs are {:?}", new_curr_diffs);
                    curr_diffs = new_curr_diffs;
                    last_diffs[idx] = curr_diffs[curr_diffs.len() - 1];
                    if curr_diffs.iter().all(|&item| item == 0) {
                        break
                    }
                    idx += 1;
                }
                //println!("Row is {ip}, found {:?}", last_diffs);
                let curr_ans: i32 = last_diffs.iter().sum();
                ans += curr_ans;
            }
        }
    }
    ans
}

fn puzzle9b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let minus_one: i32 = -1;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let nums: Vec<i32> = ip.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                let mut first_diffs: [i32; 32] = [0; 32];  // 32 should be plenty
                first_diffs[0] = nums[0];
                let mut curr_diffs: Vec<i32> = nums;
                let mut idx: usize = 1;
                loop {
                    let mut new_curr_diffs: Vec<i32> = vec![];
                    for i in 0..(curr_diffs.len() - 1) {
                        new_curr_diffs.push(curr_diffs[i + 1] - curr_diffs[i])
                    }
                    //println!("New curr_diffs are {:?}", new_curr_diffs);
                    curr_diffs = new_curr_diffs;
                    first_diffs[idx] = curr_diffs[0] * minus_one.pow(idx.try_into().unwrap());
                    if curr_diffs.iter().all(|&item| item == 0) {
                        break
                    }
                    idx += 1;
                }
                let curr_ans: i32 = first_diffs.iter().sum();
                //println!("For row {ip} got answer {}", curr_ans);
                ans += curr_ans;
            }
        }
    }
    ans
}
