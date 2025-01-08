use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle04;

impl puzzle_solver::Solver for Puzzle04 {
    fn part_1(&self, file_path: &str) -> String {
        let mut ans: i32 = 0;

        if let Ok(lines) = file_reader::read_lines(file_path) {
            for line in lines {
                if let Ok(ip) = line {
                    let (first_range, second_range) = ip.split_once(",").unwrap();
                    let (x0, x1) = first_range.split_once("-").unwrap();
                    let (y0, y1) = second_range.split_once("-").unwrap();
                    let x0 = x0.parse::<i32>().unwrap();
                    let x1 = x1.parse::<i32>().unwrap();
                    let y0 = y0.parse::<i32>().unwrap();
                    let y1 = y1.parse::<i32>().unwrap();

                    if x0 <= y0 && x1 >= y1 {
                        ans += 1;
                    }
                    else if y0 <= x0 && y1 >= x1 {
                        ans += 1;
                    }
                }
            }
        }
        ans.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let mut ans: i32 = 0;

        if let Ok(lines) = file_reader::read_lines(file_path) {
            for line in lines {
                if let Ok(ip) = line {
                    let (first_range, second_range) = ip.split_once(",").unwrap();
                    let (x0, x1) = first_range.split_once("-").unwrap();
                    let (y0, y1) = second_range.split_once("-").unwrap();
                    let x0 = x0.parse::<i32>().unwrap();
                    let x1 = x1.parse::<i32>().unwrap();
                    let y0 = y0.parse::<i32>().unwrap();
                    let y1 = y1.parse::<i32>().unwrap();

                    if x0 <= y0 && x1 >= y0 {
                        ans += 1;
                    }
                    else if y0 <= x0 && y1 >= x0 {
                        ans += 1;
                    }
                }
            }
        }
        ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("305", "811")
    }

    fn name(&self) -> &'static str {
        "Day 4: Camp Cleanup"
    }
}
