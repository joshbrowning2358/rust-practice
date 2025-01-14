use crate::common::{Point, dijkstra, Node};
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle12;

impl puzzle_solver::Solver for Puzzle12 {
    fn part_1(&self, file_path: &str) -> String {
        let (grid, start, end) = parse(file_path);
        println!("Grid is {grid:?}, start {start:?}, end {end:?}");

        fn get_neighbors(n: &Node<i16, i16>) -> Vec<Node<i16, i16>> {
            return vec![]
        }
        dijkstra::<i16, i16>(
            Node{pt: start, dist: 0, path: vec![]},
            Node{pt: end, dist: 10000, path: vec![]},
            get_neighbors 
        );
        return String::from("TODO")
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
