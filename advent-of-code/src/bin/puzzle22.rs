use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "data/puzzle22/example.txt";
    //let file_path = "data/puzzle22/example2.txt";
    //let file_path = "data/puzzle22/edge_case2.txt";
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

fn part_a(file_path: &str) -> usize {
    let bricks = parse_input(file_path);
    let (supported, _) = drop_bricks(&bricks);

    let mut to_break: HashSet<usize> = HashSet::new();
    for i in 0..bricks.len() {to_break.insert(i);}
    for supporters in supported.values() {
        if supporters.len() == 1 {
            for to_remove in supporters.clone().drain() {
                to_break.remove(&to_remove);
            }
        }
    }
    to_break.len()
}

fn part_b(file_path: &str) -> usize {
    let bricks = parse_input(file_path);
    println!("Parsed bricks {:?}", bricks);
    let (supported, supporters) = drop_bricks(&bricks);
    println!("Supported is {:?}", supported);
    println!("Supporters is {:?}", supporters);

    let mut ans = 0;
    for i in 0..bricks.len() {
        // Initialize the queue
        let mut to_break_queue: Vec<usize> = Vec::new();
        let mut broken: HashSet<usize> = HashSet::new();  // Don't include initial

        for to_break in supporters.get(&i).unwrap().clone().drain() {
            to_break_queue.push(to_break);
        }
        while to_break_queue.len() > 0 {
            let new_brick = to_break_queue.pop().unwrap();
            if broken.contains(&new_brick) {
                continue
            }

            broken.insert(new_brick);
            if supporters.contains_key(&new_brick) {
                for to_break in supporters.get(&new_brick).unwrap().clone().drain() {
                    to_break_queue.push(to_break);
                }
            }
        }
        println!("Breaking brick {i} gives {}", broken.len());
        ans += broken.len();
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

fn drop_bricks(bricks: &Vec<Brick>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    // supported maps brick_id -> a vector of brick_ids that support it
    let mut supported: HashMap<usize, HashSet<usize>> = HashMap::new();
    // supporters maps brick_id -> a vector of brick_ids that it supports
    let mut supporters: HashMap<usize, HashSet<usize>> = HashMap::new();
    let nrows: i32 = 500;
    let ncols: i32 = 500;
    let mut heights = vec![0; (nrows * ncols) as usize];
    // brick_locs maps a 3D coordinate to the brick index at that position
    let mut brick_locs: HashMap<[i32; 3], usize> = HashMap::new();
    
    for (i, brick) in bricks.iter().enumerate() {
        let mut brick_support: HashSet<usize> = HashSet::new();
        let mut drop_height = 1;
        for pt in &brick.pts {
            drop_height = max(drop_height, heights[(pt[0] * ncols + pt[1]) as usize] + 1);
        }
        //println!("Dropping brick {i} to height {drop_height}");
        for pt in &brick.pts {
            // Determine supporting bricks
            if drop_height > 1 { // If not, brick lands on ground
                if heights[(pt[0] * ncols + pt[1]) as usize] == drop_height - 1 {
                    let supporting_brick = *brick_locs.get(&[pt[0], pt[1], drop_height - 1]).unwrap();
                    brick_support.insert(supporting_brick);

                    let mut old: HashSet<usize>;
                    if supporters.contains_key(&supporting_brick) {
                        old = supporters.get_mut(&supporting_brick).unwrap().clone();
                    } else {
                        old = HashSet::new();
                    }
                    old.insert(i);
                    supporters.insert(supporting_brick, old);
                }
            }

            // Brick will drop to drop height; update heights
            if heights[(pt[0] * ncols + pt[1]) as usize] >= drop_height {
                // If we've already bumped up drop height but have a vertical brick then add 1 to height
                heights[(pt[0] * ncols + pt[1]) as usize] += 1;
                // Update brick_locs
                brick_locs.insert([pt[0], pt[1], heights[(pt[0] * ncols + pt[1]) as usize]], i);
            } else {
                heights[(pt[0] * ncols + pt[1]) as usize] = drop_height;
                // Update brick_locs
                brick_locs.insert([pt[0], pt[1], drop_height], i);
            }

        }
        supported.insert(i, brick_support);
    }
    (supported, supporters)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
