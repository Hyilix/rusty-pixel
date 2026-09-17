pub struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Circle {
    pub fn new(x: u32, y: u32, radius: u32) -> Self {
        Self {x, y, radius}
    }
}
