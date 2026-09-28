pub struct Line {
    pub x_start: i32,
    pub y_start: i32,
    pub x_end: i32,
    pub y_end: i32,
}

impl Default for Line {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Line {
    pub fn new(x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Self {
        Self {x_start, y_start, x_end, y_end}
    }

    // Get the magnitude of the line
    pub fn magnitude(&self) -> f32 {
        let x_dif: f32 = (self.x_end - self.x_start) as f32;
        let y_dif: f32 = (self.y_end - self.y_start) as f32;
        f32::sqrt(x_dif * x_dif + y_dif * y_dif)
    }
}
