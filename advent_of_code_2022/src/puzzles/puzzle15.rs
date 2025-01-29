use std::collections::HashSet;
use std::cmp::max;

use crate::common::Point;
use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle15;

impl puzzle_solver::Solver for Puzzle15 {
    fn part_1(&self, file_path: &str) -> String {
        let pts = parse(file_path);
        let row = match file_path.contains("easy1") {
            true => 10,
            false => 2000000,
        };

        let mut invalid_ranges: Vec<(i32, i32)> = vec![];
        for (sensor, beacon) in &pts {
            let dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
            if (sensor.y - row).abs() > dist {
                // Sensor is too far from row, it won't impact it at all
                continue
            }
            let delta = dist - (sensor.y - row).abs();
            let invalid_range = (sensor.x - delta, sensor.x + delta);
            // Haven't extended with current range, so add to list
            invalid_ranges.push(invalid_range);
        }
        invalid_ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut invalid_cnt = 0;
        let mut right_ptr = -10000000;
        for (left, right) in invalid_ranges {
            if right_ptr > right {
                // Last interval fully contains current, just skip
                continue
            } else if right_ptr >= left {
                // overlap
                invalid_cnt += right - right_ptr;
            } else {
                invalid_cnt += right - left + 1;
            }
            right_ptr = right;
        }

        // Any beacons already detected on that row should be removed from the count though:
        let mut overcounted_xs = HashSet::new();
        for (_, beacon) in pts {
            if beacon.y == row {
                overcounted_xs.insert(beacon.x);
            }
        }
        invalid_cnt -= overcounted_xs.len() as i32;

        return invalid_cnt.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        let pts = parse(file_path);
        let max_dim = match file_path.contains("easy1") {
            true => 20,
            false => 4000000,
        };

        let mut i: i32 = 0;
        let mut j: i32 = 0;
        // println!("Points are {pts:?}");
        'outer: loop {
            for (sensor, beacon) in &pts {
                let dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
                let dist_to_curr = (sensor.x - i).abs() + (sensor.y - j).abs();
                if dist_to_curr <= dist {
                    i = max(i + 1, sensor.x + (dist - (sensor.y - j).abs()) + 1);
                    if i > max_dim {
                        i = 0;
                        j += 1;
                    }
                    // println!("Moved to {i},{j}");
                    continue 'outer
                }
                // println!("Now at point {i},{j}, dist was {dist}, dist_to_curr {dist_to_curr}");
            }
            // Wasn't close to any sensors, found target!
            break
        }
        let ans = 4000000 * (i as i64) + j as i64;

        return ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Day 15: Beacon Exclusion Zone"
    }
}

fn parse(file_path: &str) -> Vec<(Point<i32>, Point<i32>)>{
    let mut result: Vec<(Point<i32>, Point<i32>)> = vec![];
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let (sensor_str, beacon_str) = row.split_once(": closest beacon is at x=").unwrap();
                let (_, sensor_str) = sensor_str.split_once("ensor at x=").unwrap();
                let (x_str, y_str) = sensor_str.split_once(", y=").unwrap();
                let sensor = Point{x: x_str.parse::<i32>().unwrap(), y: y_str.parse::<i32>().unwrap()};

                let (x_str, y_str) = beacon_str.split_once(", y=").unwrap();
                let beacon = Point{x: x_str.parse::<i32>().unwrap(), y: y_str.parse::<i32>().unwrap()};
                result.push((sensor, beacon));
            }
        }
    }
    return result
}
