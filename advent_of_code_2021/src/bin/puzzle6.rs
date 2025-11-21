mod file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let mut ans = part_a(&file_path, 80);
    println!("Answer to {file_name} a is {ans};");

    ans = part_a(&file_path, 256);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str, n_days: usize) -> i64 {
    let fish_vec = parse_input(file_path);
    let mut curr_ages: Vec<i64> = vec![0; 9];
    // let n_days = 80;
    
    // Populate curr_ages with the initial fish
    for fish in fish_vec {
        curr_ages[fish as usize] += 1;
    }

    // Iterate over time
    for _day in 0..n_days {
        let mut new_ages: Vec<i64> = vec![0; 9];
        for (i, cnt) in curr_ages.iter().enumerate() {
            if i == 0 {
                new_ages[6] += *cnt;
                new_ages[8] += *cnt;
            } else {
                new_ages[i - 1] += *cnt;
            }
        }
        curr_ages = new_ages;
    }

    return curr_ages.iter().sum()
}

fn parse_input(file_path: &str) -> Vec<u8> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(fish) = line {
                for fish_age in fish.split(",") {
                    result.push(fish_age.parse::<u8>().unwrap());
                }
            }
        }
    }

    return result;
}
