use std::collections::BinaryHeap;

use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle01;

impl puzzle_solver::Solver for Puzzle01 {
    fn part_1(&self, input: &str) -> String {
        let backpacks = parse(input);
        let mut ans: u32 = 0;
        for backpack in backpacks {
            let curr_ans = backpack.iter().sum::<u32>();
            if curr_ans > ans {
                ans = curr_ans;
            }
        }
        return ans.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let backpacks = parse(input);
        let mut heap: BinaryHeap<u32> = BinaryHeap::new();
        for backpack in backpacks {
            heap.push(backpack.iter().sum::<u32>());
        }
        let mut ans: u32 = 0;
        for _ in 0..3 {
            ans += heap.pop().unwrap();
        }
        return ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("70369", "203002")
    }

    fn name(&self) -> &'static str {
        "Day 1: Calorie Counting"
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];
    if let Ok(lines) = file_reader::read_lines(input) {
        let mut current_backpack: Vec<u32> = vec![];
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    result.push(current_backpack);
                    current_backpack = vec![];
                } else {
                    current_backpack.push(ip.parse::<u32>().unwrap());
                }
            }
        }
    }
    return result
}
