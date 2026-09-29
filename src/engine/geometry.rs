pub mod point;
pub mod rectangle;
pub mod circle;
pub mod line;
pub mod triangle;

#[inline]
pub fn p2p_distance(x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> f32 {
    let x_dif: f32 = (x_end - x_start) as f32;
    let y_dif: f32 = (y_end - y_start) as f32;
    f32::sqrt(x_dif * x_dif + y_dif * y_dif)
}
