use std::fs;

fn main() {
    let file_path = "./data/puzzle6/input.txt";
    //let file_path = "./data/puzzle6/example.txt";
    //let file_path = "./data/puzzle6/hard.txt";
    let ans = puzzle6a(&file_path);
    println!("First answer is {ans}!\n");

    let ans = puzzle6b(&file_path);
    println!("Second answer is {ans}!");
}

fn puzzle6a(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("{}", contents);
    
    let (time_str, distance_str) = match contents.split_once('\n') {
        Some((s, m)) => {(s, m)}
        None => {panic!("Failed to find newline in line!");}
    };
    let times = parse_line(time_str);
    let distances = parse_line(distance_str);

    let mut ans: i32 = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut start: i32 = 1;
        while start * (time - start) <= *distance {
            start += 1;
        }
        ans *= time + 1 - 2 * start;
    }

    ans
}

fn puzzle6b(file_path: &str) -> i64 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //let time: i32 = 71530;
    //let distance: i32 = 940200;
    let time: i64 = 40828492;
    let distance: i64 = 233101111101487;


    let mut start: i64 = 1;
    while start * (time - start) <= distance {
        start += 1;
    }
    time + 1 - 2 * start
}

fn parse_line(s: &str) -> Vec::<i32> {
    let (_unused, s) = match s.split_once(':') {
        Some((x, y)) => {(x, y)}
        None => {panic!("Unable to split!")}
    };

    let strs = s.split_whitespace();
    let mut result = Vec::<i32>::new();
    for s in strs {
        if s.len() > 0 {
            result.push(s.parse::<i32>().unwrap());
        }
    }
    result
}
