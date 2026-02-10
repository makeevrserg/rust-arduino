use crate::renderer::point::Point;

pub trait Canvas {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn center(&self) -> Point {
        Point::new(self.width() / 2, self.height() / 2)
    }
}
