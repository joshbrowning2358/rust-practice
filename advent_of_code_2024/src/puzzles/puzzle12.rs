use std::collections::{HashSet, BTreeSet};

use crate::file_reader;

pub fn part_a(file_path: &str) -> i32 {
    let garden = parse_input(file_path);
    let plots = build_plots(&garden);

    return cost_of_plots(&plots[1..].to_vec()) as i32
}

pub fn part_b(file_path: &str) -> i32 {
    let garden = parse_input(file_path);
    let plots = build_plots(&garden);

    return cost_of_plots_discount(&plots[1..].to_vec()) as i32
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    if let Ok(lines) = file_reader::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                if result.len() == 0 {
                    // Prepend row of '?'
                    result.push(vec!['?'; row.len() + 2]);
                }
                let mut next_row = vec!['?'];
                next_row.append(&mut row.chars().collect());
                next_row.append(&mut vec!['?']);
                result.push(next_row);
            }
        }
    }
    result.push(vec!['?'; result[0].len()]);

    return result;
}

fn build_plots(garden: &Vec<Vec<char>>) -> Vec<HashSet<(usize, usize)>> {
    let mut to_explore: BTreeSet::<(usize, usize)> = BTreeSet::new();
    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            to_explore.insert((i, j));
        }
    }

    // Explore!
    let mut plots: Vec<HashSet<(usize, usize)>> = Vec::new();
    while to_explore.len() > 0 {
        let new_node = to_explore.pop_first().unwrap();
        plots.push(explore(new_node, &mut to_explore, garden));
    }
    return plots
}

fn explore(start: (usize, usize), to_explore: &mut BTreeSet<(usize, usize)>, garden: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut to_check: Vec<(usize, usize)> = vec![start];
    let plot_type = garden[start.0][start.1];
    let mut plot: HashSet<(usize, usize)> = HashSet::new();
    plot.insert(start);

    while to_check.len() > 0 {
        let next_pos = to_check.pop().unwrap();
        to_explore.remove(&next_pos);

        if next_pos.0 > 0 && garden[next_pos.0 - 1][next_pos.1] == plot_type {
            check_new_point((next_pos.0 - 1, next_pos.1), &mut to_check, &mut plot);
        }
        if next_pos.1 > 0 && garden[next_pos.0][next_pos.1 - 1] == plot_type {
            check_new_point((next_pos.0, next_pos.1 - 1), &mut to_check, &mut plot);
        }
        if next_pos.0 < garden.len() - 1 && garden[next_pos.0 + 1][next_pos.1] == plot_type {
            check_new_point((next_pos.0 + 1, next_pos.1), &mut to_check, &mut plot);
        }
        if next_pos.1 < garden[0].len() - 1 && garden[next_pos.0][next_pos.1 + 1] == plot_type {
            check_new_point((next_pos.0, next_pos.1 + 1), &mut to_check, &mut plot);
        }
    }

    return plot
}

fn check_new_point(pt: (usize, usize), to_check: &mut Vec<(usize, usize)>, plot: &mut HashSet<(usize, usize)>) {
    if !plot.contains(&pt) {
        to_check.push(pt);
        plot.insert(pt);
    }
}

fn cost_of_plots(plots: &Vec<HashSet<(usize, usize)>>) -> usize {
    let mut cost: usize = 0;
    
    for plot in plots {
        // println!("Plot is {plot:?}");
        let area = plot.len();
        let mut perimeter: usize = 0;
        for pt in plot.iter() {
            for new_pt in vec![(pt.0 - 1, pt.1), (pt.0 + 1, pt.1), (pt.0, pt.1 - 1), (pt.0, pt.1 + 1)] {
                if !plot.contains(&new_pt) {
                    perimeter += 1;
                }
            }
        }
        let plot_cost = area * perimeter;
        // println!("Cost is {area}x{perimeter} = {plot_cost}");
        cost += plot_cost;
    }
    return cost
}

fn cost_of_plots_discount(plots: &Vec<HashSet<(usize, usize)>>) -> usize {
    let mut cost: usize = 0;

    for plot in plots {
        let area = plot.len();
        let mut boundary: Vec<(usize, usize, u8)> = Vec::new();
        for pt in plot.iter() {
            if !plot.contains(&(pt.0 - 1, pt.1)) {
                boundary.push((pt.0 - 1, pt.1, 0));
            }
            if !plot.contains(&(pt.0 + 1, pt.1)) {
                boundary.push((pt.0 + 1, pt.1, 1));
            }
            if !plot.contains(&(pt.0, pt.1 - 1)) {
                boundary.push((pt.0, pt.1 - 1, 2));
            }
            if !plot.contains(&(pt.0, pt.1 + 1)) {
                boundary.push((pt.0, pt.1 + 1, 3));
            }
        }
        let perimeter = reduce_boundary(boundary);
        let plot_cost = area * perimeter;
        //println!("Cost is {area}x{perimeter} = {plot_cost}");
        cost += plot_cost;
    }
    return cost
}

fn reduce_boundary(mut boundary: Vec<(usize, usize, u8)>) -> usize {
    // Sort boundary.
    // - First, sort fence types together (i.e. crossing from in to out moving up)
    // - Then, sort fences on the same line
    // - Finally, sort by final pos
    boundary.sort_by(|a, b| if a.2 == b.2 {
            if a.2 == 2 || a.2 == 3 {
                if a.1 == b.1 {
                    a.0.partial_cmp(&b.0).unwrap()
                } else {
                    a.1.partial_cmp(&b.1).unwrap()
                }
            } else {
                if a.0 == b.0 {
                    a.1.partial_cmp(&b.1).unwrap()
                } else {
                    a.0.partial_cmp(&b.0).unwrap()
                }
            }
        }
        else {
            a.2.partial_cmp(&b.2).unwrap()
        }
    );

    // Group together sides and compute total perimeter
    let mut perimeter: usize = 0;
    let mut last_fence = (0, 0, 5);
    for fence in boundary.iter() {
        if fence.2 != last_fence.2 {
            perimeter += 1;
        } else {
            if fence.2 == 2 || fence.2 == 3 {
                if !(fence.1 == last_fence.1 && fence.0 == last_fence.0 + 1) {
                    perimeter += 1;
                }
            } else {
                if !(fence.0 == last_fence.0 && fence.1 == last_fence.1 + 1) {
                    perimeter += 1;
                }
            }
        }
        last_fence = *fence;
    }
    //println!("Boundary is {boundary:?}, perimeter is {perimeter}");
    return perimeter
}






