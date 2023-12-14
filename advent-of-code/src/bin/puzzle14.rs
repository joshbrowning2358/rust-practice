use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle14/example.txt";
    let file_path = "data/puzzle14/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
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

    grid = north(grid, nrows, ncols);
    compute_load(grid, nrows, ncols)
}

fn compute_load(grid: Vec<char>, nrows: usize, ncols: usize) -> i32 {
    let mut ans: i32 = 0;
    for col in 0..ncols {
        for row in 0..nrows {
            let grid_val = grid[row * ncols + col];
            if grid_val == 'O' {
                //println!("Found 'O' at {row}x{col} with load {}", nrows - row);
                ans += (nrows - row) as i32;
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
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

    let mut loads: Vec<i32> = Vec::new();
    let mut proposed_offset: i32 = 100000;
    let mut match_streak = 0;
    
    loop {
        grid = north(grid, nrows, ncols);
        grid = west(grid, nrows, ncols);
        grid = south(grid, nrows, ncols);
        grid = east(grid, nrows, ncols);
        let load = compute_load(grid.clone(), nrows, ncols);

        if proposed_offset == 100000 {
            // Do we have a possible cycle?
            let candidate = match loads.iter().position(|&r| r == load) {
                Some(x) => {x as i32}
                None => {100000i32}
            };
            if candidate < 100000 {
                proposed_offset = loads.len() as i32 - candidate;
            }
        } else {
            if loads[loads.len() - proposed_offset as usize] == load {
                // Matched!  Keep going!
                match_streak += 1;
            } else {
                // Proposal doesn't work
                proposed_offset = 100000;
                match_streak = 0;
            }
        }

        if match_streak > (proposed_offset * 3) { // Just to make sure
            println!("Found cycle of length {proposed_offset}!");
            break
        }

        loads.push(load);
        println!("Computed loads of {:?}", loads);
    }

    // Found cycle len, now just need to get answer
    let target_remainder = 1000000000 % proposed_offset;
    let current_remainder = loads.len() as i32 % proposed_offset;
    let idx = loads.len() as i32 - 1 - proposed_offset - current_remainder + target_remainder;
    loads[idx as usize]
}

fn north(mut grid: Vec<char>, nrows: usize, ncols: usize) -> Vec<char> {
    for col in 0..ncols {
        let mut current_slot = 0;
        for row in 0..nrows {
            let grid_val = grid[row * ncols + col];
            match grid_val {
                'O' => {
                    //println!("Moving 'O' from {row}x{col} to {current_slot}x{col}");
                    grid[row * ncols + col] = '.';
                    grid[current_slot * ncols + col] = 'O';
                    current_slot += 1;
                },
                '#' => {
                    current_slot = row + 1;
                },
                '.' => {},
                _ => {panic!("Found invalid value!")}
            }
        }
    }
    grid
}

fn west(mut grid: Vec<char>, nrows: usize, ncols: usize) -> Vec<char> {
    for row in 0..nrows {
        let mut current_slot = 0;
        for col in 0..ncols {
            let grid_val = grid[row * ncols + col];
            match grid_val {
                'O' => {
                    grid[row * ncols + col] = '.';
                    grid[row * ncols + current_slot] = 'O';
                    current_slot += 1;
                },
                '#' => {
                    current_slot = col + 1;
                },
                '.' => {},
                _ => {panic!("Found invalid value!")}
            }
        }
    }
    grid
}

fn south(mut grid: Vec<char>, nrows: usize, ncols: usize) -> Vec<char> {
    for col in 0..ncols {
        let mut current_slot = nrows - 1;
        for row in (0..nrows).rev() {
            let grid_val = grid[row * ncols + col];
            match grid_val {
                'O' => {
                    grid[row * ncols + col] = '.';
                    grid[current_slot * ncols + col] = 'O';
                    if current_slot != 0 {
                        current_slot -= 1;
                    }
                },
                '#' => {
                    if row > 0 {current_slot = row - 1;}
                },
                '.' => {},
                _ => {panic!("Found invalid value!")}
            }
        }
    }
    grid
}

fn east(mut grid: Vec<char>, nrows: usize, ncols: usize) -> Vec<char> {
    for row in 0..nrows {
        let mut current_slot = ncols - 1;
        for col in (0..ncols).rev() {
            let grid_val = grid[row * ncols + col];
            match grid_val {
                'O' => {
                    grid[row * ncols + col] = '.';
                    grid[row * ncols + current_slot] = 'O';
                    if current_slot > 0 {current_slot -= 1;}
                },
                '#' => {
                    if col > 0 {current_slot = col - 1;}
                },
                '.' => {},
                _ => {panic!("Found invalid value!")}
            }
        }
    }
    grid
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
