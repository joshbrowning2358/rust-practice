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

        //println!("Seeds {} {} {} {}", seeds[0], seeds[1], seeds[2], seeds[3]);
        //println!("Mapped {} {} {} {}", mapped[0], mapped[1], mapped[2], mapped[3]); 

        // Reset for next loop
        seeds = mapped;
        mapped = vec![-1; seeds.len()];
    }
    *seeds.iter().min().unwrap()
}

fn puzzle5b(file_path: &str) -> i64 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let ans = contents.len() as i64;
    ans
}

fn parse_seeds(seed_str: &str) -> Vec<i64> {
    let mut seeds: Vec<i64> = vec![];
    for seed in seed_str.split([' ', '\n']) {
        if (seed.len() > 0) & (seed != "seeds:") {
            println!("Parsing {seed}");
            seeds.push(seed.parse::<i64>().unwrap())
        }
    }
    seeds
}
