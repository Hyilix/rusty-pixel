pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: u32) -> Self {
        Self {x, y, radius}
    }
}
