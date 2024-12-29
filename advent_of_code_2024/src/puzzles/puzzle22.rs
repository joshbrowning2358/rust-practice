use std::collections::{HashSet,VecDeque};

use crate::file_reader;

pub fn part_a(file_path: &str) -> i64 {
    let secret_nums = parse_input(file_path);

    let mut result: i64 = 0;
    for secret_num in secret_nums {
        let mut secret = secret_num;
        for _ in 0..2000 {
            secret = evolution(secret);
        }
        result += secret;
    }
    return result
}

pub fn part_b(file_path: &str) -> i32 {
    let secret_nums = parse_input(file_path);

    let mut prices: Vec<Vec<i8>> = vec![];
    let mut deltas: Vec<Vec<Vec<i8>>> = vec![];
    for (secret_idx, secret_num) in secret_nums.iter().enumerate() {
        let mut secret = *secret_num;
        prices.push(vec![]);
        deltas.push(vec![]);
        let mut curr_delta: VecDeque<i8> = VecDeque::from([-12, -12, -12, 12]);
        let mut last_val = (secret.clone() % 10) as i8;
        for _ in 0..2000 {
            secret = evolution(secret);
            let price = (secret.clone() % 10) as i8;
            prices[secret_idx].push(price.clone());

            curr_delta.pop_front().unwrap();
            curr_delta.push_back(price - last_val);
            deltas[secret_idx].push(Vec::from(curr_delta.clone()));
            last_val = price;
        }
    }
    
    // Search for "candidates", i.e. 4 deltas that may lead to the highest price
    let cand = get_candidates(&prices, &deltas, 7);
    let l = cand.len();
    println!("Found {l} candidates!");

    return eval_candidates(&prices, &deltas, &cand)
}

fn parse_input(file_path: &str) -> Vec<i64> {
    let mut starting_nums = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                starting_nums.push(row.parse::<i64>().unwrap());
            }
        }
    }

    return starting_nums
}

fn evolution(secret_num: i64) -> i64 {
    let mut result = ((secret_num * 64) ^ secret_num) % 16777216;
    result = ((result / 32) ^ result) % 16777216;
    result = ((result * 2048) ^ result) % 16777216;
    return result
}

fn get_candidates(prices: &Vec<Vec<i8>>, deltas: &Vec<Vec<Vec<i8>>>, cutoff: i8) -> HashSet<Vec<i8>> {
    let mut cand: HashSet<Vec<i8>> = HashSet::new();
    for (p_idx, p) in prices.iter().enumerate() {
        for (i, val) in p[4..].iter().enumerate() {
            if val >= &cutoff {
                cand.insert(deltas[p_idx][i + 4].clone());
            }
        }
    }
    return cand;
}

fn eval_candidates(prices: &Vec<Vec<i8>>, deltas: &Vec<Vec<Vec<i8>>>, cands: &HashSet<Vec<i8>>) -> i32 {
    let mut best: i32 = 0;
    let total = cands.len();
    for (c_idx, cand) in cands.iter().enumerate() {
        let mut cand_score: i32 = 0;
        'monkey: for p_idx in 0..prices.len() {
            for i_idx in 4..prices[p_idx].len() {
                if deltas[p_idx][i_idx] == *cand {
                    cand_score += prices[p_idx][i_idx] as i32;
                    continue 'monkey
                }
            }
        }
        if cand_score > best {
            best = cand_score;
        }
        if c_idx % 100 == 0 {
            println!("Ran {c_idx}/{total} iterations, best_score is {best}");
        }
    }
    return best
}
