use std::collections::{HashSet,VecDeque};

use crate::file_reader;

fn counts(file_path: &str) -> usize {
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
    let cand = get_candidates(&prices, &deltas, 0);
    let total_candidate_num = cand.len();
    return total_candidate_num
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
