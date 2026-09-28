use crate::engine::canvas::Canvas;
use crate::engine::geometry::rectangle::Rectangle;

// Draw a filled rectangle on a canvas with a given color
pub fn filled(surf: &mut Canvas, rect: &Rectangle, color: u32) {
    surf.fill_rect(rect, color);
}

// Draw an outline with the given width of a rectangle on a canvas with a given color
pub fn outlined(surf: &mut Canvas, rect: &Rectangle, color: u32, width: i16) {
    // TODO: Implement
}

// TODO: rounded rectangle corners
pub fn rounded_filled() {

}

pub fn rounded_outlined() {

}
