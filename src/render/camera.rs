// The camera of the application

use crate::render::canvas;

use crate::geometry::rectangle;

pub struct Camera {
    area: rectangle::Rectangle,
    zoom: f32,
    canvas: canvas::Canvas,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0, 0, 1, 1, 1f32)
    }
}

// Boiler plate stuff
impl Camera {
    pub fn new(px: u32, py: u32, sx: u32, sy: u32, zoom: f32) -> Self {
        Self {
            area: rectangle::Rectangle::new(px, py, sx, sy),
            zoom,
            canvas: canvas::Canvas::default(),
        }
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_position(&self) -> (u32, u32) {
        (self.area.x, self.area.y)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.area.width, self.area.height)
    }

    pub fn get_x(&self) -> u32 {
        self.area.x
    }

    pub fn get_y(&self) -> u32 {
        self.area.y
    }

    pub fn get_width(&self) -> u32 {
        self.area.width
    }

    pub fn get_height(&self) -> u32 {
        self.area.height
    }

    pub fn set_width(&mut self, new_w: u32) {
        self.area.width = new_w;
    }

    pub fn set_height(&mut self, new_h: u32) {
        self.area.height = new_h;
    }

    pub fn get_canvas(&self) -> &canvas::Canvas {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut canvas::Canvas {
        &mut self.canvas
    }
}

// Camera methods
impl Camera {
    pub fn present_to_screen(
        &self,
        screen: &mut canvas::Canvas,
        screen_size: (u32, u32),
    ) {
        let mut clone: canvas::Canvas = self.canvas.clone();

        clone.scale_by(self.zoom);

        let pos : (u32, u32) = (
            screen_size.0 / 2 - clone.get_width() / 2,
            screen_size.1 / 2 - clone.get_height() / 2,
        );

        println!("Present position: {:?} , {}, {}", pos, self.area.x, self.area.y);

        screen.blit(
            &clone.get_zone(
                &rectangle::Rectangle::new(
                    (pos.0 as i32 - self.area.x as i32).min(0) as u32,
                    (pos.1 as i32 - self.area.y as i32).min(0) as u32,
                    self.area.width,
                    self.area.height,
                )),
            pos);
    }
}
