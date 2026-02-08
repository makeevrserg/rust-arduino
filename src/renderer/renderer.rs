use crate::renderer::point::Point;
use crate::renderer::canvas::Canvas;

pub trait Renderer {
    fn draw_pixel(&mut self, point: Point, color: bool);

    fn draw_line(&mut self, p1: Point, p2: Point, color: bool);

    fn draw_circle(&mut self, center: Point, radius: i32, color: bool, filled: bool);

    fn clear(&mut self, color: bool);

    fn flush(&mut self);

    fn canvas(&self) -> &dyn Canvas;
}
