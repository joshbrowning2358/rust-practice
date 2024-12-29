use std::collections::HashMap;
use memoize::memoize;

use crate::file_reader;

pub fn part_a(file_path: &str) -> i64 {
    let stones = parse_input(file_path);
    let n_splits = 25;

    let mut cache: HashMap<(u64, u8), i64> = HashMap::new();

    let mut result: i64 = 0;
    for stone in stones {
        let cnt = split_stone(stone, n_splits, &mut cache);
        result += cnt;
        // println!("Cache is {cache:?}");
    }
    return result
}

pub fn part_b(file_path: &str) -> i64 {
    let stones = parse_input(file_path);
    let n_splits = 75;

    let mut result: i64 = 0;
    for stone in stones {
        let cnt = split_stone_mem(stone, n_splits);
        result += cnt;
        // println!("Cache is {cache:?}");
    }
    return result
}

fn parse_input(file_path: &str) -> Vec<u64> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(vals) = line {
                result = vals.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            }
        }
    }

    return result;
}

fn split_stone(stone: u64, n_splits: u8, cache: &mut HashMap<(u64, u8), i64>) -> i64 {
    if let Some(cached_v) = cache.get(&(stone, n_splits)) {
        return *cached_v
    }

    let mut log: u32 = 0;
    if stone > 0 {
        log = stone.ilog10();
    }
    let ans;
    if n_splits == 1 {
        if stone == 0 {
            ans = 1;
        } else if log % 2 == 1 {
            ans = 2;
        } else {
            ans = 1;
        }
    } else {
        if stone == 0 {
            ans = split_stone(1, n_splits - 1, cache);
        } else if log % 2 == 1 {
            let large = stone / 10u64.pow((log + 1) / 2);
            let small = stone - large * 10u64.pow((log + 1) / 2);
            // println!("Splitting {stone} into {large} and {small}");
            ans = split_stone(large, n_splits - 1, cache) + split_stone(small, n_splits - 1, cache);
        } else {
            ans = split_stone(stone * 2024, n_splits - 1, cache);
        }
    }
    cache.insert((stone, n_splits), ans);
    return ans
}

#[memoize]
fn split_stone_mem(stone: u64, n_splits: u8) -> i64 {
    let mut log: u32 = 0;
    if stone > 0 {
        log = stone.ilog10();
    }
    let ans;
    if n_splits == 1 {
        if stone == 0 {
            ans = 1;
        } else if log % 2 == 1 {
            ans = 2;
        } else {
            ans = 1;
        }
    } else {
        if stone == 0 {
            ans = split_stone_mem(1, n_splits - 1);
        } else if log % 2 == 1 {
            let large = stone / 10u64.pow((log + 1) / 2);
            let small = stone - large * 10u64.pow((log + 1) / 2);
            // println!("Splitting {stone} into {large} and {small}");
            ans = split_stone_mem(large, n_splits - 1) + split_stone_mem(small, n_splits - 1);
        } else {
            ans = split_stone_mem(stone * 2024, n_splits - 1);
        }
    }
    return ans
}
