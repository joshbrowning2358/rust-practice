use std::collections::HashMap;

use crate::common::{Point, dijkstra, dijkstra_vec};
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle12;

impl puzzle_solver::Solver for Puzzle12 {
    fn part_1(&self, file_path: &str) -> String {
        let (grid, start, end) = parse(file_path);

        let mut adjacency: HashMap<Point<i16>, Vec<(Point<i16>, i32)>> = HashMap::new();
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                let mut cand: Vec<(Point<i16>, i32)> = vec![];
                if x > 0 {
                    if grid[y][x - 1] <= grid[y][x] + 1 {
                        cand.push((Point{x: x as i16 - 1, y: y as i16}, 1));
                    }
                }
                if y > 0 {
                    if grid[y - 1][x] <= grid[y][x] + 1 {
                        cand.push((Point{x: x as i16, y: y as i16 - 1}, 1));
                    }
                }
                if x < grid[0].len() - 1 {
                    if grid[y][x + 1] <= grid[y][x] + 1 {
                        cand.push((Point{x: x as i16 + 1, y: y as i16}, 1));
                    }
                }
                if y < grid.len() - 1 {
                    if grid[y + 1][x] <= grid[y][x] + 1 {
                        cand.push((Point{x: x as i16, y: y as i16 + 1}, 1));
                    }
                }
                adjacency.insert(Point{x: x as i16, y: y as i16}, cand);
            }
        }
        let (dist, _path) = dijkstra(start, end, &adjacency);
        return dist.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let (grid, _, start) = parse(file_path);

        // We want to go from highest -> lowest point so that we can stop as soon as we hit a valid starting point.
        // Thus, we have to define the adjacency backwards to what we did previously, i.e. a node is adjacent to
        // another iff it's only 1 higher or fewer
        let mut adjacency: HashMap<Point<i16>, Vec<(Point<i16>, i32)>> = HashMap::new();
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                let mut cand: Vec<(Point<i16>, i32)> = vec![];
                if x > 0 {
                    if grid[y][x - 1] >= grid[y][x] - 1 {
                        cand.push((Point{x: x as i16 - 1, y: y as i16}, 1));
                    }
                }
                if y > 0 {
                    if grid[y - 1][x] >= grid[y][x] - 1 {
                        cand.push((Point{x: x as i16, y: y as i16 - 1}, 1));
                    }
                }
                if x < grid[0].len() - 1 {
                    if grid[y][x + 1] >= grid[y][x] - 1 {
                        cand.push((Point{x: x as i16 + 1, y: y as i16}, 1));
                    }
                }
                if y < grid.len() - 1 {
                    if grid[y + 1][x] >= grid[y][x] - 1 {
                        cand.push((Point{x: x as i16, y: y as i16 + 1}, 1));
                    }
                }
                adjacency.insert(Point{x: x as i16, y: y as i16}, cand);
            }
        }

        let mut end_pts: Vec<Point<i16>> = vec![];
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                if grid[y][x] == 1 {
                    end_pts.push(Point{x: x as i16, y: y as i16});
                }
            }
        }
        let (dist, _path) = dijkstra_vec(start, end_pts, &adjacency);
        return dist.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("420", "")
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
