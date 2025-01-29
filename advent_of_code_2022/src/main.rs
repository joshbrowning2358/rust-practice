use std::env;

use advent_of_code_2022::puzzle_solver::Solver;
use advent_of_code_2022::puzzles::{puzzle01, puzzle02, puzzle03, puzzle04, puzzle05, puzzle06, puzzle08, puzzle09, puzzle10, puzzle11, puzzle12, puzzle13, puzzle14, puzzle15};

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        panic!("Please call this as 'cargo run <puzzle_id> <filename>', e.g. cargo run 7 input");
    }

    if &args[1] == "all" {
        run_all();
    } else {
        let puzzle_id = args[1].parse::<u8>().unwrap();
        let mut dataset = "input";
        if args.len() == 3 {
            dataset = &args[2];
        }
        run_day(puzzle_id, dataset);
    }
}

fn run_all() {
    for puzzle_id in 1..16 {
        if puzzle_id == 7 {
            println!("IMPLEMENT DAY 7!!!");
            continue
        }
        run_day(puzzle_id, "input");
    }
}

fn run_day(puzzle_id: u8, dataset: &str) {
    let file_path = format!("data/puzzle{:0>2}/{dataset}.txt", puzzle_id);

    let solver = get_puzzle(puzzle_id);
    println!("Attempting to solve {}", solver.name());

    let ans1 = solver.part_1(&file_path);
    let ans2 = solver.part_2(&file_path);
    let expected = solver.expected();
    println!("Answer to part 1 is {ans1}, matches expected = {}", ans1 == expected.0);
    println!("Answer to part 2 is {ans2}, matches expected = {}", ans2 == expected.1);
}

fn get_puzzle(day: u8) -> Box<dyn Solver> {
    return match day {
        1 => Box::new(puzzle01::Puzzle01),
        2 => Box::new(puzzle02::Puzzle02),
        3 => Box::new(puzzle03::Puzzle03),
        4 => Box::new(puzzle04::Puzzle04),
        5 => Box::new(puzzle05::Puzzle05),
        6 => Box::new(puzzle06::Puzzle06),
        // 7 => Box::new(puzzle07::Puzzle07),
        8 => Box::new(puzzle08::Puzzle08),
        9 => Box::new(puzzle09::Puzzle09),
        10 => Box::new(puzzle10::Puzzle10),
        11 => Box::new(puzzle11::Puzzle11),
        12 => Box::new(puzzle12::Puzzle12),
        13 => Box::new(puzzle13::Puzzle13),
        14 => Box::new(puzzle14::Puzzle14),
        15 => Box::new(puzzle15::Puzzle15),
        _ => {panic!("No puzzle implemented for day {day}!");}
    }
}
