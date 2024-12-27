use std::collections::HashMap; 
use std::collections::HashSet;

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
    let (mut map, mut loc) = parse_input(file_path);

    loop {
        let dir: (i32, i32) = match map[loc.0 as usize][loc.1 as usize] {
            '^' => (-1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            _ => panic!("Found invalid char at guard position!"),
        };
        let next_loc = (loc.0 + dir.0, loc.1 + dir.1);
        if next_loc.0 < 0 || next_loc.1 < 0 || next_loc.0 > map.len() as i32 - 1 || next_loc.1 > map[0].len() as i32 -1 {
            map[loc.0 as usize][loc.1 as usize] = 'X';
            break
        }
        if map[next_loc.0 as usize][next_loc.1 as usize] == '#' {
            map[loc.0 as usize][loc.1 as usize] = match map[loc.0 as usize][loc.1 as usize] {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Found invalid char when rotating!"),
            };
        } else {
            map[next_loc.0 as usize][next_loc.1 as usize] = map[loc.0 as usize][loc.1 as usize];
            map[loc.0 as usize][loc.1 as usize] = 'X';
            loc = next_loc;
        }
        //for row in 0..map.len() {
        //    let map_row = &map[row];
        //    println!("{map_row:?}");
        //}
        //println!(" ");
    }

    let mut result = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'X' {
                result += 1
            }
        }
    }

    return result
}

fn part_b(file_path: &str) -> i32 {
    // 2176 is too high
    // 2037 is too high
    let (mut map, mut loc) = parse_input(file_path);
    let guard_start = loc;
    let mut visited = HashMap::new();
    for dir in ['^', '>', 'v', '<'] {
        visited.insert(dir, vec![vec![false; map[0].len()]; map.len()]);
    }
    let mut result: HashSet::<(i32, i32)> = HashSet::new();
    loop {
        let dir: (i32, i32) = match map[loc.0 as usize][loc.1 as usize] {
            '^' => (-1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            _ => panic!("Found invalid char at guard position!"),
        };
        let next_loc = (loc.0 + dir.0, loc.1 + dir.1);

        if next_loc.0 < 0 || next_loc.1 < 0 || next_loc.0 > map.len() as i32 - 1 || next_loc.1 > map[0].len() as i32 -1 {
            map[loc.0 as usize][loc.1 as usize] = 'X';
            break
        }

        // Check if we loop if we place an obstruction right in front of the guard
        let already_obst = map[next_loc.0 as usize][next_loc.1 as usize] == '#';
        let already_visited = visited.get(&'^').unwrap()[next_loc.0 as usize][next_loc.1 as usize] |
            visited.get(&'>').unwrap()[next_loc.0 as usize][next_loc.1 as usize] |
            visited.get(&'v').unwrap()[next_loc.0 as usize][next_loc.1 as usize] |
            visited.get(&'<').unwrap()[next_loc.0 as usize][next_loc.1 as usize];
        if !already_obst & !already_visited {
            let mut map_copy = map.clone();
            map_copy[next_loc.0 as usize][next_loc.1 as usize] = '#';
            if found_loop(map_copy, loc, visited.clone()) {
                if next_loc != guard_start {
                    result.insert(next_loc);
                }
            }
        }
        
        let arrow = map[loc.0 as usize][loc.1 as usize];
        visited.get_mut(&arrow).unwrap()[loc.0 as usize][loc.1 as usize] = true;
        if map[next_loc.0 as usize][next_loc.1 as usize] == '#' {
            map[loc.0 as usize][loc.1 as usize] = match map[loc.0 as usize][loc.1 as usize] {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Found invalid char when rotating!"),
            };
        } else {
            map[next_loc.0 as usize][next_loc.1 as usize] = arrow;
            loc = next_loc;
        }
    }
    return result.len() as i32
}

fn found_loop(mut map: Vec<Vec<char>>, mut loc: (i32, i32), mut visited: HashMap::<char, Vec<Vec<bool>>>) -> bool {
    let mut is_loop: bool = false;
    // println!("Starting at loc {loc:?}");
    loop {
        let dir: (i32, i32) = match map[loc.0 as usize][loc.1 as usize] {
            '^' => (-1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            _ => panic!("Found invalid char at guard position!"),
        };
        // println!("Found dir {dir:?}");
        let next_loc = (loc.0 + dir.0, loc.1 + dir.1);
        if next_loc.0 < 0 || next_loc.1 < 0 || next_loc.0 > map.len() as i32 - 1 || next_loc.1 > map[0].len() as i32 -1 {
            map[loc.0 as usize][loc.1 as usize] = 'X';
            break
        }
        let arrow = map[loc.0 as usize][loc.1 as usize];
        if map[next_loc.0 as usize][next_loc.1 as usize] == '#' {
            map[loc.0 as usize][loc.1 as usize] = match map[loc.0 as usize][loc.1 as usize] {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Found invalid char when rotating!"),
            };
            visited.get_mut(&arrow).unwrap()[loc.0 as usize][loc.1 as usize] = true;
        } else {
            map[next_loc.0 as usize][next_loc.1 as usize] = arrow;
            if visited.get_mut(&arrow).unwrap()[loc.0 as usize][loc.1 as usize] == true {
                is_loop = true;
                break
            }
            visited.get_mut(&arrow).unwrap()[loc.0 as usize][loc.1 as usize] = true;
            loc = next_loc;
        }
        // println!("Moved to {loc:?}");
    }
    return is_loop
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut result = Vec::new();

    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut found_start: bool = false;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(map_line) = line {
                result.push(map_line.chars().collect());
                if map_line.contains('^') {
                    col = 0;
                    for c in map_line.chars() {
                        if c == '^' {
                            found_start = true;
                            break
                        }
                        col += 1;
                    }
                }
            }
            if !found_start {
                row +=1;
            }
        }
    }

    return (result, (row, col))
}
