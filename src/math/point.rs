#![no_std]


use embedded_graphics::geometry::Point;
use crate::math::math::{cos_fast, sin_fast};

pub fn rotate_point(p: Point, cx: i32, cy: i32, angle: i32) -> Point {
    // Convert milli-radians to degrees (1000 milli-radians = 1 radian)
    let angle_rad = (angle as f32) / 1000.0;
    let angle_deg = angle_rad * 180.0 / core::f32::consts::PI;

    // Translate point to origin
    let x = (p.x - cx) as f32;
    let y = (p.y - cy) as f32;

    // Apply rotation matrix
    let cos_a = cos_fast(angle_deg);
    let sin_a = sin_fast(angle_deg);

    let xr = x * cos_a - y * sin_a;
    let yr = x * sin_a + y * cos_a;

    // Translate back and round to i32
    Point::new(
        (xr + cx as f32) as i32,
        (yr + cy as f32) as i32,
    )
}
