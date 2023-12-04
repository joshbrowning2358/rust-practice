use std::fs;
use std::collections::HashMap;
//use dict::{Dict, DictIface};

fn main(){
    let file_path = "./data/puzzle3/input.txt";
    //let file_path = "./data/puzzle3/example.txt";
    //let file_path = "./data/puzzle3/hard.txt";
    let mut ans = puzzle3a(&file_path);
    println!("First answer is {ans}!");
    
    ans = puzzle3b(&file_path);
    println!("Second answer is {ans}!");
}

fn puzzle3a(file_path: &str) -> i32 {
    let mut ans = 0;
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("\n {contents}");

    let char_array: Vec<char> = contents.chars().collect::<Vec<_>>();
    let width: i32 = get_width(&char_array) as i32;
    let height: i32 = char_array.len() as i32 / width - 1;

    // Iterate over char_array, looking for ints
    let mut row: i32;
    let mut start_col: i32;
    let mut end_col: i32;

    let mut current_num = String::from("");
    let mut i: usize = 0;
    let mut c: char;
    let mut val_to_check: char;
    
    while i < char_array.len() {
        c = char_array[i];
        if c.is_numeric() {
            current_num.push_str(&c.to_string());
        } else if current_num.len() > 0 {
            // Found a string representing a number, need to check around it
            let mut has_neighbor: bool = false;
            row = i as i32 / width as i32;
            start_col = (i as i32 - current_num.len() as i32) % width;
            end_col = i as i32 % width;
            for row_iter in (row as i32 - 1)..(row as i32 + 2) {
                for col_iter in (start_col as i32 - 1)..(end_col as i32 + 1) {
                    if 0 <= row_iter && row_iter <= height && 0 <= col_iter && col_iter <= width {
                        // Valid access, get char at array
                        val_to_check = char_array[(row_iter * width + col_iter) as usize];
                        if !val_to_check.is_numeric() && val_to_check != '.' && val_to_check != '\n' {
                            has_neighbor = true;
                        }
                    }
                }
            }

            if has_neighbor {
                ans += current_num.parse::<i32>().unwrap();
            } 
            current_num = String::from("")
        }
        i += 1;
    }

    ans
}

fn puzzle3b(file_path: &str) -> i32{
    let mut ans = 0;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut gear_nums: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut key: i32;

    let char_array: Vec<char> = contents.chars().collect::<Vec<_>>();
    let width: i32 = get_width(&char_array) as i32;
    let height: i32 = char_array.len() as i32 / width - 1;

    // Iterate over char_array, looking for ints
    let mut row: i32;
    let mut start_col: i32;
    let mut end_col: i32;

    let mut current_num = String::from("");
    let mut i: usize = 0;
    let mut c: char;
    let mut val_to_check: char;

    while i < char_array.len() {
        c = char_array[i];
        if c.is_numeric() {
            current_num.push_str(&c.to_string());
        } else if current_num.len() > 0 {
            // Found a string representing a number, need to check around it
            row = i as i32 / width as i32;
            start_col = (i as i32 - current_num.len() as i32) % width;
            end_col = i as i32 % width;
            for row_iter in (row as i32 - 1)..(row as i32 + 2) {
                for col_iter in (start_col as i32 - 1)..(end_col as i32 + 1) {
                    if 0 <= row_iter && row_iter <= height && 0 <= col_iter && col_iter <= width {
                        // Valid access, get char at array
                        val_to_check = char_array[(row_iter * width + col_iter) as usize];
                        if val_to_check == '*' {
                            key = row_iter * width + col_iter;
                            if gear_nums.contains_key(&key) {
                                gear_nums.get_mut(&key).unwrap().push(current_num.parse::<i32>().unwrap());
                            } else {
                                gear_nums.insert(key, vec![current_num.parse::<i32>().unwrap()]);
                            }
                        }
                    }
                }
            }
            current_num = String::from("")
        }
        i += 1;
    }

    // Look for all gears that have exactly two numbers adjacent, multiply and add to ans
    for (_key, value) in gear_nums.into_iter() {
        if value.len() == 2 {
            ans += value[0] * value[1]
        }
    }

    ans
}

fn get_width(char_array: &Vec<char>) -> usize {
    let width: usize;

    let mut i: usize = 0;
    loop {
        if char_array[i] == '\n' {
            width = i;
            break
        }
        i += 1;
    }
    width + 1
}
