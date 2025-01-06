use std::collections::VecDeque;
use std::fs;

use regex;

fn main() {
    let file_path = "./data/puzzle5/input.txt";
    //let file_path = "./data/puzzle5/example.txt";
    //let file_path = "./data/puzzle5/hard.txt";
    let mut ans = puzzle5a(&file_path);
    println!("First answer is {ans}!");

    ans = puzzle5b(&file_path);
    println!("Second answer is {ans}!");
}

fn puzzle5a(file_path: &str) -> i64 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //Parse seeds
    let (seed_str, remaining) = match contents.split_once("seed-to-soil map:") {
        Some((s, m)) => {(s, m)}
        None => {panic!("Failed to find 'seed-to-soil map:' in line!");}
    };
    let mut seeds = parse_seeds(&seed_str);
    let mut mapped = vec![-1; seeds.len()];

    //Read maps
    let mut source: i64 = 0;
    let mut dest: i64 = 0;
    let mut range: i64 = 0;
    let reg = regex::Regex::new(r"soil-to-fertilizer map:\n|fertilizer-to-water map:\n|water-to-light map:\n|light-to-temperature map:\n|temperature-to-humidity map:\n|humidity-to-location map:\n").unwrap();
    for map_str in reg.split(remaining) {
        for map_line in map_str.split('\n') {
            if map_line.len() > 3 {
                // Parse the current line to determine what maps where
                for (i, val) in map_line.split(' ').enumerate() {
                    match i {
                        0 => {dest = val.parse::<i64>().unwrap();}
                        1 => {source = val.parse::<i64>().unwrap();}
                        2 => {range = val.parse::<i64>().unwrap();}
                        _ => {panic!("Too many values!");}
                    };
                }

                // Map applicable seeds
                for (i, seed) in seeds.iter().enumerate() {
                    if (seed >= &source) & (seed < &(source + range)) {
                        mapped[i] = dest + (seed - source);
                    }
                }
            }
        }

        // Handle unmapped
        for i in 0..mapped.len() {
            if mapped[i] == -1 {
                mapped[i] = seeds[i];
            }
        }

        // Reset for next loop
        seeds = mapped;
        mapped = vec![-1; seeds.len()];
    }
    *seeds.iter().min().unwrap()
}

fn puzzle5b(file_path: &str) -> i64 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //Parse seeds
    let (seed_str, remaining) = match contents.split_once("seed-to-soil map:") {
        Some((s, m)) => {(s, m)}
        None => {panic!("Failed to find 'seed-to-soil map:' in line!");}
    };
    let seeds = parse_seeds(&seed_str);
    let mut inqueue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut outqueue: VecDeque<(i64, i64)> = VecDeque::new();

    // Fill the inqueue for the first time
    for i in 0..seeds.len() / 2 {
        inqueue.push_back((seeds[i * 2], seeds[i * 2 + 1]));
    }

    //Read maps
    let mut source: i64 = 0;
    let mut dest: i64 = 0;
    let mut range: i64 = 0;
    let reg = regex::Regex::new(r"soil-to-fertilizer map:\n|fertilizer-to-water map:\n|water-to-light map:\n|light-to-temperature map:\n|temperature-to-humidity map:\n|humidity-to-location map:\n").unwrap();
    for map_str in reg.split(remaining) {
        'popping: while inqueue.len() > 0 {
            let mut new_range: (i64, i64) = match inqueue.pop_front() {
                Some((a, b)) => {(a, b)},
                None => {panic!("Couldn't pop, this shouldn't happen!");}
            };
            for map_line in map_str.split('\n') {
                if map_line.len() > 3 {
                    // Parse the current line to determine what maps where
                    for (i, val) in map_line.split(' ').enumerate() {
                        match i {
                            0 => {dest = val.parse::<i64>().unwrap();}
                            1 => {source = val.parse::<i64>().unwrap();}
                            2 => {range = val.parse::<i64>().unwrap();}
                            _ => {panic!("Too many values!");}
                        };
                    }

                    // Map applicable seeds
                    if (new_range.0 >= source) & (new_range.0 < (source + range)) {
                        // left of new_range is in mappable range
                        if new_range.0 + new_range.1 < (source + range) {
                            outqueue.push_front((dest + new_range.0 - source, new_range.1));
                            continue 'popping;
                        } else {
                            let overlapping_range = source + range - new_range.0;
                            //println!("Overlapping range, using {overlapping_range} with source {source} range {range} new_range {}", new_range.0);
                            outqueue.push_front((dest + new_range.0 - source, overlapping_range - 1));
                            inqueue.push_back((source + range, new_range.1 - overlapping_range));
                            continue 'popping;
                        }
                    }
                }
            }

            // Handle unmapped
            outqueue.push_front(new_range);

            //println!("Seeds {} {} {} {}", seeds[0], seeds[1], seeds[2], seeds[3]);
            //println!("Mapped {} {} {} {}", mapped[0], mapped[1], mapped[2], mapped[3]);
        }

        // Reset for next loop
        while outqueue.len() > 0 {
            let moved = match outqueue.pop_front() {
                Some((a, b)) => {(a, b)},
                None => {panic!("Should be able to pop here!");}
            };
            inqueue.push_front(moved);
        }
        println!("New queue is ");
        for (s, e) in inqueue.iter() {
            println!("{s} {e}");
        }
    }

    // Compute minimum output
    //let mut min_seed: i64 = 2_i64.pow(63);
    //for (min_val, _unused) in inqueue.iter() {
    //    min_seed = min(min_seed, *min_val);
    //}
    let mins: Vec<i64> = inqueue.iter().map(|&x| x.0).collect();
    *mins.iter().min().unwrap()
}

fn parse_seeds(seed_str: &str) -> Vec<i64> {
    let mut seeds: Vec<i64> = vec![];
    for seed in seed_str.split([' ', '\n']) {
        if (seed.len() > 0) & (seed != "seeds:") {
            seeds.push(seed.parse::<i64>().unwrap())
        }
    }
    seeds
}
