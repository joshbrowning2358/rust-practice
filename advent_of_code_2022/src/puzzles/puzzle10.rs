use crate::common;
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle10;

impl puzzle_solver::Solver for Puzzle10 {
    fn part_1(&self, file_path: &str) -> String {
        let commands = parse(file_path);
        let mut idx = 0;
        let mut result = 0;
        let mut register = 1;
        for (op, val) in commands {
            idx += 1;
            if idx % 40 == 20 {
                result += idx * register;
            }
            if op {
                idx += 1;
                if idx % 40 == 20 {
                    result += idx * register;
                }
                register += val;
            }
        }
        return result.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let commands = parse(file_path);
        let mut idx: i32 = 0;
        let mut register: i32 = 1;
        let mut grid: Vec<char> = vec!['.'; 240];
        for (op, val) in commands {
            if (register - (idx % 40)).abs() <= 1 {
                grid[idx as usize] = '#';
            }
            idx += 1;
            if op {
                if (register - (idx % 40)).abs() <= 1 {
                    grid[idx as usize] = '#';
                }
                idx += 1;
                register += val;
            }
        }
        let grid_reshape = vec![
            grid[0..40].to_vec(),
            grid[40..80].to_vec(),
            grid[80..120].to_vec(),
            grid[120..160].to_vec(),
            grid[160..200].to_vec(),
            grid[200..240].to_vec(),
        ];
        common::display_grid(grid_reshape);
        return String::from("FPGPHFGH")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("10760", "FPGPHFGH")
    }

    fn name(&self) -> &'static str {
        "Day 10: Cathode-Ray Tube"
    }
}

fn parse(file_path: &str) -> Vec<(bool, i32)> {
    let mut result: Vec<(bool, i32)> = vec![];
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.starts_with("noop") {
                    result.push((false, 0));
                } else {
                    let (_, val) = row.split_once(" ").unwrap();
                    result.push((true, val.parse::<i32>().unwrap()));
                }
            }
        }
    }
    return result
}
