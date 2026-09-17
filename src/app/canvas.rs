// Used to store information about the pixels
use crate::geometry::rectangle;

#[inline]
fn flat_index(x: u32, y: u32, w: u32) -> usize {
    (y * w + x) as usize
}

#[inline]
fn from_index(i: u32, w: u32) -> (u32, u32) {
    (i % w, i / w)
}

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Default for Canvas {
    // Default canvas implementation with some arbitrary values used
    fn default() -> Self {
        Self::new(8, 8, 0xFF000000)
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

    pub fn into_pixels(self) -> Vec<u32> {
        self.pixels
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

// Canvas methods
impl Canvas {
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

                // Check for bounds
                if dst_y >= self.height || dst_x >= self.width {
                    continue;
                }

                self.pixels[dst_index] = src.get_pixels()[src_index];
            }
        }
    }

    // Fill entire canvas with a color
    pub fn fill(
        &mut self,
        color: u32,
    ) {
        self.pixels = vec![color; (self.width * self.height) as usize];
    }

    // Fill a rectangle portion of the canvas
    pub fn fill_rect(
        &mut self,
        color: u32,
        rect: rectangle::Rectangle,
    ) {
        for y in 0..rect.height {
            for x in 0..rect.width {
                let dx = x + rect.x;
                let dy = y + rect.y;
                let src_index: usize = flat_index(dx, dy, self.width);

                // Check for bounds
                if dy >= self.height || dx >= self.width {
                    continue;
                }

                self.pixels[src_index] = color;
            }
        }
    }

    // Add a new color to the canvas every 'steps' pixels
    pub fn dither(
        &mut self,
        color: u32,
        steps: u32,
    ) {
        let mut current_steps = 0;
        for pixel in self.pixels.iter_mut() {
            current_steps += 1;

            if current_steps % steps != 0 {
                continue;
            }

            *pixel = color;
        }
    }

    // Scale image by factor
    pub fn scale_by(
        &mut self,
        factor: f32,
    ) {
        let width: u32 = (self.width as f32 * factor) as u32;
        let height: u32 = (self.width as f32 * factor) as u32;
        let mut pixels: Vec<u32> = vec![0; (width * height) as usize];

        for i in 0..(width * height) {
            let (new_x, new_y) = from_index(i, width);
            let (ratio_x, ratio_y): (f64, f64) = (
                new_x as f64 / width as f64,
                new_y as f64 / height as f64
                );

            let (old_x, old_y): (u32, u32) = (
                ((ratio_x * self.width as f64).floor() as u32).min(self.width - 1),
                ((ratio_y * self.height as f64).floor() as u32).min(self.height - 1),
                );

            let old_index = flat_index(old_x, old_y, self.width);

            pixels[i as usize] = self.pixels[old_index];
        }

        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }

    // Scale image to new size
    pub fn scale_to(
        &mut self,
        width: u32,
        height: u32,
    ) {
        let mut pixels: Vec<u32> = vec![0; (width * height) as usize];

        for i in 0..(width * height) {
            let (new_x, new_y) = from_index(i, width);
            let (ratio_x, ratio_y): (f64, f64) = (
                new_x as f64 / width as f64,
                new_y as f64 / height as f64
                );

            let (old_x, old_y): (u32, u32) = (
                ((ratio_x * self.width as f64).floor() as u32).min(self.width - 1),
                ((ratio_y * self.height as f64).floor() as u32).min(self.height - 1),
                );

            let old_index = flat_index(old_x, old_y, self.width);

            pixels[i as usize] = self.pixels[old_index];
        }

        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }
}

// Specific methods
impl Canvas {
    // Convert canvas content to screen size
    pub fn canvas_to_screen(&self, width: u32, height: u32) -> Vec<u32> {
        let mut temp_canvas: Canvas = Canvas::new(width, height, 0xFF000000);

        temp_canvas.blit(self, (100, 100));

        temp_canvas.into_pixels()
    }
}
