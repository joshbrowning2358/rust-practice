use std::collections::HashSet;

use crate::common::Point;
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle09;

#[derive(Debug, Clone, PartialEq, Hash)]
struct Move {
    direction: (i32, i32),
    length: i8
}

impl puzzle_solver::Solver for Puzzle09 {
    fn part_1(&self, file_path: &str) -> String {
        let moves = parse(file_path);
        let mut head = Point{x: 0i32, y: 0i32};
        let mut tail = Point{x: 0i32, y: 0i32};
        let mut tail_locs: HashSet<Point<i32>> = HashSet::new();

        for rope_move in moves {
            for _ in 0..rope_move.length {
                head.x += rope_move.direction.0;
                head.y += rope_move.direction.1;
                tail = maybe_move_tail(&head, &tail);
                tail_locs.insert(tail);
            }
        }
        return tail_locs.len().to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let moves = parse(file_path);
        let mut points = vec![Point{x: 0i32, y: 4i32}; 10];
        let mut tail_locs: HashSet<Point<i32>> = HashSet::new();

        for rope_move in moves {
            for _ in 0..rope_move.length {
                points[0].x += rope_move.direction.0;
                points[0].y += rope_move.direction.1;
                for pt_idx in 1..10 {
                    points[pt_idx] = maybe_move_tail(&points[pt_idx - 1], &points[pt_idx]);
                }
                tail_locs.insert(points[9]);
            }
        }
        return tail_locs.len().to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("6256", "2665")
    }

    fn name(&self) -> &'static str {
        "Day 9: Rope Bridge"
    }
}

fn parse(file_path: &str) -> Vec<Move> {
    let mut result = vec![];
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(dir_dist) = line {
                let (dir_code, dist) = dir_dist.split_once(" ").unwrap();
                let dir = match dir_code {
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    "U" => (0, -1),
                    "D" => (0, 1),
                    _ => panic!("Invalid dir_code {dir_code}!")
                };
                result.push(Move{direction: dir, length: dist.parse::<i8>().unwrap()});
            }
        }
    }
    return result
}

fn maybe_move_tail(head: &Point<i32>, tail: &Point<i32>) -> Point<i32>{
    let mut new_tail = tail.clone();
    if ((head.x - tail.x).abs() > 1) & ((head.y - tail.y).abs() > 1) {
        new_tail.x += (head.x - tail.x).signum();
        new_tail.y += (head.y - tail.y).signum();
    } else if (head.x - tail.x).abs() > 1 {
        // Move tail, too far behind
        new_tail.x += (head.x - tail.x).signum();
        new_tail.y = head.y;
    } else if (head.y - tail.y).abs() > 1 {
        new_tail.x = head.x;
        new_tail.y += (head.y - tail.y).signum();
    }
    return new_tail
}
