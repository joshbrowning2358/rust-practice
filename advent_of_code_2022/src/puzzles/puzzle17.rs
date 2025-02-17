use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle

impl puzzle_solver::Solver for Puzzle {
    fn part_1(&self, file_path: &str) -> String {
        return String::from("TODO")
    }

    fn part_2(&self, file_path: &str) -> String {
        return String::from("TODO")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Day : "
    }
}

fn parse(file_path: &str) -> {
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                ... = row.
            }
        }
    }
    return
}
