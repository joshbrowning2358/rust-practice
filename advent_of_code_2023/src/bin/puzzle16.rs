use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle16/example.txt";
    let file_path = "data/puzzle16/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                nrows += 1;
            }
        }
    }
    let mut grid: Vec<char> = grid_str.chars().collect();
    let ncols: usize = grid.len() / nrows;

    //println!("Grid is {:?}\nVisited is {:?}\nShape is {nrows}x{ncols}", grid, visited_cnt);
    compute_illumination(&grid, nrows, ncols, [0, -1, 0, 1])
}


fn compute_illumination(grid: &Vec<char>, nrows: usize, ncols: usize, start_state: [i32; 4]) -> i32 {
    let mut visited_cnt: Vec<i32> = vec![0; grid.len()];

    // Send in the light!
    let mut queue_to_process: VecDeque<[i32; 4]> = VecDeque::new();
    let mut obs_states: Vec<[i32; 4]> = Vec::new(); // Only visit a state if it's new
    queue_to_process.push_back(start_state);
    while queue_to_process.len() > 0 {
        let current_state = queue_to_process.pop_back().unwrap();

        // Skip current_state if it's been visited
        let mut already_visited: bool = false;
        for obs_state in obs_states.iter() {
            if array_eq(obs_state, current_state) {
                //println!("{:?} matches {:?}", obs_state, current_state);
                already_visited = true;
                break;
            }
        }

        if already_visited {
            continue
        } else {
            obs_states.push(current_state);
        }

        let current_pos = &current_state[..2];
        let current_move = &current_state[2..];
        //println!("At {:?} and moving {:?} with obs_states {:?} and queue {:?}", current_pos, current_move, obs_states, queue_to_process);
        //println!("At {:?} and moving {:?}", current_pos, current_move);
        let next_pos = [current_pos[0] + current_move[0], current_pos[1] + current_move[1]];
        if (next_pos[0] < 0) | (next_pos[0] >= nrows as i32) | (next_pos[1] < 0) | (next_pos[1] >= ncols as i32) {
            continue // Exit the grid, but keep looping in case of branching
        }
        let grid_val = grid[(next_pos[0] as usize) * ncols + (next_pos[1] as usize)];
        visited_cnt[(next_pos[0] as usize) * ncols + (next_pos[1] as usize)] += 1;
        match grid_val {
            '.' => {
                queue_to_process.push_front([next_pos[0], next_pos[1], current_move[0], current_move[1]]);
            },
            '-' => {
                if current_move[1] != 0 {
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, current_move[1]]);
                } else {
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, 1]);
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, -1]);
                }
            },
            '|' => {
                if current_move[0] != 0 {
                    queue_to_process.push_front([next_pos[0], next_pos[1], current_move[0], 0]);
                } else {
                    queue_to_process.push_front([next_pos[0], next_pos[1], 1, 0]);
                    queue_to_process.push_front([next_pos[0], next_pos[1], -1, 0]);
                }
            },
            '/' => {
                if current_move[0] == 1 { // Moved down a row
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, -1]);
                } else if current_move[0] == -1 { // Moved up a row
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, 1]);
                } else if current_move[1] == 1 { // Moved right
                    queue_to_process.push_front([next_pos[0], next_pos[1], -1, 0]);
                } else { // Moved left
                    queue_to_process.push_front([next_pos[0], next_pos[1], 1, 0]);
                }
            },
            _ => { // Handle \ that can't be processed
                if current_move[0] == 1 { // Moved down a row
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, 1]);
                } else if current_move[0] == -1 { // Moved up a row
                    queue_to_process.push_front([next_pos[0], next_pos[1], 0, -1]);
                } else if current_move[1] == 1 { // Moved right
                    queue_to_process.push_front([next_pos[0], next_pos[1], 1, 0]);
                } else { // Moved left
                    queue_to_process.push_front([next_pos[0], next_pos[1], -1, 0]);
                }
            }
        }
    }

    //println!("\n\nVisited:");
    //for i in 0..nrows {
    //    println!("{:?}", &visited_cnt[(nrows * i)..(nrows * (i + 1))]);
    //}

    visited_cnt.iter().filter(|&x| x > &0).count() as i32
}

fn array_eq(x: &[i32; 4], y: [i32; 4]) -> bool {
    for i in 0..4 {
        if x[i] != y[i] {return false}
    }
    true
}

fn part_b(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                nrows += 1;
            }
        }
    }
    let mut grid: Vec<char> = grid_str.chars().collect();
    let ncols: usize = grid.len() / nrows;

    //println!("Grid is {:?}\nVisited is {:?}\nShape is {nrows}x{ncols}", grid, visited_cnt);
    let mut ans: i32 = 0;
    for row in 0..nrows {
        let curr_ans = compute_illumination(&grid, nrows, ncols, [row as i32, -1, 0, 1]);
        if curr_ans > ans {ans = curr_ans;}
        let curr_ans = compute_illumination(&grid, nrows, ncols, [row as i32, ncols as i32, 0, -1]);
        if curr_ans > ans {ans = curr_ans;}
        println!("Finished row {row}");
    }

    for col in 0..ncols {
        let curr_ans = compute_illumination(&grid, nrows, ncols, [-1, col as i32, 1, 0]);
        if curr_ans > ans {ans = curr_ans;}
        let curr_ans = compute_illumination(&grid, nrows, ncols, [nrows as i32, col as i32, -1, 0]);
        if curr_ans > ans {ans = curr_ans;}
    }

    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
