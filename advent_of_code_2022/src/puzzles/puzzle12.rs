use std::collections::HashSet;

use binary_heap_plus::BinaryHeap;

use crate::common::{Point, Node};
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle12;

impl puzzle_solver::Solver for Puzzle12 {
    fn part_1(&self, file_path: &str) -> String {
        let (grid, start, end) = parse(file_path);
        println!("Grid is {grid:?}, start {start:?}, end {end:?}");

        let mut unvisited = BinaryHeap::from_vec_cmp(
            vec![], |a: &Node<i16, i16>, b: &Node<i16, i16>| b.dist.cmp(&a.dist)
        );
        let mut visited: HashSet::<Point<i16>> = HashSet::new();
        unvisited.push(Node{pt: start, path: vec![], dist: 0});

        let mut final_dist = 1000;
        loop {
            let n = unvisited.pop().unwrap();
            if visited.contains(&n.pt) {
                println!("Already visited, skipping!");
                continue
            }
            println!("Visiting node at {:?}", n.pt);
            visited.insert(n.pt);

            if n.pt == end {
                final_dist = n.dist;
                break
            }

            // First, check for the points that are within the grid
            let mut new_points = vec![];
            if n.pt.x > 0 {
                new_points.push(Point{x: n.pt.x - 1, y: n.pt.y})
            }
            if (n.pt.x as usize) < grid[0].len() - 1 {
                new_points.push(Point{x: n.pt.x + 1, y: n.pt.y})
            }
            if n.pt.y > 0 {
                new_points.push(Point{x: n.pt.x, y: n.pt.y - 1})
            }
            if (n.pt.y as usize) < grid.len() - 1 {
                new_points.push(Point{x: n.pt.x, y: n.pt.y + 1})
            }

            // Next, validate the grid heights are valid locations
            for new_point in new_points {
                if visited.contains(&new_point) {
                    continue
                }
                if grid[new_point.y as usize][new_point.x as usize] <= grid[n.pt.y as usize][n.pt.x as usize] + 1 {
                    let mut new_path = n.path.clone();
                    new_path.push(n.clone());
                    unvisited.push(Node{pt: new_point, path: new_path, dist: n.dist + 1});
                }
                println!("Adding point {new_point:?}");
            }
            if unvisited.len() == 0 {
                panic!("Ran out of points to visit, something is wrong!");
            }
        }
        return final_dist.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let (grid, start, end) = parse(file_path);
        println!("Grid is {grid:?}, start {start:?}, end {end:?}");
        return String::from("TODO")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Day 12: Hill Climbing Algorithm"
    }
}

fn parse(file_path: &str) -> (Vec<Vec<u8>>, Point<i16>, Point<i16>) {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start: Point<i16> = Point{x: 0, y: 0};
    let mut end: Point<i16> = Point{x: 0, y: 0};

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for (row_idx, line) in lines.enumerate() {
            if let Ok(row) = line {
                let mut grid_row: Vec<u8> = vec![];
                for (col_idx, alpha_height) in row.chars().enumerate() {
                    let mut height = alpha_height.to_ascii_lowercase() as u8 - 96;
                    if alpha_height == 'S' {
                        height = 1;
                        start = Point{x: col_idx as i16, y: row_idx as i16};
                    } else if alpha_height == 'E' {
                        height = 26;
                        end = Point{x: col_idx as i16, y: row_idx as i16};
                    }
                    grid_row.push(height);
                }
                grid.push(grid_row);
            }
        }
    }
    return (grid, start, end)
}
