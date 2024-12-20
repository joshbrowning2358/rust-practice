use advent_of_code_2024::file_reader;

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

fn part_a(file_path: &str) -> i32 {
    let wordsearch = parse_input(file_path);
    let nrows = wordsearch.len() as i16;
    let ncols = wordsearch[0].len() as i16;
    let mut result: i32 = 0;

    let col_shifts: Vec<i16> = vec![1, 1, 0, -1, -1, -1, 0, 1];
    let row_shifts: Vec<i16> = vec![0, -1, -1, -1, 0, 1, 1, 1];

    for row in 0..nrows {
        for col in 0..ncols {
            if wordsearch[row as usize][col as usize] == 'X' {
                for it in col_shifts.iter().zip(row_shifts.iter()) {
                    let (col_shift, row_shift) = it;
                    let mut matched: bool = true;
                    for offset in 1..4 {
                        let curr_row = (row as i16) + row_shift * offset;
                        let curr_col = (col as i16) + col_shift * offset;
                        if curr_row > nrows - 1 || curr_col > ncols - 1|| curr_row < 0 || curr_col < 0 {
                            matched = false;
                            break
                        }
                        let letter = match offset {
                            1 => 'M',
                            2 => 'A',
                            3 => 'S',
                            _ => panic!("Unmatched letter!"),
                        };

                        if wordsearch[curr_row as usize][curr_col as usize] != letter {
                            matched = false;
                            break
                        }
                    }
                    if matched {
                        result += 1;
                    }
                }
            }
        }
    }

    return result
}

fn part_b(file_path: &str) -> i32 {
    // 2590 is too high
    let wordsearch = parse_input(file_path);
    let nrows = wordsearch.len() as i16;
    let ncols = wordsearch[0].len() as i16;
    let mut result: i32 = 0;

    let col_shifts: Vec<i16> = vec![1, -1, -1, 1];
    let row_shifts: Vec<i16> = vec![-1, -1, 1, 1];

    for row in 0..nrows {
        for col in 0..ncols {
            if wordsearch[row as usize][col as usize] == 'M' {
                for it in col_shifts.iter().zip(row_shifts.iter()) {
                    let (col_shift, row_shift) = it;
                    let mut matched: bool = true;
                    for offset in 1..3 {
                        let curr_row = (row as i16) + row_shift * offset;
                        let curr_col = (col as i16) + col_shift * offset;
                        if curr_row > nrows - 1 || curr_col > ncols - 1|| curr_row < 0 || curr_col < 0 {
                            matched = false;
                            break
                        }
                        let letter = match offset {
                            1 => 'A',
                            2 => 'S',
                            _ => panic!("Unmatched letter!"),
                        };

                        if wordsearch[curr_row as usize][curr_col as usize] != letter {
                            matched = false;
                            break
                        }
                    }

                    if matched {
                        // Now, check for a crossing MAS
                        let col_shifts2: Vec<i16> = vec![*col_shift, 0];
                        let row_shifts2: Vec<i16> = vec![0, *row_shift];
                        for it2 in col_shifts2.iter().zip(row_shifts2.iter()) {
                            let mut matched2: bool = true;
                            let (col_shift2, row_shift2) = it2;
                            let x_row = (row as i16) + row_shift2 * 2;
                            let x_col = (col as i16) + col_shift2 * 2;
                            let (row_x_shifts, col_x_shifts) = match col_shift2 {
                                0 => ([-row_shift2, ], [*col_shift,]) ,
                                _ => ([*row_shift, ], [-col_shift2,]),
                            };
                            // Check for M at start
                            if x_row > nrows - 1 || x_col > ncols - 1 || x_row < 0 || x_col < 0 {
                                continue
                            }
                            if wordsearch[x_row as usize][x_col as usize] != 'M' {
                                continue
                            }
                            for it3 in col_x_shifts.iter().zip(row_x_shifts.iter()) {
                                let mut curr_row = x_row;
                                let mut curr_col = x_col;
                                let (col_x_shift, row_x_shift) = it3;
                                for offset in 1..3 {
                                    curr_row += row_x_shift;
                                    curr_col += col_x_shift;
                                    if curr_row > nrows - 1 || curr_col > ncols - 1|| curr_row < 0 || curr_col < 0 {
                                        matched2 = false;
                                        break
                                    }
                                    let letter = match offset {
                                        1 => 'A',
                                        2 => 'S',
                                        _ => panic!("Unmatched letter!"),
                                    };
    
                                    if wordsearch[curr_row as usize][curr_col as usize] != letter {
                                        matched2 = false;
                                        break
                                    }
                                }
                                if matched2 {
                                    result += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return result / 2
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                result.push(row.chars().collect());
            }
        }
    }

    return result;
}
