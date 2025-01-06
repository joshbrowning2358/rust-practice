// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug, Clone, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}
