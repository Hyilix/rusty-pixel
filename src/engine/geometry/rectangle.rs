pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {x, y, width, height}
    }

    // Get the rectangle area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Get the rectangle perimeter
    pub fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}
