use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle21/example.txt";
    //let file_path = "data/puzzle21/input.txt";
    let file_path = "data/puzzle21/easy.txt";

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

    explore_without_wrapping(s_loc, &grid, nrows, ncols, 6)
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
    let n_steps = 15;
    println!("Found s as {:?} in grid of {nrows}x{ncols}", s_loc);

    let brute_ans = part_b_brute_force(s_loc, &grid, nrows, ncols, n_steps);
    println!("Brute force answer {:?}", brute_ans);

    // Compute with logic!
    let full_on_cross = n_steps.div_euclid(s_loc[0] + s_loc[1]);
    //let full_off_cross = full_on_cross * (full_on_cross - 1) / 2;
    let even_explored_cnt = explore_without_wrapping(s_loc, &grid, nrows, ncols, ncols as i32 + 1);
    let odd_explored_cnt = explore_without_wrapping(s_loc, &grid, nrows, ncols, ncols as i32);
    let mut steps = (n_steps - ncols as i32 / 2) % (ncols as i32) - 1;
    let mut partial_on_cross = explore_without_wrapping([s_loc[0], 0], &grid, nrows, ncols, steps) +  
        explore_without_wrapping([s_loc[0], nrows as i32 - 1], &grid, nrows, ncols, steps) +
        explore_without_wrapping([0, s_loc[1]], &grid, nrows, ncols, steps) +
        explore_without_wrapping([ncols as i32 - 1, s_loc[1]], &grid, nrows, ncols, steps);
    if (n_steps - (ncols as i32 - 1)) % ncols as i32 > ncols as i32 / 2 {
        // Two partial on-cross nodes on all 4 sides!
        steps = (n_steps - ncols as i32 / 2) % (ncols as i32) + ncols as i32 - 1;
        partial_on_cross += explore_without_wrapping([s_loc[0], 0], &grid, nrows, ncols, steps) +
            explore_without_wrapping([s_loc[0], nrows as i32 - 1], &grid, nrows, ncols, steps) +
            explore_without_wrapping([0, s_loc[1]], &grid, nrows, ncols, steps) +
            explore_without_wrapping([ncols as i32 - 1, s_loc[1]], &grid, nrows, ncols, steps);
    }
    steps = (n_steps - 1) % ncols as i32;
    let mut partial_off_cross_big = explore_without_wrapping([0, 0], &grid, nrows, ncols, steps) +
        explore_without_wrapping([nrows as i32 - 1, 0], &grid, nrows, ncols, steps) +
        explore_without_wrapping([0, ncols as i32 - 1], &grid, nrows, ncols, steps) +
        explore_without_wrapping([nrows as i32 - 1, ncols as i32 - 1], &grid, nrows, ncols, steps);
    partial_off_cross_big *= (n_steps - 1).div_euclid(ncols as i32); 
    let mut partial_off_cross_small = explore_without_wrapping([0, 0], &grid, nrows, ncols, n_steps - (full_on_cross + 1) * ncols as i32) +
        explore_without_wrapping([nrows as i32, 0], &grid, nrows, ncols, n_steps - (full_on_cross + 1) * ncols as i32) +
        explore_without_wrapping([0, ncols as i32], &grid, nrows, ncols, n_steps - (full_on_cross + 1) * ncols as i32) +
        explore_without_wrapping([nrows as i32, ncols as i32], &grid, nrows, ncols, n_steps - (full_on_cross + 1) * ncols as i32);
    partial_off_cross_small *= full_on_cross;
    println!("full_on_cross: {full_on_cross}");
    //println!("full_off_cross: {full_off_cross}");
    println!("partial_on_cross: {partial_on_cross}");
    println!("partial_off_cross_big: {partial_off_cross_big}");
    println!("partial_off_cross_small: {partial_off_cross_small}");
    let full_even_nodes = 1 + (full_on_cross - 1).div_euclid(2) * 4 + even_off_cross_nodes(full_on_cross);
    let full_odd_nodes = full_on_cross.div_euclid(2) * 4 + odd_off_cross_nodes(full_on_cross);
    println!("Full even nodes {full_even_nodes} and off-cross {}", even_off_cross_nodes(full_on_cross));
    println!("Full odd nodes {full_odd_nodes} and off-cross {}", odd_off_cross_nodes(full_on_cross));
    //println!("Logical approach: {}", ((full_on_cross - 1) * 4 + 1 + full_off_cross) * explored_cnt + partial_on_cross + partial_off_cross_big + partial_off_cross_small);
    println!("Logical approach: {}", (
        full_even_nodes * odd_explored_cnt +
        full_odd_nodes * even_explored_cnt +
        partial_on_cross + 
        partial_off_cross_big + 
        partial_off_cross_small
    ));
    0
}

fn even_off_cross_nodes(full_on_cross: i32) -> i32 {
    let mut factor;
    if full_on_cross % 2 == 1 {
        factor = full_on_cross - 2
    } else {
        factor = full_on_cross - 3
    }
    if factor <= 0 {return 0}
    let mut ans = 1;
    while factor > 0 {
        ans *= factor;
        factor -= 2;
    }
    ans
}

fn odd_off_cross_nodes(full_on_cross: i32) -> i32 {
    let base = (full_on_cross - 2).div_euclid(2);
    if base <= 0 {return 0}
    2 * base * (base + 1) / 2 * 4
}

fn part_b_brute_force(s_loc: [i32; 2], grid: &Vec<char>, nrows: usize, ncols: usize, n_steps: i32) -> i32 {
    let mut odd_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut even_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut to_explore: Vec<[i32; 2]> = Vec::new();

    even_nodes.insert(s_loc);
    to_explore.push(s_loc);
    for i in 1..(n_steps + 1) {
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
    if n_steps % 2 == 0 {
        return even_nodes.len() as i32
    } else {
        return odd_nodes.len() as i32
    }
}

fn explore_without_wrapping(s_loc: [i32; 2], grid: &Vec<char>, nrows: usize, ncols: usize, n_steps: i32) -> i32 {
    if n_steps < 0 {return 0}
    let mut odd_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut even_nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut to_explore: Vec<[i32; 2]> = Vec::new();

    even_nodes.insert(s_loc);
    to_explore.push(s_loc);
    for i in 1..(n_steps + 1) {
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
    if n_steps % 2 == 1 {
        odd_nodes.len() as i32
    } else {
        even_nodes.len() as i32
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
