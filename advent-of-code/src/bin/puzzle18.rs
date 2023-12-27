use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::i32;
use std::iter::zip;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle18/example.txt"; let interior = [1, 1];
    let file_path = "data/puzzle18/input.txt"; let interior = [1, 1];

    let ans = part_a(file_path, interior);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str, _interior: [i32; 2]) -> f64 {
    let mut directions: VecDeque<[i32; 2]> = VecDeque::new();
    let mut distances: VecDeque<i32> = VecDeque::new();
    let mut last_direction: String = String::new();
    let mut first_direction: String = String::new();
    let mut trench_area: f64 = 0.;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let first_paren = ip.find('(').unwrap();
                let (dir, dist) = ip[0..(first_paren - 1)].split_once(' ').unwrap();
                directions.push_back(match dir {
                    "R" => [0, 1],
                    "L" => [0, -1],
                    "U" => [-1, 0],
                    "D" => [1, 0],
                    &_ => panic!("Weird direction!"),
                });
                let dist = dist.parse::<i32>().unwrap();
                distances.push_back(dist);

                if trench_area == 0. {first_direction = dir.to_string();}

                // Update trench area
                trench_area += (dist as f64 + 1.) / 2.;
                // Correct for corners
                trench_area += get_adjustment(dir, &last_direction);
                last_direction = dir.to_string();
            }
        }
    }

    trench_area += get_adjustment(&first_direction, &last_direction);
    //floodfill(&visited, interior) + (visited.len() as i32)
    let col_locs = get_column_locations(&directions, &distances);
    compute_area(col_locs) + trench_area
}

fn part_b(file_path: &str) -> f64 {
    let mut directions: VecDeque<[i32; 2]> = VecDeque::new();
    let mut distances: VecDeque<i32> = VecDeque::new();
    let mut last_direction: String = String::new();
    let mut first_direction: String = String::new();
    let mut trench_area: f64 = 0.;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let first_paren = ip.find('(').unwrap();
                let hex = &ip[(first_paren + 2)..(first_paren + 7)];
                directions.push_back(match ip[(first_paren + 7)..(first_paren + 8)].to_string().as_str() {
                    "0" => [0, 1],
                    "1" => [1, 0],
                    "2" => [0, -1],
                    "3" => [-1, 0],
                    &_ => panic!("Invalid direction!"),
                });
                let dist = i32::from_str_radix(hex, 16).unwrap();
                distances.push_back(dist);

                if trench_area == 0. {first_direction = ip[(first_paren + 7)..(first_paren + 8)].to_string();}

                // Update trench area
                trench_area += (dist as f64 + 1.) / 2.;
                // Correct for corners
                trench_area += get_adjustment(&ip[(first_paren + 7)..(first_paren + 8)], &last_direction);
                last_direction = ip[(first_paren + 7)..(first_paren + 8)].to_string();
            }
        }
    }
    
    trench_area += get_adjustment(&first_direction, &last_direction);
    let col_locs = get_column_locations(&directions, &distances);
    let ans = compute_area(col_locs);
    println!("Got area of {}", ans);
    ans + trench_area
}

fn get_adjustment(dir: &str, last_dir: &str) -> f64 {
    if dir == "R" || dir == "0" {
        if last_dir == "U" || last_dir == "3" {return -0.25;}
        if last_dir == "D" || last_dir == "1" {return -0.75;}
    } else if dir == "L" || dir == "2" {
        if last_dir == "U" || last_dir == "3" {return -0.75;}
        if last_dir == "D" || last_dir == "1" {return -0.25;}
    } else if dir == "U" || dir == "3" {
        if last_dir == "R" || last_dir == "0" {return -0.75;}
        if last_dir == "L" || last_dir == "2" {return -0.25;}
    } else if dir == "D" || dir == "1" {
        if last_dir == "R" || last_dir == "0" {return -0.25;}
        if last_dir == "L" || last_dir == "2" {return -0.75;}
    }
    println!("Failed to return with inputs {} and {}", dir, last_dir);
    0.
}

