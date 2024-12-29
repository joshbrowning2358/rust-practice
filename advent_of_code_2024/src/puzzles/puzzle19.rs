use std::collections::HashMap;
use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let (towels_string, towel_sets) = parse_input(file_path);
    let towels = towels_string.iter().map(|x| x.as_str()).collect();

    let mut result: i32 = 0;
    for towel_set in towel_sets {
        result += check_contains(&towel_set, &towels) as i32;
    }
    return result
}

pub fn part_b(file_path: &str) -> i64 {
    let (towels_string, towel_sets) = parse_input(file_path);
    let towels = towels_string.iter().map(|x| x.as_str()).collect();
    let mut cache: HashMap<String, i64> = HashMap::new();

    let mut result: i64 = 0;
    for towel_set in towel_sets {
        result += count_contains(towel_set, &towels, &mut cache);
    }
    return result
}

fn parse_input(file_path: &str) -> (Vec<String>, Vec<String>) {
    let mut towel_sets = Vec::new();
    let mut towels = Vec::new();

    let mut i: usize = 0;
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if i == 0 {
                    towels = row.split(", ").map(|x| x.to_string()).collect();
                } else if i > 1 {
                    towel_sets.push(row);
                }
                i += 1;
            }
        }
    }

    return (towels, towel_sets)
}

fn check_contains(towel_set: &str, towels: &Vec<&str>) -> bool {
    if towel_set.len() == 0 {
        return true
    }

    for towel in towels {
        if towel_set.starts_with(towel) {
            let result = check_contains(&towel_set[towel.len()..], &towels);
            if result {return true}
        }
    }
    return false
}

fn count_contains(towel_set: String, towels: &Vec<&str>, cache: &mut HashMap<String, i64>) -> i64 {
    if towel_set.len() == 0 {
        return 1
    }
    if cache.contains_key(&towel_set) {
        return *cache.get(&towel_set).unwrap();
    }

    let mut valid_towels: i64 = 0;
    for towel in towels {
        if towel_set.starts_with(towel) {
            valid_towels += count_contains(towel_set[towel.len()..].to_string(), &towels, cache);
        }
    }
    cache.insert(towel_set, valid_towels);
    return valid_towels
}
