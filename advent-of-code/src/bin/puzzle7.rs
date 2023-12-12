use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "./data/puzzle7/input.txt";
    let file_path = "./data/puzzle7/example.txt";
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
    println!("\n {contents}");


    let mut char_sorter: HashMap<char, i32> = HashMap::new();

    for line in contents.split('\n') {
        if line.len() > 5 {
                let (cards, bid) = match line.split_once(' ') {
                    Some((x, y)) => {(x, y)},
                    None => {panic!("Unable to parse hand/bid!")}
                };
                println!("Parsed cards {cards} and bid {bid}");
                for card in cards.chars() {
                    if char_sorter.contains_key(&card) {
                        *char_sorter.get_mut(&card).unwrap() += 1;
                    } else {
                        char_sorter.insert(card, 1);
                    }
                }
                five_kind.insert(cards, 1);
                
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
    //for k in five_kind.keys().sort_by(|a, b| sorter_map(a).cmp(sorter_map(&b)) {
    //    println!("Key is {k}");
    //}

    for (k, v) in three_kind.iter().sorted_by(|a, b| sorter_map(a.to_string()).cmp(&sorter_map(b.to_string()))) {
        println!("{k} and {v}");
    }

    let ans = 0;
    ans
}

fn puzzle7b(file_path: &str) -> i64 {
    0
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sorter_map(s: String) {
    for i in 0..s.len() {
        let c = s.remove(i);
        if c == 'A' {
            s.insert(i, 'Z');
        } else if c == 'K' {
            s.insert(i, 'Y');
        } else if c == 'T' {
            s.insert(i, 'A');
        } else {
            s.insert(i, c);
        }
    }
}
