use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "data/puzzle22/example.txt";
    //let file_path = "data/puzzle22/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

#[derive(Debug)]
struct Brick {
    pts: Vec<[i32; 3]>
}

fn part_a(file_path: &str) -> i32 {
    let bricks = parse_input(file_path);
    println!("Parsed bricks {:?}", bricks);
    0
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                ans += ip.len() as i32;
            }
        }
    }
    ans
}

fn parse_input(file_path: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (start_str, end_str) = ip.split_once('~').unwrap();
                let start: Vec<i32> = start_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                let end: Vec<i32> = end_str.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

                let mut x = start[0];
                let mut y = start[1];
                let mut z = start[2];
                let mut pts: Vec<[i32; 3]> = Vec::new();
                while (x != end[0]) | (y != end[1]) | (z != end[2]) {
                    pts.push([x, y, z]);
                    if x < end[0] {x += 1;}
                    if y < end[1] {y += 1;}
                    if z < end[2] {z += 1;}
                }
                pts.push([x, y, z]);
                bricks.push(Brick{pts: pts});
            }
        }
    }
    bricks.sort_by_key(|b| b.pts[0][2]);
    bricks
}

fn drop_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let dropped_bricks: Vec<Brick> = Vec::new();
    // supports maps brick_id -> a vector of brick_ids that it supports
    let supports: HashMap<i32, Vec<i32>>;
    
    for brick in bricks {
        let mut max_height = 1;
        let mut support: Vec<i32> = Vec::new();
        for pt in &brick.pts {
            for (dropped_idx, dropped_brick) in dropped_bricks.iter().enumerate() {
                for dropped_pt in &dropped_brick.pts {
                    if (pt[0] == dropped_pt[0]) & (pt[0] == dropped_pt[0]) {
                        // Potentially caught by brick!  Check heights
                        if dropped_pt[2] > max_height - 1 {
                            max_height = dropped_pt[2] + 1;
                            support = vec![dropped_idx as i32; 1];
                        } else if dropped_pt[2] == max_height - 1 {
                            support.push(dropped_idx as i32);
                        }
                    // TODO: Continue from here!
                    }
                }
            }
        }
    }

    dropped_bricks
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
