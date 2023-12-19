use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

fn main() {
    let file_path = "./data/puzzle7/input.txt";
    //let file_path = "./data/puzzle7/example.txt";
    //let file_path = "./data/puzzle7/hard.txt";
    let ans = puzzle7a(&file_path);
    println!("First answer is {ans}!\n");

    let ans = puzzle7b(&file_path);
    println!("Second answer is {ans}!");
}

fn puzzle7a(file_path: &str) -> i32 {
    let mut five_kind: HashMap<&str, i32> = HashMap::new();
    let mut four_kind: HashMap<&str, i32> = HashMap::new();
    let mut three_kind: HashMap<&str, i32> = HashMap::new();
    let mut pair: HashMap<&str, i32> = HashMap::new();
    let mut two_pair: HashMap<&str, i32> = HashMap::new();
    let mut full_house: HashMap<&str, i32> = HashMap::new();
    let mut high_card: HashMap<&str, i32> = HashMap::new();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut char_sorter: HashMap<char, i32> = HashMap::new();

    for line in contents.split('\n') {
        if line.len() > 5 {
                let (cards, bid) = line.split_once(' ').unwrap();
                for card in cards.chars() {
                    if char_sorter.contains_key(&card) {
                        *char_sorter.get_mut(&card).unwrap() += 1;
                    } else {
                        char_sorter.insert(card, 1);
                    }
                }
                
                // Sort sizes, determine hand type
                let mut biggest: i32 = 0;
                let mut second: i32 = 0;
                for (_unused, cnt) in char_sorter.iter() {
                    if cnt > &biggest {
                        second = biggest;
                        biggest = *cnt;
                    } else if cnt > &second {
                        second = *cnt
                    }
                }

                match biggest {
                    5 => {five_kind.insert(cards, bid.parse::<i32>().unwrap());}
                    4 => {four_kind.insert(cards, bid.parse::<i32>().unwrap());}
                    3 => {
                        if second == 2 {
                            full_house.insert(cards, bid.parse::<i32>().unwrap());
                        } else {
                            three_kind.insert(cards, bid.parse::<i32>().unwrap());
                        }
                    }
                    2 => {
                        if second == 2 {
                            two_pair.insert(cards, bid.parse::<i32>().unwrap());
                        } else {
                            pair.insert(cards, bid.parse::<i32>().unwrap());
                        }
                    }
                    1 => {high_card.insert(cards, bid.parse::<i32>().unwrap());}
                    _ => {panic!("This shouldn't happen!");}
                };

                char_sorter.clear();

        }
    }

    let mut total_rank: i32 = 0;
    let mut hand_idx: i32 = 1;
    for hands in [high_card, pair, two_pair, three_kind, full_house, four_kind, five_kind] {
        let mut fixed: HashMap<String, i32> = HashMap::new();
        for k in hands.keys() {
            let new_key = sorter_map(k, false);
            fixed.insert(new_key, *hands.get(k).unwrap());
        }
        //println!("Fixed is {:?}", fixed);
        for (_k, v) in fixed.iter().sorted() {
            //println!("Inserting score of {} for hand {k}", v * hand_idx);
            total_rank += v * hand_idx;
            hand_idx += 1;
        }
    }

    total_rank
}

fn puzzle7b(file_path: &str) -> i32 {
    let mut five_kind: HashMap<&str, i32> = HashMap::new();
    let mut four_kind: HashMap<&str, i32> = HashMap::new();
    let mut three_kind: HashMap<&str, i32> = HashMap::new();
    let mut pair: HashMap<&str, i32> = HashMap::new();
    let mut two_pair: HashMap<&str, i32> = HashMap::new();
    let mut full_house: HashMap<&str, i32> = HashMap::new();
    let mut high_card: HashMap<&str, i32> = HashMap::new();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut char_sorter: HashMap<char, i32> = HashMap::new();

    for line in contents.split('\n') {
        if line.len() > 5 {
            let mut num_js = 0;
            let (cards, bid) = line.split_once(' ').unwrap();
            for card in cards.chars() {
                if card == 'J' {
                    num_js += 1;
                    continue
                }

                if char_sorter.contains_key(&card) {
                    *char_sorter.get_mut(&card).unwrap() += 1;
                } else {
                    char_sorter.insert(card, 1);
                }
            }

            // Sort sizes, determine hand type
            let mut biggest: i32 = 0;
            let mut second: i32 = 0;
            for (_unused, cnt) in char_sorter.iter() {
                if cnt > &biggest {
                    second = biggest;
                    biggest = *cnt;
                } else if cnt > &second {
                    second = *cnt
                }
            }
            biggest += num_js;

            match biggest {
                5 => {five_kind.insert(cards, bid.parse::<i32>().unwrap());}
                4 => {four_kind.insert(cards, bid.parse::<i32>().unwrap());}
                3 => {
                    if second == 2 {
                        full_house.insert(cards, bid.parse::<i32>().unwrap());
                    } else {
                        three_kind.insert(cards, bid.parse::<i32>().unwrap());
                    }
                }
                2 => {
                    if second == 2 {
                        two_pair.insert(cards, bid.parse::<i32>().unwrap());
                    } else {
                        pair.insert(cards, bid.parse::<i32>().unwrap());
                    }
                }
                1 => {high_card.insert(cards, bid.parse::<i32>().unwrap());}
                _ => {panic!("This shouldn't happen!");}
            };

            char_sorter.clear();
        }
    }

    let mut total_rank: i32 = 0;
    let mut hand_idx: i32 = 1;
    for hands in [high_card, pair, two_pair, three_kind, full_house, four_kind, five_kind] {
        let mut fixed: HashMap<String, i32> = HashMap::new();
        for k in hands.keys() {
            let new_key = sorter_map(k, true);
            fixed.insert(new_key, *hands.get(k).unwrap());
        }
        println!("Fixed is {:?}", fixed);
        for (k, v) in fixed.iter().sorted() {
            println!("Inserting score of {} for hand {k}", v * hand_idx);
            total_rank += v * hand_idx;
            hand_idx += 1;
        }
    }

    total_rank
}

fn sorter_map(s: &str, map_j: bool) -> String {
    let card_string = s.to_string();
    let mut new_string = String::new();
    for c in card_string.chars() {
        if c == 'A' {
            new_string.push_str(&'Z'.to_string());
        } else if c == 'K' {
            new_string.push_str(&'Y'.to_string());
        } else if c == 'T' {
            new_string.push_str(&'A'.to_string());
        } else if map_j & (c == 'J') {
            new_string.push_str(&'1'.to_string());
        } else {
            new_string.push_str(&c.to_string());
        }
    }
    new_string
}
