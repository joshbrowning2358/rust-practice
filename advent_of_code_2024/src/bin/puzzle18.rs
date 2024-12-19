use binary_heap_plus::BinaryHeap;
use std::collections::HashSet;

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
    println!("Answer to {file_name} b is {ans:?};");
}

fn part_a(file_path: &str) -> i32 {
    // 214 is too low
    // 1276 is too high
    let (locs, grid_size) = parse_input(file_path, true);
    let locs = locs[..1024].to_vec();
    //let (locs, grid_size) = parse_input(file_path, false);

    let mut grid = vec![vec!['.'; (grid_size.x + 1) as usize]; (grid_size.y + 1) as usize];
    for loc in &locs {
        grid[loc.y as usize][loc.x as usize] = '#';
    }

    let start = Point{x: 0, y: 0};
    let (dist, path) = dijkstra(&locs, start, grid_size);
    
    for pt in path {
        if grid[pt.y as usize][pt.x as usize] == '#' {
            panic!("Path runs over a #");
        } else {
            grid[pt.y as usize][pt.x as usize] = 'O'
        }
    }

    for grid_row in &grid {
        let s: String = grid_row.into_iter().collect();
        println!("{s:?}");
    }

    return dist
}

fn part_b(file_path: &str) -> Point {
    let (locs, grid_size) = parse_input(file_path, true);
    let start = Point{x: 0, y: 0};
    let mut lower = 0;
    let mut upper = locs.len();
    let mut cand = (lower + upper) / 2;

    while upper > lower + 1 {
        let curr_locs = locs[..cand].to_vec();
        let (dist, _) = dijkstra(&curr_locs, start.clone(), grid_size.clone());
        
        if dist == -1 {
            upper = cand;
        } else {
            lower = cand;
        }
        cand = (upper + lower) / 2
    }
    let ans = locs[lower].clone();

    return ans
}

fn parse_input(file_path: &str, all_pts: bool) -> (Vec<Point>, Point) {
    let mut result = Vec::new();

    let grid_size: Point;
    let mut num_pts: usize;
    if file_path.contains("easy1") {
        grid_size = Point{x: 6, y: 6};
        num_pts = 12;
    } else if file_path.contains("input") {
        grid_size = Point{x: 70, y: 70};
        num_pts = 1024;
    } else {
        panic!("Unrecognized file_path {file_path}");
    }
    if all_pts {
        num_pts = 100000;
    }

    let mut found_pts: usize = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let (x, y) = row.split_once(',').unwrap();
                let new_pt = Point{x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap()};
                result.push(new_pt);

                found_pts += 1;
                if found_pts == num_pts {
                    break
                }
            }
        }
    }
    return (result, grid_size)
}

fn dijkstra(locs: &Vec<Point>, start: Point, end: Point) -> (i32, Vec<Point>) {
    let mut unvisited = BinaryHeap::from_vec_cmp(
        vec![], |a: &Node, b: &Node| b.dist.cmp(&a.dist)
    );
    let mut visited: HashSet::<Point> = HashSet::new();
    // let mut to_explore_dists: HashMap::<Point, i32> = HashMap::new();

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
            if (cand.x < 0) | (cand.y < 0) | (cand.x > end.x) | (cand.y > end.y) {
                continue
            }
            if locs.contains(&cand) {
                continue
            }
            //if let Some(val) = to_explore_dists.get(&cand) {
            //    if val <= &(curr_node.dist + 1) {
            //        continue
            //    }
            //}
            let mut new_path = curr_node.path.clone();
            new_path.push(cand.clone());
            unvisited.push(Node{pt: cand.clone(), dist: curr_node.dist + 1, path: new_path});
            // to_explore_dists.insert(cand, curr_node.dist + 1);
        }
        if unvisited.len() == 0 {
            return (-1, vec![])
        }
    }
    return (final_dist, final_path)
}
