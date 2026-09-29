use crate::engine::canvas::Canvas;
use crate::engine::geometry::{triangle::Triangle, point::Point};

// Draw a filled triangle of a color onto a canvas
pub fn filled(canvas: &mut Canvas, tri: &Triangle, color: u32) {
    let x_start: i32 = (tri.corner1.x).min(tri.corner2.x).min(tri.corner3.x);
    let x_end: i32 = (tri.corner1.x).max(tri.corner2.x).max(tri.corner3.x);
    let y_start: i32 = (tri.corner1.y).min(tri.corner2.y).min(tri.corner3.y);
    let y_end: i32 = (tri.corner1.y).max(tri.corner2.y).max(tri.corner3.y);

    println!("For triangle with corners ({:?} {:?} {:?}), look for ({x_start} {y_start}), ({x_end} {y_end})",
        tri.corner1,tri.corner2,tri.corner3);

    for y in y_start..y_end + 1 {
        for x in x_start..x_end + 1 {
            let curr_point: Point = Point::pair_to_point((x, y));

            if tri.contains(curr_point) {
                canvas.draw_point((x, y), color);
            }
        }
    }
}
