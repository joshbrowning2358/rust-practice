use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

const HUGE_DISTANCE: i32 = 100000000;
const RADIX: u32 = 10;

#[derive(Copy, Clone)]
struct Node {
    position: [i32; 2],
    direction: [i32; 2],
    streak: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.position, self.streak, self.direction).cmp(&(other.position, other.streak, other.direction))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.position, self.streak, self.direction) == (other.position, other.streak, other.direction)
    }
}

impl Eq for Node { }

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
        self.streak.hash(state);
    }
}


fn main() {
    //let file_path = "data/puzzle17/example.txt";
    let file_path = "data/puzzle17/input.txt";
    //let file_path = "data/puzzle17/easy.txt";
    //let file_path = "data/puzzle17/super_easy.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

fn part_a(file_path: &str) -> i32 {
    let mut grid_str = String::new();
    let mut nrows: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                grid_str.push_str(&ip.to_string());
                nrows += 1;
            }
        }
    }
    let grid: Vec<i32> = grid_str.chars().map(|x| x.to_digit(RADIX).unwrap() as i32).collect();
    let ncols: usize = grid.len() / nrows;
    let mut shortest_path: HashMap<Node, i32> = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    to_visit.push(Reverse((0, Node{position: [0, 0], direction: [0, 0], streak: 0})));
    let mut ans = HUGE_DISTANCE;
    let mut iter_counter = 0;

    // Get next node to visit
    loop {
        if to_visit.len() == 0 {
            println!("Exhausted all search options, done!");
            break
        }
        let Reverse((next_distance, next_node)) = to_visit.pop().unwrap();
        //println!("Now visiting {:?} with distance {next_distance} and properties streak={} direction={:?}", next_node.position, next_node.streak, next_node.direction);

        let curr_distance = match shortest_path.get(&next_node) {
            Some(x) => {
                if x > &ans {
                    break;  // Found a path to destination that's closer than the smallest remaining distance 
                }
                *x
            },
            None => {HUGE_DISTANCE}
        };

        if next_distance < curr_distance {
            shortest_path.insert(next_node, next_distance); // Update distance with new best
        }

        // Visit node and update distances
        for (row_step, col_step) in zip([-1, 1, 0, 0], [0, 0, -1, 1]) {
            if ((row_step == -1 * next_node.direction[0]) & (row_step != 0)) | ((col_step == -1 * next_node.direction[1]) & (col_step != 0)) {
                continue // Can't reverse the crucible back the way it came
            }
            let next_pos = [next_node.position[0] + row_step, next_node.position[1] + col_step];
            if (next_pos[0] < 0) | (next_pos[0] >= nrows as i32) | (next_pos[1] < 0) | (next_pos[1] >= ncols as i32) {
                continue
            }
            let mut current_streak = 1;
            if next_node.direction == [row_step, col_step] {
                current_streak = next_node.streak + 1;
            }
            if current_streak < 4 {
                //println!("Using index {} using {}x{nrows} + {}", (next_pos[0] as usize) * ncols + (next_pos[1] as usize), next_pos[0], next_pos[1]);
                let new_distance = &(grid[(next_pos[0] as usize) * ncols + (next_pos[1] as usize)] + next_distance);
                //println!("Current node {:?}|{}, adding {:?}|{}", next_node.position, next_distance, next_pos, new_distance);
                let new_node = Node{position: next_pos, direction: [row_step, col_step], streak: current_streak};
                if !visited.contains(&new_node) {
                    to_visit.push(Reverse((*new_distance, new_node)));
                }
                if next_pos == [(nrows - 1) as i32, (ncols - 1) as i32] {
                    ans = min(ans, *new_distance);
                }
            }
        }
        visited.insert(next_node);
        iter_counter += 1;
        if visited.len() % 1000 == 0 {
            println!("Visited {} nodes!", visited.len());
        }
        //if iter_counter > 100 {
        //    break
        //}
    }

    ans
}

fn part_b(file_path: &str) -> i32 {
    let mut ans: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    ans
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
