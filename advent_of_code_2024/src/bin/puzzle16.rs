use std::collections::{HashMap, HashSet};
use binary_heap_plus::BinaryHeap;
use advent_of_code_2024::common::Point;
use advent_of_code_2024::file_reader;

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
    let (map, start, end) = parse_input(file_path);

    let (dist, _) = dijkstra(&map, start, end, false);
    return dist
}

fn part_b(file_path: &str) -> i32 {
    let (map, start, end) = parse_input(file_path);

    let (_, paths) = dijkstra(&map, start, end, true);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for path in paths {
        for pt in path {
            visited.insert(pt);
        }
    }
    return visited.len() as i32
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut result = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut i: usize = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                result.push(row.chars().collect());
                if row.contains('S') {
                    start.1 = i;
                    start.0 = 0;
                    for c in row.chars() {
                        if c == 'S' {
                            break
                        }
                        start.0 += 1;
                    }
                }
                if row.contains('E') {
                    end.1 = i;
                    end.0 = 0;
                    for c in row.chars() {
                        if c == 'E' {
                            break
                        }
                        end.0 += 1;
                    }
                }
            }
            i += 1;
        }
    }

    return (result, start, end)
}

fn dijkstra(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), find_multiple: bool) -> (i32, Vec<HashSet<(usize, usize)>>) {
    let mut unvisited = BinaryHeap::from_vec_cmp(
        vec![], |a: &((usize, usize), u8, i32, HashSet<(usize, usize)>), b: &((usize, usize), u8, i32, HashSet<(usize, usize)>)| b.2.cmp(&a.2)
    );
    let mut visited: HashMap::<((usize, usize), u8), i32> = HashMap::new();
    unvisited.push((start, 1, 0, HashSet::new()));

    let mut final_dist: i32 = 10000000;
    let mut final_paths: Vec<HashSet<(usize, usize)>> = Vec::new();
    loop {
        let (curr_pos, dir, dist, mut curr_path) = unvisited.pop().unwrap();
        visited.insert((curr_pos, dir), dist);
        curr_path.insert(curr_pos);
        if (curr_pos == end) & (dist <= final_dist) {
            final_dist = dist;
            final_paths.push(curr_path.clone());
            if !find_multiple {
                break
            }
        }
        if find_multiple & (dist > final_dist) {
            break
        }
        let next_pos = match dir {
            0 => (curr_pos.0 - 1, curr_pos.1),
            1 => (curr_pos.0 + 1, curr_pos.1),
            2 => (curr_pos.0, curr_pos.1 - 1),
            3 => (curr_pos.0, curr_pos.1 + 1),
            _ => panic!("Invalid direction code!")
        };
        if map[next_pos.1][next_pos.0] != '#' {
            if !visited.contains_key(&(next_pos, dir)) {
                unvisited.push((next_pos, dir, dist + 1, curr_path.clone()));
            } else if find_multiple & (*visited.get(&(next_pos, dir)).unwrap() == dist) {
                // let old_node = unvisited.remove(&(next_pos, dir)).unwrap();
                // let mut new_path = old_node.3.clone();
                // new_path.append(curr_path);
                unvisited.push((next_pos, dir, dist + 1, curr_path.clone()));
            }
        }
        let rotated_dirs = match dir {
            0 => vec![2, 3],
            1 => vec![2, 3],
            2 => vec![0, 1],
            3 => vec![0, 1],
            _ => panic!("Invalid direction code!")
        };
        for rotated_dir in rotated_dirs {
            if !visited.contains_key(&(curr_pos, rotated_dir)) {
                unvisited.push((curr_pos, rotated_dir, dist + 1000, curr_path.clone()));
            } else if find_multiple & (*visited.get(&(curr_pos, rotated_dir)).unwrap() == dist) {
                // let old_node = unvisited.remove(&(curr_pos, rotated_dir)).unwrap();
                // let mut new_path = old_node.3.clone();
                // new_path.append(curr_path);
                unvisited.push((curr_pos, rotated_dir, dist + 1000, curr_path.clone()));
            }
        }
    }
    return (final_dist, final_paths)
}
