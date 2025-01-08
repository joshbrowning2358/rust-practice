use crate::file_reader;
use crate::puzzle_solver;

#[derive(Clone, Debug)]
pub struct Node {
    children: Vec<Node>,
    node_size: u64,
    name: String,
}

pub struct Puzzle07;

impl puzzle_solver::Solver for Puzzle07 {
    fn part_1(&self, input: &str) -> String {
        return String::from("TODO")
    }

    fn part_2(&self, input: &str) -> String {
        return String::from("TODO")
    }

    fn expected(&self) -> (&'static str, &'static str) {
        // ("2756096", "23117829")
        ("0", "0")
    }

    fn name(&self) -> &'static str {
        "Day 7: No Space Left On Device"
    }

}

fn parse(input: &str, _: bool) -> Node {
    let mut current_node = Node{children: vec![], node_size: 0, name: String::from("root")};
    let root = current_node;
    if let Ok(lines) = file_reader::read_lines(input) {
        for line in lines {
            if let Ok(command) = line {
                if command == "$ cd .." {
                    (current_node, _) = get_parent(&root, &current_node);
                } else if command.starts_with("$ cd") {
                    let (_, new_name) = command.split_once("cd ").unwrap();
                    let mut new_node = Node{children: vec![], node_size: 0, name: new_name.to_string()};
                    current_node.children.push(new_node);
                    current_node = new_node;
                } else if command.starts_with("$ ls") {
                    continue
                } else {
                    let (size_str, _) = command.split_once(" ").unwrap();
                    current_node.node_size += size_str.parse::<u64>().unwrap();
                }
            }
        }
    }
    return root
}



fn get_parent(root: &Node, current_node: &Node) -> (Node, bool) {
    for child in root.children {
        if child.name == current_node.name {
            return (root.clone(), true)
        }
        let (possible_parent, success) = get_parent(&child, current_node);
        if success {
            return (possible_parent, true)
        }
    }
    return (root.clone(), false)
}

