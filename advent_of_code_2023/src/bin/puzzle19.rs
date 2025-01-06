use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde::Deserialize;
use serde_json;

fn main() {
    //let file_path = "data/puzzle19/example.txt";
    let file_path = "data/puzzle19/input.txt";

    let ans = part_a(file_path);
    println!("Answer to puzzle A is {ans};");

    let ans = part_b(file_path);
    println!("Answer to puzzle B is {ans};");
}

#[derive(Deserialize, Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn part_a(file_path: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut rules: HashMap<String, Vec<(char, bool, i32, String)>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut key: String;
    //let mut next_rule: String;
    let mut new_rule: Vec<(char, bool, i32, String)>;

    let mut found_split: bool = false;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(mut ip) = line {
                if ip.len() == 0 {
                  found_split = true;
                  continue;
                }

                if !found_split {
                    let first_curl = ip.find('{').unwrap();
                    key = ip[0..first_curl].to_string();
                    new_rule = parse_rule(&ip[(first_curl + 1)..(ip.len() - 1)]);
                    rules.insert(key, new_rule);
                } else {
                    ip = ip.replace("x", "\"x\"");
                    ip = ip.replace("m", "\"m\"");
                    ip = ip.replace("a", "\"a\"");
                    ip = ip.replace("s", "\"s\"");
                    ip = ip.replace("=", ":");
                    let new_part: Part = serde_json::from_str(&ip).unwrap();
                    parts.push(new_part);
                }
            }
        }
    }
    //println!("Parsed rules and got {:?}", rules);
    //println!("Also found {} parts", parts.len());

    for part in parts {
        //println!("Processing part {:?}", part);
        let mut curr_flow = "in";
        while (curr_flow != "A") & (curr_flow != "R") {
            //println!("    Flow is {curr_flow}");
            let rule_list = rules.get(curr_flow).unwrap();
            for (attr, greater, val, dest) in rule_list {
                let obs_val: i32;
                match attr {
                    'x' => {obs_val = part.x;},
                    'm' => {obs_val = part.m;},
                    'a' => {obs_val = part.a;},
                    's' => {obs_val = part.s;},
                    '_' => {
                        //println!("        Failed to match, using default {dest}");
                        curr_flow = dest;
                        break;
                    }
                    _ => {panic!("Found invalid attr ({attr}) in rule!");}
                }
                if (obs_val > *val) ^ (!greater) {
                    //println!("        Matched rule, sending to {dest}");
                    curr_flow = dest;
                    break;
                }
            }
        }
        if curr_flow == "A" {
            //println!("Found accepted part {:?}, adding {}", part, part.x + part.m + part.a + part.s);
            ans += part.x + part.m + part.a + part.s;
        }
    }

    ans
}

fn part_b(file_path: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut rules: HashMap<String, Vec<(char, bool, i32, String)>> = HashMap::new();
    let mut key: String;
    let mut new_rule: Vec<(char, bool, i32, String)>;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(mut ip) = line {
                if ip.len() == 0 {
                  break
                }

                let first_curl = ip.find('{').unwrap();
                key = ip[0..first_curl].to_string();
                new_rule = parse_rule(&ip[(first_curl + 1)..(ip.len() - 1)]);
                rules.insert(key, new_rule);
            }
        }
    }

    count_accepted(1, 4000, 1, 4000, 1, 4000, 1, 4000,"in", &rules)
}

fn count_accepted(
        mut x_min: i64,
        mut x_max: i64,
        mut m_min: i64,
        mut m_max: i64,
        mut a_min: i64,
        mut a_max: i64,
        mut s_min: i64,
        mut s_max: i64,
        node: &str,
        rules: &HashMap<String, Vec<(char, bool, i32, String)>>) -> i64 {

    println!("Checking ({x_min}, {x_max}), ({m_min}, {m_max}), ({a_min}, {a_max}), ({s_min}, {s_max}) for node {node}");

    let mut ans = 0;
    if (x_min > x_max) | (m_min > m_max) | (a_min > a_max) | (s_min > s_max) {
        return ans;
    }

    if node == "A" {
        return (x_max - x_min + 1) * (m_max - m_min + 1) * (a_max - a_min + 1) * (s_max - s_min + 1);
    } else if node == "R" {
        return 0
    }

    let rule_list = rules.get(node).expect("Unexpected node found!");
    for (attr, greater, val, dest) in rule_list {
        let val = *val as i64;
        match attr {
            'x' => {
                if *greater {
                    ans += count_accepted(val + 1, x_max, m_min, m_max, a_min, a_max, s_min, s_max, dest, &rules);
                    x_max = val;
                } else {
                    ans += count_accepted(x_min, val - 1, m_min, m_max, a_min, a_max, s_min, s_max, dest, &rules);
                    x_min = val;
                }
            },
            'm' => {
                if *greater {
                    ans += count_accepted(x_min, x_max, val + 1, m_max, a_min, a_max, s_min, s_max, dest, &rules);
                    m_max = val;
                } else {
                    ans += count_accepted(x_min, x_max, m_min, val - 1, a_min, a_max, s_min, s_max, dest, &rules);
                    m_min = val;
                }
            },
            'a' => {
                if *greater {
                    ans += count_accepted(x_min, x_max, m_min, m_max, val + 1, a_max, s_min, s_max, dest, &rules);
                    a_max = val;
                } else {
                    ans += count_accepted(x_min, x_max, m_min, m_max, a_min, val -1, s_min, s_max, dest, &rules);
                    a_min = val;
                }
            },
            's' => {
                if *greater {
                    ans += count_accepted(x_min, x_max, m_min, m_max, a_min, a_max, val + 1, s_max, dest, &rules);
                    s_max = val;
                } else {
                    ans += count_accepted(x_min, x_max, m_min, m_max, a_min, a_max, s_min, val -1, dest, &rules);
                    s_min = val;
                }
            },
            '_' => {
                ans += count_accepted(x_min, x_max, m_min, m_max, a_min, a_max, s_min, s_max, dest, &rules);
                break
            }
            _ => {panic!("Found invalid attr ({attr}) in rule!");}
        }
    }
    ans
}

fn parse_rule(rule_str: &str) -> Vec<(char, bool, i32, String)> {
    let mut new_rule = Vec::new();
    let mut end_state: String = String::from("Should not be unitialized!");
    'str_parse: for str_to_parse in rule_str.split(',') {
        let mut char_iter = str_to_parse.chars();
        let var: char = char_iter.next().unwrap();
        let cond: bool;
        let cond_char: char;
        match char_iter.next() {
            Some(x) => {cond_char = x},
            None => {
                end_state = String::new();
                end_state.push(var);
                break 'str_parse;
            }
        }
        if cond_char == '>' {
            cond = true;
        } else if cond_char == '<' {
            cond = false
        } else {
            // In end state
            end_state = String::new();
            end_state.push(var);
            end_state.push(cond_char);
            loop {
                match char_iter.next() {
                    Some(x) => {end_state.push(x);}
                    None => {break 'str_parse;}
                }
            }
        }

        let mut val = String::new();
        loop {
            let next_char = char_iter.next().unwrap();
            if next_char == ':' {break;}
            val.push(next_char);
        }
        let val = val.parse::<i32>().unwrap();

        let mut next_rule = String::new();
        loop {
            match char_iter.next() {
                Some(x) => {next_rule.push(x);}
                None => {break;}
            };
        }
        new_rule.push((var, cond, val, next_rule));
    }
    // Add the end state
    new_rule.push(('_', false, 0, end_state));
    return new_rule
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
