use num::integer::lcm;

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

fn part_a(file_path: &str) -> i64 {
    let equations = parse_input(file_path);

    let mut b_push: i64;
    let mut result: i64 = 0;
    for ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) in equations {
        let mut success: bool = false;
        for a_push in 0..100 {
            let target = (prize_x - a_x * a_push, prize_y - a_y * a_push);
            if target.0 % b_x == 0 && target.1 % b_y == 0 && target.0 / b_x == target.1 / b_y {
                success = true;
                b_push = target.0 / b_x;
                result += a_push * 3 + b_push;
                break
            }
        }
    }
    return result
}

fn part_b(file_path: &str) -> i64 {
    // 103082379222467 is too low
    let equations = parse_input(file_path);
    let mut result: i64 = 0;
    for ((a_x, a_y), (b_x, b_y), (mut prize_x, mut prize_y)) in equations {
        prize_x += 10000000000000;
        prize_y += 10000000000000;

        let modulo = prize_x % b_x;
        let mut success = false;
        let mut i: i64 = 0;
        for _ in 0..b_x {
            if (modulo - a_x * i) % b_x == 0 {
                success = true;
                break
            }
            i += 1;
        }

        // Only continue if we can match x-pos, otherwise we're done
        if success {
            let mut a_push = i;
            let mut b_push = (prize_x - a_push * a_x) / b_x;
            //println!("\nInitial pushes {a_push} and {b_push}");
            let mult = lcm(a_x, b_x);

            // We've already found #A and #B to satistfy the x position, and we can now adjust those numbers using the
            // GCD to find additional solutions, i.e. #A + mult/b_x and #B - mult/a_x will also work.  This shift will
            // change the final y position by some constant amount for each shift, and so we have to figure out how 
            // much and determine if we can ever get the y position correct.
            let increase_a = mult / a_x;
            let decrease_b = mult / b_x;
            //println!("Increase a {increase_a} decrease b {decrease_b}");
            let delta_y = increase_a * a_y - decrease_b * b_y;
            //println!("ax {a_x} a_y {a_y} bx {b_x} by {b_y} mult {mult}");
            let y_error = prize_y - a_push * a_y - b_push * b_y;
            //println!("Found y_error {y_error} and delta_y {delta_y}");
            if y_error == 0 && delta_y == 0 {
                panic!("We haven't handled this case yet, as there are quite a few solutions!!!");
            } else if y_error == 0 {
                println!("Found exact solution right away!");
            } else {
                if y_error % delta_y == 0 {
                    a_push += y_error / delta_y * mult / a_x;
                    b_push -= y_error / delta_y * mult / b_x;
                    println!("Final answer is {a_push} and {b_push}");
                } else {
                    println!("No solution!");
                    continue
                }
            }
            result += a_push * 3 + b_push;
        }
    }
    return result
}

fn parse_input(file_path: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let mut result = Vec::new();

    let mut A: (i64, i64) = (0, 0);
    let mut B: (i64, i64) = (0, 0);
    let mut prize: (i64, i64) = (0, 0);

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    result.push((A, B, prize));
                } else {
                    let (id, values) = row.split_once(": ").unwrap();
                    if id == "Button A" {
                        let (left, right) = values.split_once(", ").unwrap();
                        A = (left[2..].parse::<i64>().unwrap(), right[2..].parse::<i64>().unwrap());
                    } else if id == "Button B" {
                        let (left, right) = values.split_once(", ").unwrap();
                        B = (left[2..].parse::<i64>().unwrap(), right[2..].parse::<i64>().unwrap());
                    } else if id == "Prize" {
                        let (left, right) = values.split_once(", ").unwrap();
                        prize = (left[2..].parse::<i64>().unwrap(), right[2..].parse::<i64>().unwrap());
                    } else {
                        panic!("Invalid id {id}!");
                    }
                }
            }
        }
    }

    return result;
}
