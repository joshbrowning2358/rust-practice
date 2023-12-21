use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn main() {
    let file_path = "data/puzzle21/example.txt";
    //let file_path = "data/puzzle21/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: usize = 0;
    let mut s_loc: [i32; 2] = [0, 0];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                if ip.contains('S') {
                    s_loc = [nrows as i32, ip.find('S').unwrap() as i32];
                }
                nrows += 1;
            }
        }
    }
    let grid: Vec<char> = grid_str.chars().collect();
    let ncols: usize = grid.len() / nrows;
    println!("Found s as {:?} in grid of {nrows}x{ncols}", s_loc);

    let mut odd_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut even_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut to_explore: Vec<[i32; 2]> = Vec::new();

    even_nodes.insert(s_loc);
    to_explore.push(s_loc);
    for i in 1..65 {
        let mut next_to_explore: Vec<[i32; 2]> = Vec::new();
        while to_explore.len() > 0 {
            let node = to_explore.pop().unwrap();
            for (r, c) in zip([-1, 1, 0, 0], [0, 0, -1, 1]) {
                let new_pos = [node[0] + r, node[1] + c];
                if (new_pos[0] < 0) | (new_pos[0] >= nrows as i32) | (new_pos[1] < 0) | (new_pos[1] >= ncols as i32) {
                    continue  // Invalid location, skip!
                }
                if grid[(new_pos[0] * (ncols as i32) + new_pos[1]) as usize] == '#' {
                    continue  // At rock
                }
                if odd_nodes.contains(&new_pos) | even_nodes.contains(&new_pos) {
                    continue  // Already visited
                }
                if i % 2 == 1 {
                    odd_nodes.insert(new_pos);
                } else {
                    even_nodes.insert(new_pos);
                }
                next_to_explore.push(new_pos);
            }
        }
        to_explore = next_to_explore;
    }
    even_nodes.len() as i32
}

fn part_b(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: usize = 0;
    let mut s_loc: [i32; 2] = [0, 0];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                if ip.contains('S') {
                    s_loc = [nrows as i32, ip.find('S').unwrap() as i32];
                }
                nrows += 1;
            }
        }
    }
    let grid: Vec<char> = grid_str.chars().collect();
    let ncols: usize = grid.len() / nrows;
    println!("Found s as {:?} in grid of {nrows}x{ncols}", s_loc);

    let mut odd_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut even_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut to_explore: Vec<[i32; 2]> = Vec::new();

    even_nodes.insert(s_loc);
    to_explore.push(s_loc);
    for i in 1..5001 {
        let mut next_to_explore: Vec<[i32; 2]> = Vec::new();
        while to_explore.len() > 0 {
            let node = to_explore.pop().unwrap();
            for (r, c) in zip([-1, 1, 0, 0], [0, 0, -1, 1]) {
                let new_pos = [node[0] + r, node[1] + c];
                let row_idx = (new_pos[0].rem_euclid(nrows as i32)) as usize;
                let col_idx = (new_pos[1].rem_euclid(ncols as i32)) as usize;
                if grid[row_idx * ncols + col_idx] == '#' {
                    continue  // At rock
                }
                if odd_nodes.contains(&new_pos) | even_nodes.contains(&new_pos) {
                    continue  // Already visited
                }
                if i % 2 == 1 {
                    odd_nodes.insert(new_pos);
                } else {
                    even_nodes.insert(new_pos);
                }
                next_to_explore.push(new_pos);
            }
        }
        to_explore = next_to_explore;
    }
    even_nodes.len() as i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
