use std::env;
use advent_of_code_2024::puzzles::{puzzle1, puzzle2, puzzle3, puzzle4, puzzle5, puzzle6, puzzle7, puzzle8, puzzle9, puzzle10, puzzle11, puzzle12, puzzle13, puzzle14, puzzle15, puzzle16, puzzle17, puzzle18, puzzle19, puzzle20, puzzle21, puzzle22, puzzle23, puzzle24, puzzle25};

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        panic!("Please call this as 'cargo run puzzleX <filename>' or 'cargo run puzzleX'");
    }
    
    let puzzle_id = &args[1];
    let mut dataset = "input";
    if args.len() == 3 {
        dataset = &args[2];
    }
    let file_path = format!("data/{puzzle_id}/{dataset}.txt");

    if puzzle_id == "puzzle1" {
        let ans_a = puzzle1::part_a(&file_path);
        let ans_b = puzzle1::part_b(&file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle2" {
        let ans_a = puzzle2::part_a(&file_path);
        let ans_b = puzzle2::part_b(&file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle3" {
        let ans_a = puzzle3::part_a(&file_path);
        let ans_b = puzzle3::part_b(&file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle4" {
        let ans_a = puzzle4::part_a(&file_path);
        let ans_b = puzzle4::part_b(&file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle5" {
        let ans_a = puzzle5::part_a( & file_path);
        let ans_b = puzzle5::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle6" {
        let ans_a = puzzle6::part_a( & file_path);
        let ans_b = puzzle6::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle7" {
        let ans_a = puzzle7::part_a( & file_path);
        let ans_b = puzzle7::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle8" {
        let ans_a = puzzle8::part_a( & file_path);
        let ans_b = puzzle8::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle9" {
        let ans_a = puzzle9::part_a( & file_path);
        let ans_b = puzzle9::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle10" {
        let ans_a = puzzle10::part_a( & file_path);
        let ans_b = puzzle10::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle11" {
        let ans_a = puzzle11::part_a( & file_path);
        let ans_b = puzzle11::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle12" {
        let ans_a = puzzle12::part_a( & file_path);
        let ans_b = puzzle12::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle13" {
        let ans_a = puzzle13::part_a( & file_path);
        let ans_b = puzzle13::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle14" {
        let ans_a = puzzle14::part_a( & file_path);
        let ans_b = puzzle14::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle15" {
        let ans_a = puzzle15::part_a( & file_path);
        let ans_b = puzzle15::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle16" {
        let ans_a = puzzle16::part_a( & file_path);
        let ans_b = puzzle16::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle17" {
        let ans_a = puzzle17::part_a( & file_path);
        let ans_b = puzzle17::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle18" {
        let ans_a = puzzle18::part_a( & file_path);
        let ans_b = puzzle18::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b:?}");
    } else if puzzle_id == "puzzle19" {
        let ans_a = puzzle19::part_a( & file_path);
        let ans_b = puzzle19::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle20" {
        let ans_a = puzzle20::part_a( & file_path);
        let ans_b = puzzle20::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle21" {
        let ans_a = puzzle21::part_a( & file_path);
        let ans_b = puzzle21::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle22" {
        let ans_a = puzzle22::part_a( & file_path);
        let ans_b = puzzle22::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle23" {
        let ans_a = puzzle23::part_a( & file_path);
        let ans_b = puzzle23::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle24" {
        let ans_a = puzzle24::part_a( & file_path);
        let ans_b = puzzle24::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    } else if puzzle_id == "puzzle25" {
        let ans_a = puzzle25::part_a( & file_path);
        let ans_b = puzzle25::part_b( & file_path);
        println!("Answer to part a is {ans_a}");
        println!("Answer to part b is {ans_b}");
    }
}
