// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::Hash;

use binary_heap_plus::BinaryHeap;

#[derive(Debug, Clone, Eq, Hash, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::cmp::PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}

pub fn display_grid(grid: Vec<Vec<char>>) {
    for row in grid {
        let disp_row: String = row.iter().collect();
        println!("{disp_row:?}");
    }
}

//trait InfinityNorm<T> {
//    fn inf_norm(&self, other: &Self) -> T;
//}
//
//impl<T: std::ops::Sub, std::ops::abs> InfinityNorm<T> for Point<T> {
//    fn inf_norm(&self, other: &Self) -> T {
//        let x_dist = (self.x - other.x).abs();
//        let y_dist = (self.y - other.y).abs();
//        if x_dist > y_dist {
//            return x_dist
//        }
//        return y_dist
//    }
//}

#[derive(Clone, Eq, Hash)]
pub struct Node<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone> {
    pub pt: Point<T>,
    pub path: Vec<Node<T, V>>,
    pub dist: V,
}

impl<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone> PartialEq for Node<T, V> {
    fn eq(&self, other: &Self) -> bool {
        (self.pt.x == other.pt.x) & (self.pt.y == other.pt.y)
    }
}

pub fn dijkstra<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone>(start: Node<T, V>, end: Node<T, V>, get_neighbors: impl Fn(&Node<T, V>) -> Vec<Node<T, V>>) -> (V, Vec<Node<T, V>>) {
    // Types:
    //    T: Type of x/y points (probably i32, u32, f32, etc.)
    //    V: Type of dist (probably u32 or f32)
    // Args:
    //    start: Starting node
    //    end: Ending node
    //    get_neighbors: Function getting neighbors when given node.  Should also update the paths of said neighbors to include themselves.
    let mut unvisited = BinaryHeap::from_vec_cmp(
        vec![], |a: &Node<T, V>, b: &Node<T, V>| b.dist.cmp(&a.dist)
    );
    let mut visited: HashSet::<Node<T, V>> = HashSet::new();
    // let mut to_explore_dists: HashMap::<Point, i32> = HashMap::new();

    unvisited.push(start);

    let final_dist: V;
    let final_path: Vec<Node<T, V>>;
    loop {
        let curr_node = unvisited.pop().unwrap();
        if visited.contains(&curr_node) {
            continue
        }
        visited.insert(curr_node.clone());
        if curr_node == end {
            final_dist = curr_node.dist;
            final_path = curr_node.path;
            break
        }
        let candidates = get_neighbors(&curr_node);
        for cand in candidates {
            if visited.contains(&cand) {
                continue
            }
            unvisited.push(cand);
        }
        if unvisited.len() == 0 {
            panic!("No new nodes to visit!");
        }
    }
    return (final_dist, final_path)
}
