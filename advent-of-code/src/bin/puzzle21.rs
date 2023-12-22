use std::collections::HashSet;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle21/example.txt";
    let file_path = "data/puzzle21/input.txt";
    //let file_path = "data/puzzle21/easy.txt";

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

    explore_without_wrapping(s_loc, &grid, nrows as i32, ncols as i32, 6)
}

fn part_b(file_path: &str) -> i64 {
    let mut grid_str = String::new();
    let mut nrows: i32 = 0;
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
    let ncols: i32 = grid.len() as i32 / nrows;
    let mut n_steps = 26501365i32;
    let mut logical_ans: i64;
    loop {

        //let brute_ans = part_b_brute_force(s_loc, &grid, nrows, ncols, n_steps);
        let brute_ans: i64 = part_b_brute_force(s_loc, &grid, nrows, ncols, 11) as i64;
        println!("Steps ({n_steps}): Brute force answer {:?}", brute_ans);

        // Compute with logic!
        let full_on_cross = (n_steps + 1) / ncols;
        //let full_off_cross = full_on_cross * (full_on_cross - 1) / 2;
        let even_explored_cnt = explore_without_wrapping(s_loc, &grid, nrows, ncols, ncols + 1) as i64;
        let odd_explored_cnt = explore_without_wrapping(s_loc, &grid, nrows, ncols, ncols) as i64;
        let mut steps = (n_steps - ncols / 2) % ncols - 1;
        let mut partial_on_cross = (explore_without_wrapping([s_loc[0], 0], &grid, nrows, ncols, steps) +  
            explore_without_wrapping([s_loc[0], nrows - 1], &grid, nrows, ncols, steps) +
            explore_without_wrapping([0, s_loc[1]], &grid, nrows, ncols, steps) +
            explore_without_wrapping([ncols - 1, s_loc[1]], &grid, nrows, ncols, steps)) as i64;
        if (n_steps - (ncols - 1)) % ncols > ncols / 2 {
            // Two partial on-cross nodes on all 4 sides!
            steps = (n_steps - ncols / 2) % (ncols as i32) + ncols as i32 - 1;
            partial_on_cross += (explore_without_wrapping([s_loc[0], 0], &grid, nrows, ncols, steps) +
                explore_without_wrapping([s_loc[0], nrows as i32 - 1], &grid, nrows, ncols, steps) +
                explore_without_wrapping([0, s_loc[1]], &grid, nrows, ncols, steps) +
                explore_without_wrapping([ncols as i32 - 1, s_loc[1]], &grid, nrows, ncols, steps)) as i64;
        }
        let mut partial_off_cross = 0i64;
        let partial_size_counts = get_partial_nums(n_steps, ncols);
        for partial_size_count in partial_size_counts {
            let [mult, expl] = partial_size_count;
            partial_off_cross += (mult as i64) * (
                explore_without_wrapping([0, 0], &grid, nrows, ncols, expl) +
                explore_without_wrapping([nrows - 1, 0], &grid, nrows, ncols, expl) +
                explore_without_wrapping([0, ncols - 1], &grid, nrows, ncols, expl) +
                explore_without_wrapping([nrows - 1, ncols - 1], &grid, nrows, ncols, expl)
            ) as i64;
        }
        let full_even_nodes = 1 + max(0, ((full_on_cross - 1) as f32 / 2f32).floor() as i64) * 4 + even_off_cross_nodes(full_on_cross as i64) * 4;
        let full_odd_nodes = ((full_on_cross as f32) / 2f32).floor() as i64 * 4 + odd_off_cross_nodes(full_on_cross as i64) * 4;
        logical_ans = full_even_nodes * odd_explored_cnt +
            full_odd_nodes * even_explored_cnt +
            partial_on_cross + 
            partial_off_cross
        ;
        println!("Steps ({n_steps}): Logical approach: {logical_ans}");

        if logical_ans != brute_ans {
            println!("full_on_cross: {full_on_cross}");
            println!("partial_on_cross: {partial_on_cross}");
            println!("partial_off_cross: {partial_off_cross}");
            println!("Full even nodes {full_even_nodes} and off-cross {}", even_off_cross_nodes(full_on_cross as i64) * 4);
            println!("Full odd nodes {full_odd_nodes} and off-cross {}", odd_off_cross_nodes(full_on_cross as i64) * 4);
            break;
        }
        n_steps += 200;
    }
    logical_ans
}

fn even_off_cross_nodes(full_on_cross: i64) -> i64 {
    let mut factor: i64;
    if full_on_cross % 2 == 1 {
        factor = full_on_cross - 2
    } else {
        factor = full_on_cross - 3
    }
    if factor <= 0 {return 0}
    let mut ans = 0i64;
    while factor > 0 {
        ans += factor;
        factor -= 2;
    }
    ans
}

fn odd_off_cross_nodes(full_on_cross: i64) -> i64 {
    let mut factor: i64;
    if full_on_cross % 2 == 0 {
        factor = full_on_cross - 2
    } else {
        factor = full_on_cross - 3
    }
    if factor <= 0 {return 0}
    let mut ans = 0i64;
    while factor > 0 {
        ans += factor;
        factor -= 2;
    }
    ans
}

fn get_partial_nums(n_steps: i32, ncols: i32) -> Vec<[i32; 2]> {
    // Returns a vector of pairs of i32 elements.  Each pair represents the [# of partial nodes, how far to explore]
    let mut out: Vec<[i32; 2]> = Vec::new();
    let n_steps = n_steps - ncols - 1;  // Traverse to (0, 0) in adjacent garden
    
    let mut offset_idx = (n_steps - ncols) / (2 * ncols);
    loop {
        if n_steps < (ncols * (offset_idx + 1) + offset_idx - 1) {
            if n_steps - ncols * (offset_idx - 1) < (ncols - 1) * 2 {
                out.push([offset_idx, n_steps - ncols * (offset_idx - 1)]);
            }
        }
        if n_steps < offset_idx * ncols {break;}
        offset_idx += 1;
    }
    return out
}

fn part_b_brute_force(s_loc: [i32; 2], grid: &Vec<char>, nrows: i32, ncols: i32, n_steps: i32) -> i32 {
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
                let row_idx = (new_pos[0].rem_euclid(nrows)) as usize;
                let col_idx = (new_pos[1].rem_euclid(ncols)) as usize;
                if grid[row_idx * ncols as usize + col_idx] == '#' {
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

fn explore_without_wrapping(s_loc: [i32; 2], grid: &Vec<char>, nrows: i32, ncols: i32, n_steps: i32) -> i32 {
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
                if (new_pos[0] < 0) | (new_pos[0] >= nrows) | (new_pos[1] < 0) | (new_pos[1] >= ncols) {
                    continue  // Invalid location, skip!
                }
                if grid[(new_pos[0] * ncols + new_pos[1]) as usize] == '#' {
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
