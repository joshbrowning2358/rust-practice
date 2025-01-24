use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle13;

// enum RecursiveElement {
//     Cons(i32, Box<RecursiveElement>),
//     Nil,
// }

const RADIX: u32 = 10;

impl puzzle_solver::Solver for Puzzle13 {
    fn part_1(&self, file_path: &str) -> String {
        let signals = parse(file_path);
        
        let mut total = vec![];
        for (idx, (left, right)) in signals.iter().enumerate() {
            if compare(left.as_slice(), right.as_slice()) == 1 {
                total.push(idx + 1);
                //println!("!!!!Correct order!");
            }
        }
        let ans: usize = total.iter().sum();
        return ans.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        // Problem says to sort the entire list of packets, but really just need to find position of [[2]] and [[6]]
        // So, just compare [[2]] to all elements and determine how many it beats, then same for [[6]]
        let signals = parse(file_path);

        let vec_2 = vec!['[', '[', '2', ']', ']'];
        let vec_6 = vec!['[', '[', '6', ']', ']'];

        let mut pos_2 = 0;
        let mut pos_6 = 0;
        for (left, right) in signals.iter() {
            if compare(vec_2.as_slice(), left.as_slice()) == -1 {
                pos_2 += 1;
                pos_6 += 1;
            } else if compare(vec_6.as_slice(), left.as_slice()) == -1 {
                pos_6 += 1;
            }

            if compare(vec_2.as_slice(), right.as_slice()) == -1 {
                pos_2 += 1;
                pos_6 += 1;
            } else if compare(vec_6.as_slice(), right.as_slice()) == -1 {
                pos_6 += 1;
            }

        }
        // Add 1 to pos_2 since we use 1-based indexing.  Add 2 to pos_6 since we use 1-based indexing and it follows [[2]]
        let ans = (pos_2 + 1) * (pos_6 + 2);
        return ans.to_string()
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("5682", "")
    }

    fn name(&self) -> &'static str {
        "Day 13: Distress Signal"
    }
}

fn parse(file_path: &str) -> Vec<(Vec<char>, Vec<char>)> {
    let mut left: Vec<char> = vec![];
    let mut right: Vec<char> = vec![];

    let mut result: Vec<(Vec<char>, Vec<char>)> = vec![];
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                match i % 3 {
                    0 => {left = row.chars().collect();},
                    1 => {right = row.chars().collect();},
                    2 => {result.push((left.clone(), right.clone()));}
                    _ => {panic!("Invalid case!");}
                };
            }
        }
    }
    return result
}

fn compare(left: &[char], right: &[char]) -> i8 {
    // Assumes left and right are both lists of elements to compare
    // Returns 1 if sorted, -1 if not sorted, and 0 if a tie
    let mut left_idx = 1;
    let mut right_idx = 1;
    loop {
        // println!("Left: {left:?} Right: {right:?}, left_idx: {left_idx}, right_idx: {right_idx}");
        let mut left_has_ended = left_idx >= left.len();
        if !left_has_ended {left_has_ended = left[left_idx] == ']';}
        let mut right_has_ended = right_idx >= right.len();
        if !right_has_ended {right_has_ended = right[right_idx] == ']';}
        if left_has_ended & right_has_ended {
            return 0
        } else if left_has_ended {
            // Ran out of items on the left first, in order
            return 1
        } else if right_has_ended {
            // Ran out of items on the right first, out of order
            return -1
        } else if (left[left_idx] != '[') & (right[right_idx] != '[') {
            // Both integers, compare directly
            let mut left_val = left[left_idx].to_digit(RADIX).unwrap();
            if left[left_idx + 1] == '0' {
                left_val = 10;
                left_idx += 1;
            }
            let mut right_val = right[right_idx].to_digit(RADIX).unwrap();
            if right[right_idx + 1] == '0' {
                right_val = 10;
                right_idx += 1;
            }
            left_idx += 2;  // Jump comma on to next digit
            right_idx += 2;
            // println!("Comparing {left_val} and {right_val} directly");
            if left_val < right_val {return 1}
            if left_val > right_val {return -1}
        } else if (left[left_idx] == '[') & (right[right_idx] == '[') {
            // Both lists, recurse
            let left_end = scan_for_end(&left[left_idx..]);
            let right_end = scan_for_end(&right[right_idx..]);
            // println!("Both lists, recursing with {:?} and {:?}", &left[left_idx..(left_idx + left_end + 1)], &right[right_idx..(right_idx + right_end + 1)]);
            let comp = compare(&left[left_idx..(left_idx + left_end + 1)], &right[right_idx..(right_idx + right_end + 1)]);
            match comp {
                1 => {return 1}
                -1 => {return -1}
                _ => {}
            };
            left_idx += left_end + 2;
            right_idx += right_end + 2;
        } else if (left[left_idx] == '[') & (right[right_idx] != '[') {
            // Left is a list, convert right into one and recurse
            let left_end = scan_for_end(&left[left_idx..]);
            let mut new_right = vec!['[', right[right_idx], ']'];
            if right[right_idx + 1] == '0' {
                new_right = vec!['[', '1', '0', ']'];
            }
            // println!("Left list, recursing with {:?} and {:?}", &left[left_idx..(left_idx + left_end + 1)], new_right);
            let comp = compare(&left[left_idx..(left_idx + left_end + 1)], new_right.as_slice());
            match comp {
                1 => {return 1}
                -1 => {return -1}
                _ => {}
            };
            left_idx += left_end + 2;
            right_idx += 2
        } else if (left[left_idx] != '[') & (right[right_idx] == '[') {
            // Right is a list, convert left into one and recurse
            let right_end = scan_for_end(&right[right_idx..]);
            let mut new_left = vec!['[', left[left_idx], ']'];
            if left[left_idx + 1] == '0' {
                new_left = vec!['[', '1', '0', ']'];
            }
            // println!("Right list, recursing with {:?} and {:?}", new_left, &right[right_idx..(right_idx + right_end + 1)]);
            let comp = compare(new_left.as_slice(), &right[right_idx..(right_idx + right_end + 1)]);
            match comp { 
                1 => {return 1}
                -1 => {return -1}
                _ => {}
            };
            left_idx += 2;
            right_idx += right_end + 2;
        }
    }
}

fn scan_for_end(chars: &[char]) -> usize {
    let mut end_idx = 0;
    let mut n_open = 1;
    while n_open > 0 {
        end_idx += 1;
        if chars[end_idx] == '[' {
            n_open += 1;
        } else if chars[end_idx] == ']' {
            n_open -= 1;
        }
    }
    return end_idx
}
