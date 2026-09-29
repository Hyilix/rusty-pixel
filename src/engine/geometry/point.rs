use crate::engine::geometry::p2p_distance;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn pair_to_point(pos: (i32, i32)) -> Self {
        Self::new(pos.0, pos.1)
    }
}

impl Point {
    pub fn dist_to_other(&self, other: &Self) -> f32 {
        p2p_distance(self.x, self.y, other.x, other.y)
    }
}
