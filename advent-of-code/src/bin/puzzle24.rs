use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle24/example.txt"; let region = [10.0, 20.0];
    let file_path = "data/puzzle24/input.txt"; let region = [200000000000000.0, 400000000000000.0];

    let ans = part_a(file_path, region);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str, region: [f32; 2]) -> i32 {
    let mut ans = 0;
    let pts_velocities = parse_input(file_path);
    for i in 0..(pts_velocities.len() - 1) {
        let mi = pts_velocities[i][4] as f32 / pts_velocities[i][3] as f32;
        let bi = pts_velocities[i][1] as f32 - pts_velocities[i][0] as f32 * mi;
        for j in (i + 1)..pts_velocities.len() {
            let mj = pts_velocities[j][4] as f32 / pts_velocities[j][3] as f32;
            let bj = pts_velocities[j][1] as f32 - pts_velocities[j][0] as f32 * mj;
            // We have x, y, z, vx, vy, vz
            // 2D slope = vy / vx
            // 2D intersection: y = mx + b -> b = y - mx
            // Solve mi x + bi = mj x + bj -> x = (bj - bi) / (mi - mj)
            if mi == mj {continue}
            let x = (bj - bi) / (mi - mj);
            let y = mi * x + bi;
            //println!("Checking points {:?} and {:?} collide at {x} {y}", pts_velocities[i], pts_velocities[j]);
            if (x >= region[0]) & (x <= region[1]) & (y >= region[0]) & (y <= region[1]) {
                // Only valid if moving forward in time
                //println!("Dividends are {:?} and {:?}", (x - pts_velocities[i][0] as f32) / pts_velocities[i][3] as f32, (x - pts_velocities[j][0] as f32) / pts_velocities[j][3] as f32);
                if ((x - pts_velocities[i][0] as f32) / pts_velocities[i][3] as f32 >= 0.0) & ((x - pts_velocities[j][0] as f32) / pts_velocities[j][3] as f32 >= 0.0) {
                    ans += 1;
                    //println!("Points {:?} and {:?} collide at {x} {y}", pts_velocities[i], pts_velocities[j]);
                } else {
                    //println!("Failed past collision for {:?} and {:?} as {x} {y}",  pts_velocities[i], pts_velocities[j]);
                }
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let _pv = parse_input(file_path);
    0
}

fn parse_input(file_path: &str) -> Vec<[i64; 6]> {
    let mut out: Vec<[i64; 6]> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (pt, vel) = ip.split_once('@').unwrap();
                let pts: Vec<i64> = pt.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
                let vels: Vec<i64> = vel.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
                out.push([pts[0], pts[1], pts[2], vels[0], vels[1], vels[2]]);
            }
        }
    }
    println!("Parsed out with len {}", out.len());
    out
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
