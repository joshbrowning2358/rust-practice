mod file_reader;

fn main() {
    let file_name = file!();
    let file_path = "data/{file_name}/input.txt";

    let mut ans = part_a(file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    return 0
}

fn part_b(file_path: &str) -> i32 {
    return 0
}
