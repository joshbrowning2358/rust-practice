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

use num::abs;

fn main() {
    let file_path = "data/puzzle18/example.txt"; let interior = [1, 1];
    //let file_path = "data/puzzle18/input.txt"; let interior = [1, 1];

    let ans = part_a(file_path, interior);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");

    let mut directions = VecDeque::new();
    let mut distances = VecDeque::new();
    directions.push_back([0, 1]);
    directions.push_back([1, 0]);
    directions.push_back([0, 1]);
    directions.push_back([1, 0]);
    directions.push_back([0, -1]);
    directions.push_back([-1, 0]);
    distances.push_back(3);
    distances.push_back(3);
    distances.push_back(2);
    distances.push_back(7);
    distances.push_back(5);
    distances.push_back(10);
    let ans = get_column_locations(&directions, &distances);
    println!("Answer is {:?}", ans);
    println!("Area is {}", compute_area(ans));
}

fn part_a(file_path: &str, _interior: [i32; 2]) -> i64 {
    let mut directions: VecDeque<[i32; 2]> = VecDeque::new();
    let mut distances: VecDeque<i32> = VecDeque::new();

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
                distances.push_back(dist.parse::<i32>().unwrap())
            }
        }
    }
    //println!("Found directions {:?} and distances {:?}", directions, distances);

    //let visited = get_path(&directions, &distances);
    //println!("Visited nodes {} are {:?}", visited.len(), visited);

    //floodfill(&visited, interior) + (visited.len() as i32)
    let col_locs = get_column_locations(&directions, &distances);
    println!("Col_locs are {:?}", col_locs);
    compute_area(col_locs)
}

fn part_b(file_path: &str) -> i64 {
    let mut directions: VecDeque<[i32; 2]> = VecDeque::new();
    let mut distances: VecDeque<i32> = VecDeque::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let first_paren = ip.find('(').unwrap();
                let hex = &ip[(first_paren + 2)..(first_paren + 7)];
                distances.push_back(i32::from_str_radix(hex, 16).unwrap());
                directions.push_back(match ip[(first_paren + 7)..(first_paren + 8)].to_string().as_str() {
                    "0" => [0, 1],
                    "1" => [1, 0],
                    "2" => [0, -1],
                    "3" => [-1, 0],
                    &_ => panic!("Invalid direction!"),
                });
            }
        }
    }
    println!("Found directions {:?} and distances {:?}\n\n", directions, distances);

    let col_locs = get_column_locations(&directions, &distances);
    println!("\n\nFound col_locs {:?}", col_locs);
    println!("Got area of {}", compute_area(col_locs));
    0
}

fn get_column_locations(directions: &VecDeque<[i32; 2]>, distances: &VecDeque<i32>) -> HashMap<[i32; 2], Vec<i32>> {
    let (min_row, _, max_row, _) = find_bounds(&directions, &distances);
    println!("Found bounds of {min_row} to {max_row}");

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

fn compute_area(cols: HashMap<[i32; 2], Vec<i32>>) -> i64 {
    let mut ans: i64 = 0;
    for (range, col_list) in cols.iter() {
        let mut col_list = col_list.clone();
        col_list.sort();
        let height: i64 = (range[1] - range[0]) as i64;
        if height > 0 {
            for pair_idx in 0..(col_list.len() / 2) {
                ans += height * ((1 + col_list[pair_idx * 2 + 1] - col_list[pair_idx * 2]) as i64);
                println!("Adding {}", height * ((1 + col_list[pair_idx * 2 + 1] - col_list[pair_idx * 2]) as i64));
            }
        }
    }
    return ans;

    // Have to adjust for trenches along keys
    let mut keys: Vec<[i32; 2]> = cols.clone().into_iter().map(|(key, _value)| key).collect();
    keys.sort_by_key(|x| (x[0], x[1]));
    for i in 0..keys.len() {
        let mut cols_list = cols[&keys[i]].clone();
        cols_list.sort();
        let mut curr_cols: VecDeque<i32> = VecDeque::from(cols_list);
        let mut next_cols: VecDeque<i32>;
        if i == (keys.len()  - 1) {
            next_cols = VecDeque::new();
        } else {
            let mut cols_list = cols[&keys[i + 1]].clone();
            cols_list.sort();
            next_cols = VecDeque::from(cols_list);
        }
        println!("For keys {:?} and {:?} ans started at {ans}", curr_cols, next_cols);

        while (next_cols.len() > 0) & (curr_cols.len() > 0) {
            ans += abs(next_cols.pop_front().unwrap() - curr_cols.pop_front().unwrap()) as i64;
            ans += abs(next_cols.pop_back().unwrap() - curr_cols.pop_back().unwrap()) as i64;
        }
        while next_cols.len() > 0 {
            ans += (next_cols.pop_back().unwrap() - next_cols.pop_back().unwrap()) as i64;
        }
        while curr_cols.len() > 0 {
            ans += (curr_cols.pop_back().unwrap() - curr_cols.pop_back().unwrap()) as i64;
        }
        println!("Ans ended at {ans}");
    }
    ans
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
