use crate::file_reader;
use crate::puzzle_solver;
use crate::common::Point;

pub struct Puzzle14;

impl puzzle_solver::Solver for Puzzle14 {
    fn part_1(&self, file_path: &str) -> String {
        let (mut grid, _) = parse(file_path);
        //for (idx, row) in grid.iter().enumerate() {
            // println!("{} {} {} {} {} {} {} {}", row[492], row[493], row[494], row[495], row[496], row[497], row[498], row[499]);
            // println!("{:?}", row[490..510].iter().map(|x| *x as i32).collect::<Vec<i32>>());
            // if idx > 10 {
            //     break;
            // }
        //}

        let mut fall_cnt: i32 = 0;
        'all: loop {
            let mut sand_start = Point{x: 500, y: 0};
            loop {
                if sand_start.y == 599 {
                    break 'all;
                } else if !grid[sand_start.y + 1][sand_start.x] {
                    sand_start.y += 1;
                } else if !grid[sand_start.y + 1][sand_start.x - 1] {
                    sand_start.y += 1;
                    sand_start.x -= 1;
                } else if !grid[sand_start.y + 1][sand_start.x + 1] {
                    sand_start.y += 1;
                    sand_start.x += 1;
                } else {
                    grid[sand_start.y][sand_start.x] = true;
                    fall_cnt += 1;
                    break;
                }
            }

        }
        return fall_cnt.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let (mut grid, max_y) = parse(file_path);
        for col in 0..800 {
            grid[max_y as usize + 2][col] = true;
        }

        let mut fall_cnt: i32 = 0;
        while !grid[0][500] {
            let mut sand_start = Point{x: 500, y: 0};
            loop {
                if !grid[sand_start.y + 1][sand_start.x] {
                    sand_start.y += 1;
                } else if !grid[sand_start.y + 1][sand_start.x - 1] {
                    sand_start.y += 1;
                    sand_start.x -= 1;
                } else if !grid[sand_start.y + 1][sand_start.x + 1] {
                    sand_start.y += 1;
                    sand_start.x += 1;
                } else {
                    grid[sand_start.y][sand_start.x] = true;
                    fall_cnt += 1;
                    break;
                }
            }
        }
        return fall_cnt.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("862", "28744")
    }

    fn name(&self) -> &'static str {
        "Day 14: Regolith Reservoir"
    }
}

fn parse(file_path: &str) -> (Vec<Vec<bool>>, i32) {
    let mut grid = vec![vec![false; 800]; 600];
    let mut max_y = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let pts: Vec<&str> = row.split(" -> ").collect();
                let (x_str, y_str) = pts[0].split_once(',').unwrap();
                let mut x = x_str.parse::<i32>().unwrap();
                let mut y = y_str.parse::<i32>().unwrap();
                if y > max_y {
                    max_y = y;
                }
                grid[y as usize][x as usize] = true;
                for pt in &pts[1..] {
                    let (x_str, y_str) = pt.split_once(',').unwrap();
                    let x_end = x_str.parse::<i32>().unwrap();
                    let y_end = y_str.parse::<i32>().unwrap();
                    if y_end > max_y {
                        max_y = y_end;
                    }
                    while (x != x_end) | (y != y_end) {
                        x += (x_end - x).signum();
                        y += (y_end - y).signum();
                        grid[y as usize][x as usize] = true;
                    }
                }
            }
        }
    }
    return (grid, max_y)
}
