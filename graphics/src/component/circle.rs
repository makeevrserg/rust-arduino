use crate::renderer::{Point, Renderer};
use crate::component::Component;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: bool,
    pub fill: bool,
}

impl Circle {
    pub const fn new(center: Point, size: i32, color: bool, fill: bool) -> Self {
        Circle {
            center: center,
            radius: size,
            color: color,
            fill: fill,
        }
    }
}

impl Component for Circle {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let adjusted_center = Point {
            x: self.center.x - self.radius,
            y: self.center.y - self.radius,
        };
        renderer.draw_circle(adjusted_center, self.radius, self.color, self.fill);
    }
}