fn get_column_locations(directions: &VecDeque<[i32; 2]>, distances: &VecDeque<i32>) -> HashMap<[i32; 2], Vec<i32>> {
    let (min_row, _, max_row, _) = find_bounds(&directions, &distances);

    let mut results: HashMap<[i32; 2], Vec<i32>> = HashMap::new();
    results.insert([min_row, max_row], Vec::new());

    let mut row: i32 = 0;
    let mut col: i32 = 0;
    // Navigate the grid and update sparse column results
    for (dir, dist) in directions.iter().zip(distances.iter()) {
        //println!("Checking {:?} and dist {dist}", dir);
        if dir[0] == 0 {
            col += dist * dir[1]; // Not much to do with horizontal moves
        } else {
            let start_row = min(row, row + dir[0] * dist);
            let end_row = max(row, row + dir[0] * dist);
            
            let results_clone = results.clone();
            for key_range in results_clone.keys() {
                let [key_start, key_end] = key_range;
                if (start_row <= *key_start) & (key_end <= &end_row) { // Key is fully contained in move
                    results.get_mut(key_range).unwrap().push(col);
                } else if (key_start <= &start_row) & (end_row <= *key_end) { // Move is fully contained in key
                    let mut curr_cols = results.remove(key_range).unwrap();
                    if start_row != *key_start {results.insert([*key_start, start_row], curr_cols.clone());}
                    if end_row != *key_end {results.insert([end_row, *key_end], curr_cols.clone());}
                    curr_cols.push(col);
                    results.insert([start_row, end_row], curr_cols);
                } else if (key_start <= &start_row) & (start_row < *key_end) {
                    let mut curr_cols = results.remove(key_range).unwrap();
                    results.insert([*key_start, start_row], curr_cols.clone());
                    curr_cols.push(col);
                    results.insert([start_row, *key_end], curr_cols);
                } else if (start_row <= *key_start) & (key_start < &end_row) {
                    let mut curr_cols = results.remove(key_range).unwrap();
                    results.insert([end_row, *key_end], curr_cols.clone());
                    curr_cols.push(col);
                    results.insert([*key_start, end_row], curr_cols);
                }
            }

            row += dist * dir[0];
        }
    }

    results
}

fn compute_area(cols: HashMap<[i32; 2], Vec<i32>>) -> f64 {
    let mut ans: i64 = 0;
    for (range, col_list) in cols.iter() {
        let mut col_list = col_list.clone();
        col_list.sort();
        let height: i64 = (range[1] - range[0]) as i64;
        if height > 0 {
            for pair_idx in 0..(col_list.len() / 2) {
                let rect_area = height * ((col_list[pair_idx * 2 + 1] - col_list[pair_idx * 2]) as i64); 
                ans += rect_area;
            }
        }
    }

    ans as f64
}

fn find_bounds(directions: &VecDeque<[i32; 2]>, distances: &VecDeque<i32>) -> (i32, i32, i32, i32) {
    let mut min_row: i32 = 0;
    let mut max_row: i32 = 0;
    let mut min_col: i32 = 0;
    let mut max_col: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    for (dir, dist) in directions.iter().zip(distances.iter()) {
        match dir {
            [0, 1] => {col += dist;}
            [0, -1] => {col -= dist;}
            [1, 0] => {row += dist;}
            [-1, 0] => {row -= dist;}
            _ => {panic!("Unexpected direction!");}
        }
        if row > max_row {max_row = row;}
        if col > max_col {max_col = col;}
        if row < min_row {min_row = row;}
        if col < min_col {min_col = col;}
    }
    (min_row, min_col, max_row, max_col)
}

fn get_path(directions: &VecDeque<[i32; 2]>, distances: &VecDeque<i32>) -> HashSet<[i32; 2]> {
    let mut pos = [0, 0];
    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    for (dir, dist) in directions.iter().zip(distances.iter()) {
        for _i in 0..(*dist) { 
            if dir[0] != 0 {
                pos[0] += dir[0];
            } else {
                pos[1] += dir[1];
            }
            visited.insert(pos);
        }
    }
    visited
}

fn floodfill(path: &HashSet<[i32; 2]>, interior: [i32; 2]) -> i32 {
    let mut filled: HashSet<[i32; 2]> = HashSet::new();
    let mut to_explore: VecDeque<[i32; 2]> = VecDeque::new();

    let mut ans: i32 = 0;
    to_explore.push_back(interior);
    filled.insert(interior);
    while to_explore.len() > 0 {
        let current_node = to_explore.pop_front().unwrap();
        ans += 1;
        for (r, c) in zip([-1, 1, 0, 0], [0, 0, -1, 1]) {
            let candidate = [current_node[0] + r, current_node[1] + c];
            if (!filled.contains(&candidate)) & (!path.contains(&candidate)) {
                to_explore.push_back(candidate);
                filled.insert(candidate);
            }
        }
    }
    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
