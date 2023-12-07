use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    //let mut cards = String::from(' ');
    //let mut bid = String::new();
    let ip: &str;

    let mut char_sorter: HashMap<char, i32> = HashMap::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let (cards, bid) = match ip.split_once(' ') {
                    Some((x, y)) => {(x, y)},
                    None => {panic!("Unable to parse hand/bid!")}
                };
                println!("Parsed cards {cards} and bid {bid}");
                for card in cards.clone().chars() {
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
                    5 => {five_kind.insert(cards.clone(), bid.parse::<i32>().unwrap());}
                    4 => {println!("Four")}
                    3 => {
                        if second == 2 {
                            println!("Full house");
                        } else {
                            println!("3");
                        }
                    }
                    2 => {
                        if second == 2 {
                            println!("Two pair");
                        } else {
                            println!("2");
                        }
                    }
                    1 => {println!("One")}
                    _ => {panic!("This shouldn't happen!");}
                };

                char_sorter.clear();
                biggest = 0;
                second = 0;

            }
        }
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
