use std::collections::HashSet;
use std::fs;

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
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    houses.insert((x, y));
    for dir in contents.chars() {
        x += match dir {
            '>' => -1,
            '<' => 1,
            _ => 0
        };
        y += match dir {
            '^' => 1,
            'v' => -1,
            _ => 0
        };
        houses.insert((x, y));
    }
    return houses.len() as i32
}

fn part_b(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut rx: i32 = 0;
    let mut ry: i32 = 0;
    houses.insert((sx, sy));
    for (i, dir) in contents.chars().enumerate() {
        if i % 2 == 0 {
            sx += match dir {
                '>' => -1,
                '<' => 1,
                _ => 0
            };
            sy += match dir {
                 '^' => 1,
                 'v' => -1,
                 _ => 0
            };
            houses.insert((sx, sy));
        } else {
            rx += match dir {
                '>' => -1,
                '<' => 1,
                _ => 0
            };
            ry += match dir {
                 '^' => 1,
                 'v' => -1,
                 _ => 0
            };
            houses.insert((rx, ry));
        }
    }
    return houses.len() as i32
}

