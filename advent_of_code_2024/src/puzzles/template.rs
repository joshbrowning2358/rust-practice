use advent_of_code_2024::file_reader;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    let ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    return 0
}

fn part_b(file_path: &str) -> i32 {
    return 0
}

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
            }
        }
    }

    return result
}
