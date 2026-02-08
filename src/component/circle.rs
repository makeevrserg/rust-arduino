use crate::renderer::{Component, Point, Renderer};

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
        renderer.draw_circle(self.center, self.radius, self.color, self.fill);
    }
}
