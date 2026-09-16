// Used to store information about the pixels
#[inline]
fn flat_index(x: u32, y: u32, w: u32) -> usize {
    (y * w + x) as usize
}

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Default for Canvas {
    // Default canvas implementation with some arbitrary values used
    fn default() -> Self {
        Self::new(8, 8, 0x000000AA)
    }
}

// Boiler plate stuff
impl Canvas {
    pub fn new(width: u32, height : u32, color: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; (width * height) as usize],
        }
    }

    pub fn get_pixels(&self) -> &Vec<u32> {
        &self.pixels
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn into_pixels(self) -> Vec<u32> {
        self.pixels
    }
}

// Canvas methods
impl Canvas {
    // pub fn pixel_at_pos(x: u32, y: u32) -> usize {
    //
    // }

    // Draw a source canvas into self at a position offset
    pub fn blit(
        &mut self,
        src: &Canvas,
        position: (u32, u32),
    ) {
        for y in 0..src.height {
            for x in 0..src.width {
                let src_index: usize = flat_index(x, y, src.get_width());
                let dst_y = position.1 + y;
                let dst_x = position.0 + x;
                let dst_index: usize = flat_index(dst_x, dst_y, self.get_width());

                self.pixels[dst_index] = src.get_pixels()[src_index];
            }
        }
    }
}

// Specific methods
impl Canvas {
    // Convert canvas content to screen size
    pub fn canvas_to_screen(&self, width: u32, height: u32) -> Vec<u32> {
        let mut temp_canvas: Canvas = Canvas::new(width, height, 0u32);

        temp_canvas.blit(self, (100, 100));

        temp_canvas.into_pixels()
    }
}
