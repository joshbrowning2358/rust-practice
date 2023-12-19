use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle10/easy.txt"; let s_char = 'F';
    //let file_path = "data/puzzle10/example.txt"; let s_char = 'F';
    let file_path = "data/puzzle10/input.txt"; let s_char = 'J';
    //let file_path = "data/puzzle10/hard.txt";

    let mut ans = puzzle10a(file_path);
    println!("Answer to puzzle 10a is {ans};");

    ans = puzzle10b(file_path, s_char);
    println!("Answer to puzzle 10b is {ans};");
}

fn puzzle10a(file_path: &str) -> i32 {
    let mut s_row: i32 = -1;
    let mut s_col: i32 = -1;
    let mut pipes = String::new();

    let mut row_idx: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                pipes.push_str(&ip.to_string());
                if ip.contains("S") {
                    s_row = row_idx;
                    s_col = ip.find("S").unwrap() as i32;
                }
                row_idx += 1;
            }
        }
    }

    let pipes: Vec<char> = pipes.chars().collect();
    let n_rows = row_idx as i32;
    let n_cols = pipes.len() as i32 / n_rows;

    // Navigate the maze
    let mut row = s_row as i32;
    let mut col = s_col as i32;
    let mut dir: [i32; 2] = [0, 0];
    if pipes[((row - 1) * n_cols + col) as usize] == '|' {
        dir = [-1, 0];
        row -= 1;
    } else if pipes[((row - 1) * n_cols + col) as usize] == '7' {
        dir = [0, -1];
        col -= 1;
    } else if pipes[(row * n_cols + col + 1) as usize] == '-' {
        dir = [0, 1];
        col += 1;
    } else if pipes[((row + 1) * n_cols + col) as usize] == '|' {
        dir = [1, 0];
        row += 1;
    }
    // Missing some conds but this should work with our input
    let mut n_steps = 1;
    loop { // Continue moving until we hit S
        let next_spot = pipes[(row * n_cols + col) as usize];

        match next_spot {
            'S' => {break},
            '-' => {
                match dir {
                    [0, 1] => {},
                    [0, -1] => {},
                    _ => {panic!("Invalid direction for - pipe!");}
                }
            },
            '7' => {
                match dir {
                    [-1, 0] => {dir = [0, -1];},
                    [0, 1] => {dir = [1, 0];},
                    _ => {panic!("Invalid direction for 7 pipe!");}
                }
            },
            '|' => {
                match dir {
                    [1, 0] => {},
                    [-1, 0] => {},
                    _ => {panic!("Invalid direction for | pipe!");}
                }
            },
            'J' => {
                match dir {
                    [1, 0] => {dir = [0, -1];},
                    [0, 1] => {dir = [-1, 0];},
                    _ => {panic!("Invalid direction for J pipe!");}
                }
            },
            'L' => {
                match dir {
                    [0, -1] => {dir = [-1, 0];},
                    [1, 0] => {dir = [0, 1];},
                    _ => {panic!("Invalid direction for L pipe!");}
                }
            },
            'F' => {
                match dir {
                    [-1, 0] => {dir = [0, 1];},
                    [0, -1] => {dir = [1, 0];},
                    _ => {panic!("Invalid direction for F pipe!");}
                }
            },
            _ => {panic!("Invalid pipe, found {next_spot}!");}
        }
        // Take step based on determined direction
        row += dir[0];
        col += dir[1];

        n_steps += 1;
    }

    n_steps / 2
}

