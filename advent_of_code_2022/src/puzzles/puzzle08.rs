use std::collections::HashSet;

use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle08;

const RADIX: u32 = 10;

impl puzzle_solver::Solver for Puzzle08 {
    fn part_1(&self, file_path: &str) -> String {
        let (grid, nrows, ncols) = load_grid(file_path);

        let mut visible: HashSet<(usize, usize)> = HashSet::new();
        for row_idx in 0..nrows {
            // Look from left
            let mut highest: u32 = grid[row_idx * ncols];
            let mut col_idx: usize = 0;
            visible.insert((row_idx, col_idx));
            loop {
                col_idx += 1;
                if col_idx == ncols {
                    break
                } else if grid[row_idx * ncols + col_idx] > highest {
                    highest = grid[row_idx * ncols + col_idx];
                    visible.insert((row_idx, col_idx));
                }
            }

            // Look right
            let mut col_idx = ncols - 1;
            let mut highest: u32 = grid[row_idx * ncols + col_idx];
            visible.insert((row_idx, col_idx));
            loop {
                col_idx -= 1;
                if col_idx == 0 {
                    break
                } else if grid[row_idx * ncols + col_idx] > highest {
                    highest = grid[row_idx * ncols + col_idx];
                    visible.insert((row_idx, col_idx));
                }
            }
        }

        for col_idx in 0..ncols {
            // Look from top
            let mut highest: u32 = grid[col_idx];
            let mut row_idx: usize = 0;
            visible.insert((row_idx, col_idx));
            loop {
                row_idx += 1;
                if row_idx == nrows {
                    break
                } else if grid[row_idx * ncols + col_idx] > highest {
                    highest = grid[row_idx * ncols + col_idx];
                    visible.insert((row_idx, col_idx));
                }
            }

            // Look from bottom
            let mut row_idx = nrows - 1;
            let mut highest: u32 = grid[row_idx * ncols + col_idx];
            visible.insert((row_idx, col_idx));
            loop {
                row_idx -= 1;
                if row_idx == 0 {
                    break
                } else if grid[row_idx * ncols + col_idx] > highest {
                    highest = grid[row_idx * ncols + col_idx];
                    visible.insert((row_idx, col_idx));
                }
            }
        }

        return visible.len().to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let mut best_score: i32 = 0;
        let (grid, nrows, ncols) = load_grid(file_path);
        for house_row in 1..(nrows - 1) {
            for house_col in 1..(ncols - 1) {
                let height = grid[house_row * ncols + house_col];
                let mut above = 0;
                let mut below = 0;
                let mut left = 0;
                let mut right = 0;
            
                for offset in 1..(house_row + 1) {
                    let row_idx = house_row - offset;
                    above += 1;
                    if grid[row_idx * ncols + house_col] >= height {break}
                }

                for row_idx in (house_row + 1)..nrows {
                    below += 1;
                    if grid[row_idx * ncols + house_col] >= height {break}
                }

                for offset in 1..(house_col + 1) {
                    let col_idx = house_col - offset;
                    left += 1;
                    if grid[house_row * ncols + col_idx] >= height {break}
                }

                for col_idx in (house_col + 1)..ncols {
                    right += 1;
                    if grid[house_row * ncols + col_idx] >= height {break}
                }

                let current_score = above * below * right * left;
                if current_score > best_score {best_score = current_score;}
            }
        }

        return best_score.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("1823", "211680")
    }

    fn name(&self) -> &'static str {
        "Day 8: Treetop Tree House"
    }
}

fn load_grid(file_path: &str) -> (Vec<u32>, usize, usize) {
    let mut grid_str: String = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                nrows += 1;
            }
        }
    }
    let grid: Vec<u32> = grid_str.chars().map(|x| x.to_digit(RADIX).unwrap()).collect();
    let ncols: usize = grid.len() / nrows;
    (grid, nrows, ncols)
}
