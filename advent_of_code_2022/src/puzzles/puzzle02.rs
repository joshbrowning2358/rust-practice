use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle02;

const OFFSET: i32 = 23;

impl puzzle_solver::Solver for Puzzle02 {
    fn part_1(&self, file_path: &str) -> String {
        let mut ans: i32 = 0;

        if let Ok(lines) = file_reader::read_lines(file_path) {
            for line in lines {
                if let Ok(ip) = line {
                    let (theirs, yours) = ip.split_once(' ').unwrap();
                    let yours = yours.chars().next().unwrap() as i32 - OFFSET;
                    let theirs = theirs.chars().next().unwrap() as i32;
                    if yours == theirs {
                        ans += 3  // draw
                    } else if yours == theirs + 1 || yours == theirs - 2 {
                        ans += 6 // win
                    }
                    ans += match yours {
                        65 => {1},  // rock
                        66 => {2},  // paper
                        67 => {3},  //scissors
                        _ => {panic!("Invalid value!");}
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
                    let (theirs, yours) = ip.split_once(' ').unwrap();
                    let yours = yours.chars().next().unwrap() as i32;
                    let theirs = theirs.chars().next().unwrap() as i32;
                    ans += match yours {
                        88 => {0},  // lose
                        89 => {3},  // draw
                        90 => {6},  // win
                        _ => {panic!("Invalid value!");}
                    };
                
                    let mut your_shape = theirs + (yours - 89);
                    if your_shape < 65 {your_shape += 3;}
                    if your_shape > 67 {your_shape -= 3;}
                    ans += match your_shape {
                        65 => {1},  // rock
                        66 => {2},  // scissors
                        67 => {3},  // paper
                        _ => {panic!("Found invalid value for your_shape!");}
                    };
                }
            }
        }
        ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("14163", "12091")
    }

    fn name(&self) -> &'static str {
        "Day 2: Rock Paper Scissors"
    }
}
