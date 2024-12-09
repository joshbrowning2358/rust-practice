use std::collections::{HashMap, HashSet};
use num::integer::gcd;

mod file_reader;

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
    // 340 is too high
    let mut result: HashSet::<(i32, i32)> = HashSet::new();
    let (width, height, antennas) = parse_input(file_path);
    // Iterate over antenna type
    for (_, locs) in antennas.into_iter() {
        // Iterate over all pairs of antennas
        for i in 0..locs.len() - 1 {
            for j in (i + 1)..locs.len() {
                let diff_x = locs[i].0 - locs[j].0;
                let diff_y = locs[i].1 - locs[j].1;
                if in_bounds(locs[i].0 + diff_x, locs[i].1 + diff_y, width, height) {
                    result.insert((locs[i].0 + diff_x, locs[i].1 + diff_y));
                }
                if in_bounds(locs[j].0 - diff_x, locs[j].1 - diff_y, width, height) {
                    result.insert((locs[j].0 - diff_x, locs[j].1 - diff_y));
                }
            }
        }
    }
    return result.len() as i32
}

fn part_b(file_path: &str) -> i32 {
    let mut result: HashSet::<(i32, i32)> = HashSet::new();
    let (width, height, antennas) = parse_input(file_path);
    // Iterate over antenna type
    for (_, locs) in antennas.into_iter() {
        // Iterate over all pairs of antennas
        for i in 0..locs.len() - 1 {
            for j in (i + 1)..locs.len() {
                let (diff_x, diff_y) = reduce_diffs(locs[i].0 - locs[j].0, locs[i].1 - locs[j].1);
                // iterate in dir of 1st
                let mut curr_pos = locs[i];
                loop {
                    result.insert(curr_pos);
                    curr_pos.0 -= diff_x;
                    curr_pos.1 -= diff_y;
                    if !in_bounds(curr_pos.0, curr_pos.1, width, height) {
                        break
                    }
                }

                // iterate in dir of 2nd
                let mut curr_pos = locs[j];
                loop {
                    result.insert(curr_pos);
                    curr_pos.0 += diff_x;
                    curr_pos.1 += diff_y;
                    if !in_bounds(curr_pos.0, curr_pos.1, width, height) {
                        break
                    }
                }
            }
        }
    }
    return result.len() as i32;
}

fn parse_input(file_path: &str) -> (usize, usize, HashMap<char, Vec<(i32, i32)>>) {
    let mut width = 0;
    let mut height: usize = 0;
    let mut antennas = HashMap::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let mut curr_col = 0;
                for c in row.chars() {
                    if c != '.' {
                        antennas.entry(c)
                            .and_modify(|locs: &mut Vec<(i32, i32)>| locs.push((height as i32, curr_col as i32)))
                            .or_insert(vec![(height as i32, curr_col)]);
                    }
                    curr_col += 1;
                }
                height += 1;
                width = row.len();
            }
        }
    }

    return (width, height, antennas)
}

fn in_bounds(x: i32, y: i32, max_x: usize, max_y: usize) -> bool {
    if x < 0 || y < 0 {
        return false
    }
    if x >= max_x as i32 || y >= max_y as i32 {
        return false
    }
    return true
}

fn reduce_diffs(diff_x: i32, diff_y: i32) -> (i32, i32) {
    let diff_gcd = gcd(diff_x, diff_y);
    return (diff_x / diff_gcd, diff_y / diff_gcd)
}
