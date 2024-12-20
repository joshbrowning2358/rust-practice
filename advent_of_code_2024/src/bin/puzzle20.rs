use binary_heap_plus::BinaryHeap;
use std::collections::{HashSet,HashMap};

use advent_of_code_2024::file_reader;
use advent_of_code_2024::common::Point;

#[derive(Debug, Clone)]
pub struct Node {
    pub pt: Point,
    pub dist: i32,
    pub path: Vec<Point>,
}

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    let ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let (map, start, end) = parse_input(file_path);
    let (_, path) = dijkstra(&map, start, end);

    // cheats is a dict mapping the size of the cheat to how many times it's occurred.
    let mut cheats: HashMap<usize, i32> = HashMap::new();
    for i in 0..path.len() {
        for j in (i + 2)..path.len() {
            let two_off_y = (path[i].x == path[j].x) & ((path[i].y == path[j].y + 2) | (path[j].y == path[i].y + 2));
            let two_off_x = ((path[i].x == path[j].x + 2) | (path[j].x == path[i].x + 2)) & (path[i].y == path[j].y);
            if two_off_x | two_off_y {
                let key = j - i - 2;
                let val: &mut i32 = cheats.entry(key).or_insert(0);
                *val += 1;
            }
        }
    }
    let mut result: i32 = 0;
    for (cheat_size, cheat_count) in cheats.into_iter() {
        if cheat_size >= 100 {result += cheat_count}
    }
    return result 
}

fn part_b(file_path: &str) -> i32 {
    let (map, start, end) = parse_input(file_path);
    let (_, path) = dijkstra(&map, start, end);

    // cheats is a dict mapping the size of the cheat to how many times it's occurred.
    let mut cheats: HashMap<i32, i32> = HashMap::new();
    for i in 0..path.len() {
        for j in (i + 2)..path.len() {
            let delta_x = (path[i].x - path[j].x).abs();
            let delta_y = (path[i].y - path[j].y).abs();
            if delta_x + delta_y <= 20 { 
                let key = (j as i32) - (i as i32) - delta_x - delta_y;
                let val: &mut i32 = cheats.entry(key).or_insert(0);
                *val += 1;
            }
        }
    }
    let mut result: i32 = 0;
    for (cheat_size, cheat_count) in cheats.into_iter() {
        if cheat_size >= 100 {result += cheat_count}
    }
    return result
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, Point, Point) {
    let mut result = Vec::new();
    let mut start: Point = Point{x: 0, y: 0};
    let mut end: Point = Point{x: 0, y: 0};

    let mut i: usize = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                result.push(row.chars().collect());
                if row.contains('S') {
                    start.y = i as i32;
                    start.x = 0 as i32;
                    for c in row.chars() {
                        if c == 'S' {
                            break
                        }
                        start.x += 1;
                    }
                }
                if row.contains('E') {
                    end.y = i as i32;
                    end.x = 0 as i32;
                    for c in row.chars() {
                        if c == 'E' {
                            break
                        }
                        end.x += 1;
                    }
                }
            }
            i += 1;
        }
    }

    return (result, start, end)
}

fn dijkstra(map: &Vec<Vec<char>>, start: Point, end: Point) -> (i32, Vec<Point>) {
    let grid_size = (map[0].len() as i32, map.len() as i32);
    let mut unvisited = BinaryHeap::from_vec_cmp(
        vec![], |a: &Node, b: &Node| b.dist.cmp(&a.dist)
    );
    let mut visited: HashSet::<Point> = HashSet::new();

    unvisited.push(Node{pt: start.clone(), dist: 0, path: vec![start]});

    let final_dist: i32;
    let final_path: Vec<Point>;
    loop {
        let curr_node = unvisited.pop().unwrap();
        if visited.contains(&curr_node.pt) {
            continue
        }
        visited.insert(curr_node.pt.clone());
        if curr_node.pt == end {
            final_dist = curr_node.dist;
            final_path = curr_node.path;
            break
        }
        let cand_positions = vec![
            Point{x: curr_node.pt.x - 1, y: curr_node.pt.y},
            Point{x: curr_node.pt.x + 1, y: curr_node.pt.y},
            Point{x: curr_node.pt.x, y: curr_node.pt.y - 1},
            Point{x: curr_node.pt.x, y: curr_node.pt.y + 1},
        ];
        for cand in cand_positions {
            if visited.contains(&cand) {
                continue
            }
            if (cand.x < 0) | (cand.y < 0) | (cand.x > grid_size.0) | (cand.y > grid_size.1) {
                continue
            }
            if map[cand.y as usize][cand.x as usize] == '#' { 
                continue
            }
            let mut new_path = curr_node.path.clone();
            new_path.push(cand.clone());
            unvisited.push(Node{pt: cand.clone(), dist: curr_node.dist + 1, path: new_path});
        }
        if unvisited.len() == 0 {
            return (-1, vec![])
        }
    }
    return (final_dist, final_path)
}
