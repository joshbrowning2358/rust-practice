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

fn part_a(file_path: &str) -> i32 {
    parse_input(file_path, false);
    return 0
}

fn part_b(file_path: &str) -> i32 {
    return 0
}

fn parse_input(file_path: &str, include_diag: bool) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(pipes) = line {
                let (left_tup, right_tup) = pipes.split_once(" -> ").unwrap();
                let (left_x, left_y) = left_tup.split_once(",").unwrap();
                let (right_x, right_y) = right_tup.split_once(",").unwrap();

                result.push(vec![int(left_x), int(left_y), int(right_x), int(right_y)]);
            }
        }
    }

    return result;
}
