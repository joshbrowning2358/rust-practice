use nalgebra::{SMatrix, matrix};
//use nalgebra::base::dimension::U25;
mod file_reader;

//type Matrix5x5 = MatrixN<i32, U25>;

fn main() {
    let full_path = file!();
    let (_, mut file_name) = full_path.rsplit_once('/').unwrap();
    (file_name, _) = file_name.split_once('.').unwrap();
    let file_path = format!("data/{file_name}/input.txt");

    let mut ans = part_a(&file_path);
    println!("Answer to {file_name} a is {ans};");

    ans = part_b(&file_path);
    println!("Answer to {file_name} b is {ans};");
}

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<SMatrix<i32, 5, 5>>) {
//fn parse_input(file_path: &str) -> Vec<i32> {
    let mut called_nums: Vec::<i32> = Vec::<i32>::new();
    let mut boards: Vec::<SMatrix<i32, 5, 5>> = Vec::<SMatrix<i32, 5, 5>>::new();

    let mut board: SMatrix<i32, 5, 5> = matrix![0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0];
    let mut board_idx: usize = 0;
    let mut row: Vec::<i32>;

    if let Ok(mut lines) = file_reader::read_lines(file_path) {
        let line = lines.next().unwrap().unwrap();
        called_nums = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for line in lines {
            if let Ok(board_line) = line {
                println!("Line is {board_line:?}");
                if board_idx > 0 {
                    row = board_line
                      .split(' ')
                      .filter(|s| !s.is_empty())  // Handle consecutive spaces in parsing
                      .map(|x| x.parse::<i32>().unwrap())
                      .collect::<Vec<i32>>();
                    for col in 0..5 {
                        board[(board_idx - 1, col)] = row[col];
                    }
                }
                if board_idx == 5 {
                    boards.push(board);
                    board_idx = 0;
                } else {
                    board_idx += 1;
                }
            }
        }
    }
    println!("Found called_nums {called_nums:?} and boards {boards:?}");
    return (called_nums, boards)
}

fn part_a(file_path: &str) -> i32 {
    let (called_nums, boards) = parse_input(file_path);
    for board in boards {
        let filled: SMatrix<i32, 5, 5> = matrix![0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0; 0, 0, 0, 0, 0];
        for called_num in called_nums {
            if let Ok([idx, idx2]) = board.iter().position(|&r| r == called_num) {
                // Only operate if row,col is found
                println!("Found idx {idx}");
                // filled[(row, col)] = 1;
            }
        }
    }
    return 0
}

fn part_b(file_path: &str) -> i32 {
    return 0
}
