use std::cmp;

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

fn part_a(file_path: &str) -> u32 {
    let dims = parse_input(file_path);
    let mut total_wrapping_paper: u32 = 0;
    for (x, y, z) in dims {
        total_wrapping_paper += 2 * x * y + 2 * x * z + 2 * y * z;
        total_wrapping_paper += cmp::min(cmp::min(x * y, y * z), x * z);
    }
    return total_wrapping_paper
}

fn part_b(file_path: &str) -> u32 {
    let dims = parse_input(file_path);
    let mut total_ribbon: u32 = 0;
    for (x, y, z) in dims {
        total_ribbon += x * y * z;
        total_ribbon += cmp::min(cmp::min(x + y, y + z), x + z) * 2;
    }
    return total_ribbon 
}

fn parse_input(file_path: &str) -> Vec<(u32, u32, u32)> {
    let mut result: Vec<(u32, u32, u32)> = Vec::new();
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let dims: Vec<&str> = row.split('x').collect();
                result.push(
                    (
                        dims[0].parse::<u32>().unwrap(),
                        dims[1].parse::<u32>().unwrap(),
                        dims[2].parse::<u32>().unwrap()
                    )
                );
            }
        }
    }
    return result
}
