use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use eqsolver::multivariable::MultiVarNewton;
use eqsolver::nalgebra::{SVector, SMatrix, ArrayStorage};
use nalgebra::matrix;

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

fn part_b(file_path: &str) -> i64 {
    let pv: Vec<[i64; 6]> = parse_input(file_path);
    let pv: Vec<[f64; 6]> = pv.into_iter().map(|x| [x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64, x[4] as f64, x[5] as f64]).collect();
    // v will be (x, y, z, vx, vy, vz, t1, t2, t3)
    let scale: f64 = 1.;
    let mut pv0 = pv[0]; pv0 = [pv0[0] / scale, pv0[1] / scale, pv0[2] / scale, pv0[3], pv0[4], pv0[5]];
    let mut pv1 = pv[1]; pv1 = [pv1[0] / scale, pv1[1] / scale, pv1[2] / scale, pv1[3], pv1[4], pv1[5]];
    let mut pv2 = pv[2]; pv2 = [pv2[0] / scale, pv2[1] / scale, pv2[2] / scale, pv2[3], pv2[4], pv2[5]];
    let f = |v: SVector<f64, 6>| SVector::from([
        (pv0[1] - v[1]) * (pv0[3] - v[3]) + (v[0] - pv0[0]) * (pv0[4] - v[4]),
        (pv0[2] - v[2]) * (pv0[3] - v[3]) + (v[0] - pv0[0]) * (pv0[5] - v[5]),
        (pv1[1] - v[1]) * (pv1[3] - v[3]) + (v[0] - pv1[0]) * (pv1[4] - v[4]),
        (pv1[2] - v[2]) * (pv1[3] - v[3]) + (v[0] - pv1[0]) * (pv1[5] - v[5]),
        (pv2[1] - v[1]) * (pv2[3] - v[3]) + (v[0] - pv2[0]) * (pv2[4] - v[4]),
        (pv2[2] - v[2]) * (pv2[3] - v[3]) + (v[0] - pv2[0]) * (pv2[5] - v[5]),
    ]);

    // Jacobian of F
    let jacobian = |v: SVector<f64, 6>| matrix![
        pv0[4] - v[4], v[3] - pv0[3], 0., v[1] - pv0[1], -v[0] + pv0[0], 0.;
        pv0[5] - v[5], 0., v[3] - pv0[3], v[2] - pv0[2], 0., -v[0] + pv0[0];
        pv1[4] - v[4], v[3] - pv1[3], 0., v[1] - pv1[1], -v[0] + pv1[0], 0.;
        pv1[5] - v[5], 0., v[3] - pv1[3], v[2] - pv1[2], 0., -v[0] + pv1[0];
        pv2[4] - v[4], v[3] - pv2[3], 0., v[1] - pv2[1], -v[0] + pv2[0], 0.;
        pv2[5] - v[5], 0., v[3] - pv2[3], v[2] - pv2[2], 0., -v[0] + pv2[0];
    ];

    //let mut x = SVector::from([pv[0][3], pv[0][4], pv[0][5], 0., 0., 0.]);
    let initial = SVector::from([300000000000000. / scale, 300000000000000. / scale, 300000000000000. / scale, -pv0[3] * 2., 0., 0.]);
    let mut x = initial.clone();
    //let mut x = SVector::from([10. / scale, 10. / scale, 10. / scale, -pv0[3] * 10., 0., 0.]);
    let mut error: f64 = 10000.;
    let mut learning_rate: f64 = 1.0;
    println!("Initial error vec is {:?}", f(x));
    while error > 0.1 {
        let error_vec = f(x);
        let grad = jacobian(x);
        //println!("Grad is {:?}", grad);
        x -= learning_rate * grad.try_inverse().unwrap() * error_vec;
        error = error_vec.norm();
        learning_rate = learning_rate * 0.99;
    }
    println!("Solution is {:?} with error {:?}", x, error);
    println!("Full error vec is {:?}", f(x));
    let solution = MultiVarNewton::new(f, jacobian).with_tol(1e-3).with_itermax(1000).solve(initial);
    println!("Found solution {:?}", solution);
    x[0].round() as i64 + x[1].round() as i64 + x[2].round() as i64
}

fn part_b_nine(file_path: &str) -> i32 {
    let pv: Vec<[i64; 6]> = parse_input(file_path);
    let pv: Vec<[f64; 6]> = pv.into_iter().map(|x| [x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64, x[4] as f64, x[5] as f64]).collect();
    // v will be (x, y, z, vx, vy, vz, t1, t2, t3)
    let F = |v: SVector<f64, 9>| SVector::from([
        pv[0][0] - v[0] + v[6] * (pv[0][3] - v[3]),
        pv[0][1] - v[1] + v[6] * (pv[0][4] - v[4]),
        pv[0][2] - v[2] + v[6] * (pv[0][5] - v[5]),
        pv[1][0] - v[0] + v[7] * (pv[1][3] - v[3]),
        pv[1][1] - v[1] + v[7] * (pv[1][4] - v[4]),
        pv[1][2] - v[2] + v[7] * (pv[1][5] - v[5]),
        pv[2][0] - v[0] + v[8] * (pv[2][3] - v[3]),
        pv[2][1] - v[1] + v[8] * (pv[2][4] - v[4]),
        pv[2][2] - v[2] + v[8] * (pv[2][5] - v[5]),
    ]);

    // Jacobian of F
    let J = |v: SVector<f64, 9>| matrix![
        -1.0, 0.0, 0.0, -v[6], 0.0, 0.0, pv[0][3] - v[3], 0.0, 0.0;
        0.0, -1.0, 0.0, 0.0, -v[6], 0.0, pv[0][4] - v[4], 0.0, 0.0;
        0.0, 0.0, -1.0, 0.0, 0.0, -v[6], pv[0][5] - v[5], 0.0, 0.0;
        -1.0, 0.0, 0.0, -v[7], 0.0, 0.0, 0.0, pv[1][3] - v[3], 0.0;
        0.0, -1.0, 0.0, 0.0, -v[7], 0.0, 0.0, pv[1][4] - v[4], 0.0;
        0.0, 0.0, -1.0, 0.0, 0.0, -v[7], 0.0, pv[1][5] - v[5], 0.0;
        -1.0, 0.0, 0.0, -v[8], 0.0, 0.0, 0.0, 0.0, pv[2][3] - v[3];
        0.0, -1.0, 0.0, 0.0, -v[8], 0.0, 0.0, 0.0, pv[2][4] - v[4];
        0.0, 0.0, -1.0, 0.0, 0.0, -v[8], 0.0, 0.0, pv[2][5] - v[5];
    ];
    
    let mut x = SVector::from([pv[0][3], pv[0][4], pv[0][5], 0., 0., 0., 1., 1., 1.]); 
    let mut error = 10000.0;
    for _ in 0..10 {
        let error_vec = F(x);
        let grad = J(x);
        println!("Grad is {:?}", grad);
        x -= grad.try_inverse().unwrap() * error_vec;
        error = error_vec.norm();
        println!("New x is {:?} with error {:?}", x, error);
    }
    //let solution = MultiVarNewton::new(F, J).solve(SVector::from([1., 1., 1., 1., 1., 1., 1., 1., 1.]));
    //println!("Found solution {:?}", solution);
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
