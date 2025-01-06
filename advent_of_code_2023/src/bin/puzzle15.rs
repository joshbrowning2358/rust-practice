use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //let file_path = "data/puzzle15/example.txt";
    let file_path = "data/puzzle15/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                for seq in ip.split(',') {
                    ans += hash(seq);
                }
            }
        }
    }
    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut boxes: HashMap<u8, Vec<String>> = HashMap::new();
    let mut label: &str;
    let mut lens_id: &str;
    let mut box_offloading: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                for seq in ip.split(',') {
                    //println!("Processing {seq}");
                    if seq.contains("=") {
                        (label, lens_id) = seq.split_once("=").unwrap();
                    } else {
                        (label, lens_id) = seq.split_once("-").unwrap();
                    }
                    let box_id = hash(label) as u8;

                    if !boxes.contains_key(&box_id) {
                        boxes.insert(box_id, Vec::new());
                    }

                    let current_box = boxes.get_mut(&box_id).unwrap();
                    if lens_id.len() > 0 { // Found an =
                        let mut modified: bool = false;
                        while current_box.len() > 0 {
                            let popped_lens = current_box.pop().unwrap();
                            let (popped_label, _) = popped_lens.split_once(" ").unwrap();
                            if popped_label == label {
                                // Add the new one back instead of the original
                                box_offloading.push(seq.replace("=", " "));
                                modified = true;
                                break;
                            }
                            box_offloading.push(popped_lens);
                        }

                        while box_offloading.len() > 0 {
                            current_box.push(box_offloading.pop().unwrap())
                        }

                        if !modified {
                            current_box.push(seq.replace("=", " "));
                        }
                    } else { // Found a -
                        while current_box.len() > 0 {
                            let popped_lens = current_box.pop().unwrap();
                            let (popped_label, _) = popped_lens.split_once(" ").unwrap();
                            if popped_label == label {
                                // Remove existing and stop shifting
                                break;
                            } 
                            box_offloading.push(popped_lens);
                        }

                        while box_offloading.len() > 0 {
                            current_box.push(box_offloading.pop().unwrap())
                        }
                    }
                    //println!("Boxes are {:?}", boxes);
                }
            }
        }
    }

    //println!("Final boxes are {:?}", boxes);

    for (k, v) in boxes.drain() {
        let mut lens_position: i32 = 1;
        for lens in v.iter() {
            let (_, to_add) = lens.split_once(" ").unwrap();
            ans += (k + 1) as i32 * lens_position as i32 * to_add.parse::<i32>().unwrap();
            lens_position += 1;
        }
    }
    ans
}

fn hash(input: &str) -> i32 {
    let mut current_value: i32 = 0;
    for c in input.chars() {
        current_value += (c as u32) as i32;
        current_value *= 17;
        current_value = current_value % 256;
    }
    return current_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
