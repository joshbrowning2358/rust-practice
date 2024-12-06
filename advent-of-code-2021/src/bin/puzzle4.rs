mod file_reader;

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

fn parse_input(file_path: &str) -> (Vec<u16>, Vec<Vec<Vec<u16>>>) {
    let mut called_nums: Vec::<u16> = Vec::<u16>::new();
    let mut boards = Vec::new();

    let mut board: Vec<Vec<u16>> = Vec::new();
    let mut board_idx: usize = 0;
    let mut row: Vec::<u16>;

    if let Ok(mut lines) = file_reader::read_lines(file_path) {
        let line = lines.next().unwrap().unwrap();
        called_nums = line.split(',').map(|x| x.parse::<u16>().unwrap()).collect::<Vec<u16>>();

        for line in lines {
            if let Ok(board_line) = line {
                if board_idx > 0 {
                    row = board_line
                      .split(' ')
                      .filter(|s| !s.is_empty())  // Handle consecutive spaces in parsing
                      .map(|x| x.parse::<u16>().unwrap())
                      .collect::<Vec<u16>>();
                    board.push(row);
                }
                if board_idx == 5 {
                    boards.push(board);
                    board_idx = 0;
                    board = Vec::new();
                } else {
                    board_idx += 1;
                }
            }
        }
    }
    // println!("Called nums is {called_nums:?}");
    return (called_nums, boards)
}

fn part_a(file_path: &str) -> i32 {
    let (called_nums, boards) = parse_input(file_path);
    let mut fewest_nums = 100;
    let mut score = 0;

    for board in boards {
        // println!("Trying board {board:?}");
        let mut row_counts = [0, 0, 0, 0, 0];
        let mut col_counts = [0, 0, 0, 0, 0];
        let mut curr_num = 0;
        let is_winner = false;
        let mut uncalled_vals: Vec<u16> = board.clone().into_iter().flatten().collect();

        while !is_winner {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == called_nums[curr_num] {
                        row_counts[i] += 1;
                        col_counts[j] += 1;
                        uncalled_vals.retain(|value| *value != called_nums[curr_num]);
                    }
                }
            }
            if *row_counts.iter().max().unwrap() == 5 || *col_counts.iter().max().unwrap() == 5 {
                break
            }
            curr_num += 1;
            let val = called_nums[curr_num];
            // println!("Checking {val}, Row counts are {row_counts:?} and col_counts {col_counts:?}");
        }
        if curr_num < fewest_nums {
            fewest_nums = curr_num;
            // compute score
            score = (uncalled_vals.iter().sum::<u16>() as i32) * (called_nums[curr_num] as i32);
            println!("Found better board with nums={fewest_nums} and score {score} and curr_num {curr_num} and uncalled_vals {uncalled_vals:?}");
        }
    }
    return score
}

fn part_b(file_path: &str) -> i32 {
    let (called_nums, boards) = parse_input(file_path);
    let mut most_nums = 0;
    let mut score = 0;

    for board in boards {
        // println!("Trying board {board:?}");
        let mut row_counts = [0, 0, 0, 0, 0];
        let mut col_counts = [0, 0, 0, 0, 0];
        let mut curr_num = 0;
        let is_winner = false;
        let mut uncalled_vals: Vec<u16> = board.clone().into_iter().flatten().collect();

        while !is_winner {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == called_nums[curr_num] {
                        row_counts[i] += 1;
                        col_counts[j] += 1;
                        uncalled_vals.retain(|value| *value != called_nums[curr_num]);
                    }
                }
            }
            if *row_counts.iter().max().unwrap() == 5 || *col_counts.iter().max().unwrap() == 5 {
                break
            }
            curr_num += 1;
            let val = called_nums[curr_num];
            // println!("Checking {val}, Row counts are {row_counts:?} and col_counts {col_counts:?}");
        }
        if curr_num > most_nums {
            most_nums = curr_num;
            // compute score
            score = (uncalled_vals.iter().sum::<u16>() as i32) * (called_nums[curr_num] as i32);
            println!("Found worse board with nums={most_nums} and score {score} and curr_num {curr_num} and uncalled_vals {uncalled_vals:?}");
        }
    }
    return score
}
