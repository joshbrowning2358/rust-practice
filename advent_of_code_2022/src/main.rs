use std::env;

use advent_of_code_2022::puzzle_solver::Solver;
use advent_of_code_2022::puzzles::{puzzle01, puzzle02, puzzle03, puzzle07};

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        panic!("Please call this as 'cargo run <puzzle_id> <filename>', e.g. cargo run 7 input");
    }
    let puzzle_id = &args[1].parse::<i32>().unwrap();
    let mut dataset = "input";
    if args.len() == 3 {
        dataset = &args[2];
    }
    let file_path = format!("data/puzzle{:0>2}/{dataset}.txt", puzzle_id);

    let solver = get_puzzle(*puzzle_id);
    println!("Attempting to solve {}", solver.name());

    let ans1 = solver.part_1(&file_path);
    let ans2 = solver.part_2(&file_path);
    let expected = solver.expected();
    println!("Answer to part 1 is {ans1}, matches expected = {}", ans1 == expected.0);
    println!("Answer to part 2 is {ans2}, matches expected = {}", ans2 == expected.1);
}

fn get_puzzle(day: i32) -> Box<dyn Solver> {
    return match day {
        1 => Box::new(puzzle01::Puzzle01),
        2 => Box::new(puzzle02::Puzzle02),
        3 => Box::new(puzzle03::Puzzle03),
        7 => Box::new(puzzle07::Puzzle07),
        _ => {panic!("No puzzle implemented for day {day}!");}
    }
}
