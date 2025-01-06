use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle23/example.txt";
    let file_path = "data/puzzle23/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: i32 = 0;
    let mut s_loc: [i32; 2] = [0, 0];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                if nrows == 0 {
                    s_loc = [nrows as i32, ip.find('.').unwrap() as i32];
                }
                nrows += 1;
            }
        }
    }
    let grid: Vec<char> = grid_str.chars().collect();
    let ncols: i32 = grid.len() as i32 / nrows;
    println!("Found s as {:?} in grid of {nrows}x{ncols}", s_loc);
    let visited: HashSet<[i32; 2]> = HashSet::new();
    longest_path_from_loc(s_loc, &grid, nrows, ncols, 0, visited, true)
}

fn part_b(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: i32 = 0;
    let mut s_loc: [i32; 2] = [0, 0];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                if nrows == 0 {
                    s_loc = [nrows as i32, ip.find('.').unwrap() as i32];
                }
                nrows += 1;
            }
        }
    }
    let grid: Vec<char> = grid_str.chars().collect();
    let ncols: i32 = grid.len() as i32 / nrows;
    println!("Found s as {:?} in grid of {nrows}x{ncols}", s_loc);
    let visited: HashSet<[i32; 2]> = HashSet::new();
    longest_path_from_loc(s_loc, &grid, nrows, ncols, 0, visited, false)
}

fn longest_path_from_loc(loc: [i32; 2], grid: &Vec<char>, nrows: i32, ncols: i32, mut curr_dist: i32, mut visited: HashSet<[i32; 2]>, part_a: bool) -> i32 {
    //println!("Starting new branch at {:?}", loc);
    let mut to_explore: Vec<[i32; 2]> = Vec::new();
    let mut curr_loc = loc;
    visited.insert(curr_loc);
    loop {
        for (r, c) in zip([0, 0, -1, 1], [-1, 1, 0, 0]) {
            let next_loc = [curr_loc[0] + r, curr_loc[1] + c];
            if (next_loc[0] < 0) | (next_loc[0] >= nrows) | (next_loc[1] < 0) | (next_loc[1] >= ncols) {
                continue
            }
            let grid_val = grid[(next_loc[0] * ncols + next_loc[1]) as usize];
            if grid_val == '#' {
                continue
            }
            if part_a {
                if ((grid_val == '>') & (c != 1)) | ((grid_val == 'v') & (r != 1)) {
                    continue
                }
            }
            if visited.contains(&next_loc) {
                continue
            }
            to_explore.push(next_loc);
        }

        if to_explore.len() == 1 {
            curr_loc = to_explore.pop().unwrap();
            //println!("  Must move to {:?}", curr_loc);
            visited.insert(curr_loc);
            curr_dist += 1;
        } else if to_explore.len() > 1 {
            let mut longest = 0;
            //println!("Branching!  Exploring {:?}", to_explore);
            while to_explore.len() > 0 {
                let new_node = to_explore.pop().unwrap();
                //visited.insert(new_node);
                longest = max(longest, longest_path_from_loc(new_node, &grid, nrows, ncols, curr_dist + 1, visited.clone(), part_a));
            }
            return longest
        } else {
            return 0 // Nowhere to go, invalid path!
        }

        if curr_loc[0] == nrows - 1 {
            break;
        }
    }
    println!("Path ended, found dist of {curr_dist}!");
    curr_dist
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