fn puzzle10b(file_path: &str, s_char: char) -> i32 {
    let mut s_row: i32 = -1;
    let mut s_col: i32 = -1;
    let mut pipes = String::new();

    let mut row_idx: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                pipes.push_str(&ip.to_string());
                if ip.contains("S") {
                    s_row = row_idx;
                    s_col = ip.find("S").unwrap() as i32;
                }
                row_idx += 1;
            }
        }
    }

    let pipes: Vec<char> = pipes.chars().collect();
    let n_rows = row_idx as i32;
    let n_cols = pipes.len() as i32 / n_rows;

    let mut visited: HashSet<[i32; 2]> = HashSet::new();

    // Navigate the maze
    let mut row = s_row as i32;
    let mut col = s_col as i32;
    let mut dir: [i32; 2] = [0, 0];
    visited.insert([row, col]);
    if pipes[((row - 1) * n_cols + col) as usize] == '|' {
        dir = [-1, 0];
        row -= 1;
    } else if pipes[((row - 1) * n_cols + col) as usize] == '7' {
        dir = [0, -1];
        col -= 1;
    } else if pipes[(row * n_cols + col + 1) as usize] == '-' {
        dir = [0, 1];
        col += 1;
    } else if pipes[((row + 1) * n_cols + col) as usize] == '|' {
        dir = [1, 0];
        row += 1;
    }
    // Missing some conds but this should work with our input
    loop { // Continue moving until we hit S
        visited.insert([row, col]);
        let next_spot = pipes[(row * n_cols + col) as usize];

        match next_spot {
            'S' => {break}
            '-' => {
                match dir {
                    [0, 1] => {},
                    [0, -1] => {},
                    _ => {panic!("Invalid direction for - pipe!");}
                }
            },
            '7' => {
                match dir {
                    [-1, 0] => {dir = [0, -1];},
                    [0, 1] => {dir = [1, 0];},
                    _ => {panic!("Invalid direction for 7 pipe!");}
                }
            },
            '|' => {
                match dir {
                    [1, 0] => {},
                    [-1, 0] => {},
                    _ => {panic!("Invalid direction for | pipe!");}
                }
            },
            'J' => {
                match dir {
                    [1, 0] => {dir = [0, -1];},
                    [0, 1] => {dir = [-1, 0];},
                    _ => {panic!("Invalid direction for J pipe!");}
                }
            },
            'L' => {
                match dir {
                    [0, -1] => {dir = [-1, 0];},
                    [1, 0] => {dir = [0, 1];},
                    _ => {panic!("Invalid direction for L pipe!");}
                }
            },
            'F' => {
                match dir {
                    [-1, 0] => {dir = [0, 1];},
                    [0, -1] => {dir = [1, 0];},
                    _ => {panic!("Invalid direction for F pipe!");}
                }
            },
            _ => {panic!("Invalid pipe, found {next_spot}!");}
        }
        // Take step based on determined direction
        row += dir[0];
        col += dir[1];
    }
    println!("Found {} visited nodes!", visited.len());

    // Compute interior area
    let mut area = 0;
    let mut seen: char = 'x';
    for i in 0..n_rows {
        let mut inside: bool = false;
        for j in 0..n_cols {
            if !visited.contains(&[i, j]) {
                // Not on the path, just need to add 1 if we're inside
                if inside {area += 1; println!("Adding to area at pos {i},{j}");}
            } else {
                match pipes[(i * n_cols + j) as usize] {
                    '-' => {continue;}, // - won't impact inside/outside-ness
                    '|' => {inside = !inside}, // | => always flip
                    'L' => {
                        seen = 'L';
                    },
                    'J' => {
                        match seen {
                            'L' => {seen = 'x';},
                            'F' => {
                                inside = !inside;
                                seen = 'x';
                            }
                            _ => {panic!("Found a J but seen is {seen}");}
                        }
                    },
                    '7' => {
                        match seen {
                            'L' => {
                                seen = 'x';
                                inside = !inside;
                            },
                            'F' => {
                                seen = 'x';
                            }
                            _ => {panic!("Found a 7 but seen is {seen}");}
                        }
                    },
                    'F' => {
                        seen = 'F';
                    }
                    'S' => {
                        if s_char == 'F' {
                            seen = 'F';
                        } else {
                            match seen {
                                'L' => {seen = 'x';}
                                'F' => {
                                    inside = !inside;
                                    seen = 'x';
                                },
                                _ => {panic!("Found S (i.e. a J) but seen is {seen}");}
                            }
                        }
                    }
                    _ => {panic!("Invalid pipe value found!");}
                }
            }
        }
    }
    area
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
