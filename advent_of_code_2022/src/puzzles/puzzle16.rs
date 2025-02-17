use std::collections::HashMap;

use memoize::memoize;

use crate::file_reader;
use crate::puzzle_solver;

pub struct Puzzle16;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    flow: u32,
    connected: Vec<String>,
}


impl puzzle_solver::Solver for Puzzle16 {
    fn part_1(&self, file_path: &str) -> String {
        let mut cache = get_global_hashmap();
        let valves = parse(file_path);
        let open_valves = vec![];
        let press_relieved = max_pressure_relieved("AA".to_string(), 20, &open_valves, &valves);
        return press_relieved.to_string()
    }

    fn part_2(&self, file_path: &str) -> String {
        return String::from("TODO")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        ("", "")
    }

    fn name(&self) -> &'static str {
        "Day 16: Proboscidea Volcanium"
    }
}

fn max_pressure_relieved(
    current_loc: String,
    steps_remaining: u32,
    open_valves: &Vec<String>,
    valves: &HashMap<String, Valve>,
) -> u32 {
    if (current_loc, steps_remaining, open_valves) in cache {
        return cache.get((current_loc, steps_remaining, open_valves)).unwrap()
    }
    let mut baseline = 0;
    for valve_id in open_valves {
        baseline += valves.get(valve_id).unwrap().flow;
    }

    if steps_remaining <= 1 {
        // No time to open anything impactful, just return baseline
        return baseline
    }

    let mut best = baseline;
    if ! open_valves.contains(&current_loc) {
        let mut new_valves = open_valves.clone();
        new_valves.push(current_loc.clone());
        best = baseline + max_pressure_relieved(current_loc.clone(), steps_remaining - 1, &new_valves, valves);
    }
    if steps_remaining == 2 {
        // No time to move and open, so open current and move on
        return best
    }
    for valve_id in &valves.get(&current_loc).unwrap().connected {
        let cand = baseline + max_pressure_relieved(valve_id.clone(), steps_remaining - 1, &open_valves, valves);
        if cand > best {
            best = cand;
        }
    }
    cache.insert((current_loc, steps_remaining, open_valves), best);
    return best
}

fn parse(file_path: &str) -> HashMap<String, Valve>{
    let mut output = HashMap::new();
    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                println!("Row is {row:?}");
                let (mut valve_id, remaining) = row.split_once(" has flow rate=").unwrap();
                (_, valve_id) = valve_id.split_once("alve ").unwrap();
                let (flow_str, mut connected_str) = remaining.split_once("; tunnel").unwrap();
                (_, connected_str) = connected_str.split_once("valve").unwrap();
                (_, connected_str) = connected_str.split_once(" ").unwrap();
                let flow: u32 = flow_str.parse::<u32>().unwrap();
                let connected = match connected_str.contains(",") {
                    true => connected_str.split(", ").map(|x| x.to_string()).collect(),
                    false => vec![connected_str.to_string()]
                };
                output.insert(
                    valve_id.to_string(),
                    Valve{flow: flow, connected: connected}
                );
            }
        }
    }
    return output
}

fn get_global_hashmap() -> MutexGuard<'static, HashMap<(String, u32, &Vec<String>), u32>> {
    static map: OnceLock<Mutex<HashMap<(String, u32, &Vec<String>), u16>>> = OnceLock::new();
    map.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned")
}
