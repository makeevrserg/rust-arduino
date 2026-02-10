use crate::renderer::Point;
use crate::math::trigonometry::{cos_fast, sin_fast};

fn rotate(p: Point, c: Point, angle: i32) -> Point {
    // Convert milli-radians to degrees
    let angle_rad = (angle as f32) / 1000.0;
    let angle_deg = angle_rad * 180.0 / core::f32::consts::PI;

    // Translate point to origin
    let x = (p.x - c.x) as f32;
    let y = (p.y - c.y) as f32;

    // Apply rotation matrix
    let cos_a = cos_fast(angle_deg);
    let sin_a = sin_fast(angle_deg);

    let xr = x * cos_a - y * sin_a;
    let yr = x * sin_a + y * cos_a;

    Point::new(
        (xr + c.x as f32) as i32,
        (yr + c.y as f32) as i32,
    )
}

pub trait Rotator {
    fn rotate(self, center: Point, angle: i32) -> Self;
}

impl Rotator for Point {
    fn rotate(self, center: Point, angle: i32) -> Self {
        rotate(self, center, angle)
    }
}
