// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use num::traits::Zero;
use std::cmp::PartialEq;
use std::collections::{HashSet, HashMap};
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

#[derive(Clone, Eq, Hash, Debug)]
struct Node<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone> {
    pub pt: Point<T>,
    pub path: Vec<Point<T>>,
    pub dist: V,
}

impl<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone> PartialEq for Node<T, V> {
    fn eq(&self, other: &Self) -> bool {
        (self.pt.x == other.pt.x) & (self.pt.y == other.pt.y)
    }
}

pub fn dijkstra<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone + std::ops::Add<Output = V> + Zero + Copy>(
    start: Point<T>,
    end: Point<T>,
    adjacency: &HashMap<Point<T>, Vec<(Point<T>, V)>>
) -> (V, Vec<Point<T>>) {
    return dijkstra_vec(start, vec![end], adjacency)
}


pub fn dijkstra_vec<T: Eq + Hash + Clone, V: std::cmp::Ord + Hash + Clone + std::ops::Add<Output = V> + Zero + Copy>(
    start: Point<T>,
    end_pts: Vec<Point<T>>,
    adjacency: &HashMap<Point<T>, Vec<(Point<T>, V)>>
) -> (V, Vec<Point<T>>) {
    // Types:
    //    T: Type of x/y points (probably i32, u32, f32, etc.)
    //    V: Type of dist (probably u32 or f32, could be i32)
    // Args:
    //    start: Starting point
    //    end: Ending point
    //    adjacency: HashMap specifying, for each point, a vector of (Point, distance) tuples which are adjacent
    let mut unvisited = BinaryHeap::from_vec_cmp(
        vec![], |a: &Node<T, V>, b: &Node<T, V>| b.dist.cmp(&a.dist)
    );
    let mut visited: HashSet::<Point<T>> = HashSet::new();

    unvisited.push(Node{pt: start.clone(), path: vec![start], dist: V::zero()});

    loop {
        let curr_node = unvisited.pop().unwrap();
        if visited.contains(&curr_node.pt) {
            continue
        }
        visited.insert(curr_node.pt.clone());
        for end in end_pts.iter() {
            if curr_node.pt == *end {
                return (curr_node.dist, curr_node.path)
            }
        }
        let candidates = adjacency.get(&curr_node.pt).unwrap();
        for (cand, dist) in candidates {
            if visited.contains(cand) {
                continue
            }
            let mut new_path = curr_node.path.clone();
            new_path.push(cand.clone());
            unvisited.push(Node{pt: cand.clone(), path: new_path, dist: *dist + curr_node.dist});
        }
        if unvisited.len() == 0 {
            // return Err(("No nodes left!").into())
            panic!("No nodes left!");
        }
    }
}
