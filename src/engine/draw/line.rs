use crate::engine::canvas::Canvas;
use crate::engine::geometry::line::Line;

// Draw a 1 width line of a color on a canvas
pub fn simple(canvas: &mut Canvas, line: &Line, color: u32) {
    let y_step: f32 = (line.y_end - line.y_start) as f32 / (line.x_end - line.x_start) as f32;
    let mut y: f32 = line.y_start as f32;

    for x in line.x_start..line.x_end {
        canvas.draw_point((x, y.round() as i32), color);
        y += y_step;
    }
}

// TODO: Implement AA line drawing
pub fn aa() {

}
