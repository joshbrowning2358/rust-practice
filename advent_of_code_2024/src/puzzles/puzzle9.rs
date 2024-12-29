use crate::file_reader;

pub fn part_a(file_path: &str) -> i64 {
    // 6444783503027 is too low
    let files = parse_input(file_path);
    let mut result: i64 = 0;
    let mut is_gap: bool = false;
    // Keeps track of where we are in the disk filling
    let mut left_idx: usize = 0;
    // Keeps track of which file_id we're looking at
    let mut left_file_id: usize = 0;
    let mut left_file_idx: usize = 0;
    let mut right_file_id: usize = (files.len() / 2) as usize;
    let mut right_file_idx: usize = files.len() - 1;
    let mut right_remainder_len: usize = 0;

    while left_file_id <= right_file_id {
        if !is_gap {
            // Counting existing files
            let file_len = files[left_file_idx];
            for _i in 0..file_len {
                result += (left_idx * left_file_id) as i64;
                left_idx += 1;
            }
            left_file_idx += 1;
            left_file_id += 1;
        } else if left_file_id == right_file_id {
            // At end, use up remainder
            for _i in 0..right_remainder_len {
                result += (left_idx * right_file_id) as i64;
                left_idx += 1;
            }
            break
        } else {
            // Moving files from right to left side
            let mut gap_len = files[left_file_idx];
            left_file_idx += 1;
            while gap_len > 0 {
                let right_file_len: usize;
                if right_remainder_len == 0 {
                    right_file_len = files[right_file_idx];
                } else {
                    right_file_len = right_remainder_len;
                }
                if gap_len >= right_file_len {
                    gap_len = gap_len - right_file_len;
                    for _i in 0..right_file_len {
                        result += (left_idx * right_file_id) as i64;
                        left_idx += 1;
                    }
                    right_file_id -= 1;
                    right_file_idx -= 2; // Skip the gap on the right
                    right_remainder_len = 0;
                } else {
                    right_remainder_len = right_file_len - gap_len;
                    for _i in 0..gap_len {
                        result += (left_idx * right_file_id) as i64;
                        left_idx += 1;
                    }
                    gap_len = 0;
                }
            }
        }
        is_gap = !is_gap;
    }
    return result
}

pub fn part_b(file_path: &str) -> i64 {
    // 6478233125745 is too high

    // Build the initial files
    let file_sizes = parse_input(file_path);
    let mut disk: Vec<i64> = Vec::new();
    let mut file_id = 0;
    let mut is_gap: bool = false;
    for file_size in file_sizes.iter() {
        if is_gap {
            disk.append(&mut vec![-1; *file_size as usize]);
        } else {
            disk.append(&mut vec![file_id; *file_size as usize]);
        }
        if !is_gap {
            file_id += 1;
        }
        is_gap = !is_gap;
    }
    // println!("Disk is {disk:?}");

    // Build the compressed files
    // Keeps track of where we are in the disk filling
    let mut left_idx: usize = 0;
    let mut right_idx: usize = disk.len() - 1;
    'move_file: while right_idx > left_idx {
        // println!("Running with idxs {left_idx} {right_idx}");
        // Move the pointer back if we're on empty space
        while disk[right_idx] == -1 && right_idx > 0 {
            right_idx -= 1;
        }

        // Determine the size of the rightmost file we need to move
        let file_id = disk[right_idx];
        let mut file_size = 0;
        while disk[right_idx] == file_id && right_idx > 0 {
            right_idx -= 1;
            file_size += 1;
        }
        // println!("Moving file {file_id} with size {file_size}");

        // Find the earliest gap we can insert it into (increasing left_idx to first gap to avoid excessive checking)
        let mut gap_size = 0;
        let mut gap_idx = left_idx;
        // println!("Searching for a gap at {gap_idx}");
        while gap_size < file_size {
            if disk[gap_idx] < 0 {
                gap_size += 1;
            } else {
                gap_size = 0;
            }
            if gap_idx > right_idx {
                continue 'move_file  // can't move the file
            }
            gap_idx += 1;
        }
        // println!("Found gap of {gap_size} ending at {gap_idx}");

        // Move the file, if possible
        for _ in 0..file_size {
            disk[gap_idx - 1] = file_id;
            disk[right_idx + 1] = -1;
            gap_idx -= 1;
            right_idx += 1;
        }
        // let disk_slice = &disk[0..300];
        // println!("Disk is {disk:?}\n");

        // Update left_idx
        while disk[left_idx + 1] != -1 {
            left_idx += 1;
        }
    }

    let mut result: i64 = 0;
    let mut i: i64 = 0;
    for disk_val in disk.iter() {
        if *disk_val != -1 {
            result += disk_val * i;
        }
        i += 1;
    }
    
    return result
}

fn parse_input(file_path: &str) -> Vec<usize> {
    let mut result = Vec::new();
    const RADIX: u32 = 10;

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(memory) = line {
                result = memory.chars().map(|x| x.to_digit(RADIX).unwrap() as usize).collect();
            }
        }
    }

    return result
}
