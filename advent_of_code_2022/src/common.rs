// An attribute to hide warnings for unused code.
#![allow(dead_code)]

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
