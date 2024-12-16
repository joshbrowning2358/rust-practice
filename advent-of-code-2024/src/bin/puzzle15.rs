use std::collections::HashSet;

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

fn part_a(file_path: &str) -> i32 {
    let (mut map, moves, (mut x, mut y)) = parse_input(file_path);
    // Move the robot around
    for robot_move in moves {
        let delta = match robot_move {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid move!"),
        };

        let mut new_pos = (x + delta.0, y + delta.1);
        while (map[new_pos.1 as usize][new_pos.0 as usize] != '.') & (map[new_pos.1 as usize][new_pos.0 as usize] != '#') {
            new_pos = (new_pos.0 + delta.0, new_pos.1 + delta.1);
        }
        let can_move = map[new_pos.1 as usize][new_pos.0 as usize] == '.';
        if can_move {
            // Moving is possible, so first set the last position visited to a box and then set the move position one
            // away to the new robot.  Note that if the robot didn't hit a box at all then this logic will first place
            // a box where the robot moved and then place the robot on that box, thus implementing the right logic.
            map[new_pos.1 as usize][new_pos.0 as usize] = 'O';
            map[(y + delta.1) as usize][(x + delta.0) as usize] = '@';
            map[y as usize][x as usize] = '.';
            x += delta.0;
            y += delta.1;
        }
        //for map_line in &map {
        //    println!("{map_line:?}");
        //}
        //println!("");
    }

    // Compute the score of the current map
    return score(&map)
}

fn part_b(file_path: &str) -> i32 {
    let (mut map, moves, (mut x, mut y)) = parse_input_b(file_path);
    // Move the robot around
    for robot_move in moves {
        //for map_row in &map {
        //    println!("{map_row:?}");
        //}
        //println!("\n");

        let delta = match robot_move {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid move!"),
        };
        let (can_move, boxes_impacted) = find_boxes(&map, (x, y), delta);
        if can_move {
            // If dir is up or down, we must sort the found boxes appropriately
            let mut boxes: Vec<(i32, i32)> = boxes_impacted.into_iter().collect();
            if delta == (0, 1) {
                boxes.sort_by(|a, b| if a.1 == b.1 {
                    b.0.partial_cmp(&a.0).unwrap()
                } else {
                    b.1.partial_cmp(&a.1).unwrap()
                });
            } else if delta == (0, -1) {
                boxes.sort_by(|a, b| if a.1 == b.1 {
                    b.0.partial_cmp(&a.0).unwrap()
                } else {
                    a.1.partial_cmp(&b.1).unwrap()
                });
            } else if delta == (1, 0) {
                boxes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap())
            } else if delta == (-1, 0) {
                boxes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            }
            for pt in boxes.iter() {
                map[(pt.1 + delta.1) as usize][(pt.0 + delta.0) as usize] = map[pt.1 as usize][pt.0 as usize];
                map[pt.1 as usize][pt.0 as usize] = '.';
            }
            // map[y as usize][x as usize] = '.';
            x += delta.0;
            y += delta.1;
        }
    }
    //for map_row in &map {
    //    println!("{map_row:?}");
    //}
    //println!("\n");

    // Compute the score of the current map
    return score(&map)
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, Vec<char>, (i32, i32)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    let mut map_done = false;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut i = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    map_done = true;
                    continue;
                }

                if map_done {
                    for c in row.chars(){
                        moves.push(c);
                    }
                } else {
                    map.push(row.chars().collect());
                    if row.contains('@') {
                        start_y = i;
                        start_x = 0;
                        for c in row.chars() {
                            if c == '@' {
                                break
                            }
                            start_x += 1;
                        }
                    }
                }
            }
            i += 1;
        }
    }

    return (map, moves, (start_x as i32, start_y as i32))
}

fn parse_input_b(file_path: &str) -> (Vec<Vec<char>>, Vec<char>, (i32, i32)) {
    let (map, moves, (x, y)) = parse_input(file_path);
    let mut new_map: Vec<Vec<char>> = Vec::new();
    for row in map {
        let mut new_row = Vec::new();
        for c in row.iter() {
            if *c == '@' {
                new_row.push('@');
                new_row.push('.');
            } else if *c == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else {
                new_row.push(*c);
                new_row.push(*c);
            }
        }
        new_map.push(new_row);
    }
    return (new_map, moves, (x * 2, y))
}

fn score(map: &Vec<Vec<char>>) -> i32 {
    let mut result: usize = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if (map[i][j] == 'O') | (map[i][j] == '[') {
                result += 100 * i + j;
            }
        }
    }
    return result as i32
}

fn find_boxes(map: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> (bool, HashSet<(i32, i32)>) {
    // Finds all boxes that a robot at position pos would impact when moving in direction dir.  If this results in the
    // robot or a box hitting a wall, such a value will be returned immediately and all impacted boxes may not be 
    // recorded.
    let mut visited = HashSet::new();
    visited.insert(pos);
    if map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == '#' {
        return (false, visited)
    } else if map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == '[' {
        if dir.0 == 0 {
            let (valid_left, visited_left) = find_boxes(map, (pos.0 + dir.0, pos.1 + dir.1), dir);
            let (valid_right, visited_right) = find_boxes(map, (pos.0 + dir.0 + 1, pos.1 + dir.1), dir);
            if (!valid_left) | (!valid_right) {
                return (false, visited)
            } else {
                visited.extend(visited_left);
                visited.extend(visited_right);
            }
        } else {
            // Just moving horizontally, don't need to handle multiple cases
            let (valid, visited_new) = find_boxes(map, (pos.0 + dir.0, pos.1 + dir.1), dir);
            if !valid {
                return (false, visited)
            } else {
                visited.extend(visited_new);
            }
        }
    } else if map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] == ']' {
        if dir.0 == 0 {
            let (valid_left, visited_left) = find_boxes(map, (pos.0 + dir.0 - 1, pos.1 + dir.1), dir);
            let (valid_right, visited_right) = find_boxes(map, (pos.0 + dir.0, pos.1 + dir.1), dir);
            if (!valid_left) | (!valid_right) {
                return (false, visited)
            } else {
                visited.extend(visited_left);
                visited.extend(visited_right);
            }
        } else {
            // Just moving horizontally, don't need to handle multiple cases
            let (valid, visited_new) = find_boxes(map, (pos.0 + dir.0, pos.1 + dir.1), dir);
            if !valid {
                return (false, visited)
            } else {
                visited.extend(visited_new);
            }
        }
    }
    return (true, visited)
}
