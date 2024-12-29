use std::collections::HashSet;

use crate::file_reader;

pub fn part_a(file_path: &str) -> usize {
    let map = parse_input(file_path);
    let width = map[0].len();
    let height = map.len();

    let mut result: usize = 0;
    for i in 0..width {
        for j in 0..height {
            if map[i][j] == 0 {
                let reachable = explore(&map, i, j, 0, HashSet::new());
                result += reachable.len();
            }
        }
    }
    return result
}

pub fn part_b(file_path: &str) -> usize {
    let map = parse_input(file_path);
    let width = map[0].len();
    let height = map.len();

    let mut result: usize = 0;
    for i in 0..width {
        for j in 0..height {
            if map[i][j] == 0 {
                result += exploreb(&map, i, j, 0);
            }
        }
    }
    return result
}

fn parse_input(file_path: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    const RADIX: u32 = 10;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                result.push(row.chars().map(|x| x.to_digit(RADIX).unwrap() as u8).collect());
            }
        }
    }

    return result
}

fn explore(map: &Vec<Vec<u8>>, i: usize, j: usize, height: u8, mut curr_visited: HashSet<(usize, usize)>) -> HashSet<(usize, usize)>{
    if height == 9 {
        curr_visited.insert((i, j));
        return curr_visited
    }

    if i >= 1 && map[i - 1][j] == height + 1 {
        curr_visited = explore(map, i - 1, j, height + 1, curr_visited);
    }
    if i < map.len() - 1 && map[i + 1][j] == height + 1 {
        curr_visited = explore(map, i + 1, j, height + 1, curr_visited);
    }
    if j >= 1 && map[i][j - 1] == height + 1 {
        curr_visited = explore(map, i, j - 1, height + 1, curr_visited);
    }
    if j < map[0].len() - 1 && map[i][j + 1] == height + 1 {
        curr_visited = explore(map, i, j + 1, height + 1, curr_visited);
    }

    return curr_visited
}

fn exploreb(map: &Vec<Vec<u8>>, i: usize, j: usize, height: u8) -> usize{
    if height == 9 {
        return 1 as usize
    }

    let mut result: usize = 0;
    if i >= 1 && map[i - 1][j] == height + 1 {
        result += exploreb(map, i - 1, j, height + 1);
    }
    if i < map.len() - 1 && map[i + 1][j] == height + 1 {
        result += exploreb(map, i + 1, j, height + 1);
    }
    if j >= 1 && map[i][j - 1] == height + 1 {
        result += exploreb(map, i, j - 1, height + 1);
    }
    if j < map[0].len() - 1 && map[i][j + 1] == height + 1 {
        result += exploreb(map, i, j + 1, height + 1);
    }

    return result 
}

