use std::{thread, time};
use std::collections::HashSet;

use advent_of_code_2024::file_reader;

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
    let (robots, width, height) = parse_input(file_path);

    let mut quadrants = vec![0; 4];
    for ((pos_x, pos_y), (vel_x, vel_y)) in robots {
        let end_pos_x = (pos_x + vel_x * 100).rem_euclid(width);
        let end_pos_y = (pos_y + vel_y * 100).rem_euclid(height);
        if end_pos_x < width / 2 {
            if end_pos_y < height / 2 {
                quadrants[0] += 1;
            } else if end_pos_y > height / 2 {
                quadrants[1] += 1;
            }
        } else if end_pos_x > width / 2 {
            if end_pos_y < height / 2 {
                quadrants[2] += 1;
            } else if end_pos_y > height / 2 {
                quadrants[3] += 1;
            }
        }
    }
    return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn part_b(file_path: &str) -> i32 {
    // 1000 is too low
    // 100000 is too high
    let (robots, width, height) = parse_input(file_path);

    for i in 1000..100000 {
        let mut grid = vec![vec!['.'; width as usize]; height as usize];
        let mut positions = HashSet::new();
        let mut cond = false;
        for ((pos_x, pos_y), (vel_x, vel_y)) in &robots {
            let end_pos_x = (pos_x + vel_x * i).rem_euclid(width);
            let end_pos_y = (pos_y + vel_y * i).rem_euclid(height);
            grid[end_pos_y as usize][end_pos_x as usize] = '*';

            positions.insert((end_pos_x, end_pos_y));
            if (end_pos_x > 0) & (end_pos_y > 0) & (end_pos_x < width - 1) & (end_pos_y < height - 1) {
                if positions.contains(&(end_pos_x - 1, end_pos_y)) & positions.contains(&(end_pos_x + 1, end_pos_y)) & positions.contains(&(end_pos_x, end_pos_y - 1)) & positions.contains(&(end_pos_x, end_pos_y + 1)) {
                    cond = true;
                }
            }
        }
        // let cond = grid[50][49..52].iter().all(|x| *x == '*') & grid[51][49..52].iter().all(|x| *x == '*') & grid[52][49..52].iter().all(|x| *x == '*');
        // let cond = grid[51][50] == '*';
        if cond {
            println!("Iteration {i}");
            for grid_line in grid {
                let printable: String = grid_line.iter().collect();
                println!("{printable:?}");
            }
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
    return 0
}

fn parse_input(file_path: &str) -> (Vec<((i32, i32), (i32, i32))>, i32, i32) {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(robot) = line {
                let (pos, vel) = robot[2..].split_once(" v=").unwrap();
                let (pos_x, pos_y) = pos.split_once(',').unwrap();
                let (vel_x, vel_y) = vel.split_once(',').unwrap();
                result.push(
                    (
                        (pos_x.parse::<i32>().unwrap(),
                         pos_y.parse::<i32>().unwrap()),
                        (vel_x.parse::<i32>().unwrap(),
                         vel_y.parse::<i32>().unwrap())
                    )
                );
            }
        }
    }

    if file_path.contains("easy") {
        return (result, 11, 7)
    } else {
        return (result, 101, 103)
    }
}
