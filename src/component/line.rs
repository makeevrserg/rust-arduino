use crate::renderer::{Component, Point, Renderer};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: bool,
}

impl Line {
    pub const fn new(start: Point, end: Point) -> Self {
        Line {
            start,
            end,
            color: true,
        }
    }

    pub const fn with_color(start: Point, end: Point, color: bool) -> Self {
        Line { start, end, color }
    }
}

impl Component for Line {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_line(self.start, self.end, self.color);
    }
}
