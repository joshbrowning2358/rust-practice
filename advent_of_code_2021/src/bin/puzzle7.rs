mod file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let mut ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i64 {
    let crab_pos = parse_input(file_path);
    let mut best: i64 = 99999999999999;
    let mut center: i32 = 0;
    let mut curr: i64;

    loop {
        curr = 0;
        for crab in &crab_pos {
            curr += (crab - center).abs() as i64;
        }
        if curr < best {
            center += 1;
            best = curr;
        } else {
            break
        }
    }

    return best
}

fn part_b(file_path: &str) -> i64 {
    let crab_pos = parse_input(file_path);
    let mut best: i64 = 99999999999999;
    let mut center: i32 = 0;
    let mut curr: i64;

    loop {
        curr = 0;
        for crab in &crab_pos {
            let dist = (crab - center).abs();
            curr += (dist * (dist + 1) / 2) as i64;
        }
        if curr < best {
            center += 1;
            best = curr;
        } else {
            break
        }
    }

    return best
}

fn parse_input(file_path: &str) -> Vec<i32> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(crabs) = line {
                for crab_pos in crabs.split(",") {
                    result.push(crab_pos.parse::<i32>().unwrap());
                }
            }
        }
    }

    return result;
}
