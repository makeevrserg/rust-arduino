use embedded_graphics::prelude::Point;

pub struct Square {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}

impl Square {
    pub const fn new(size: i32, center: Point) -> Self {
        Square {
            top_left: Point::new(center.x - size / 2, center.y - size / 2),
            top_right: Point::new(center.x + size / 2, center.y - size / 2),
            bottom_left: Point::new(center.x + size / 2, center.y + size / 2),
            bottom_right: Point::new(center.x - size / 2, center.y + size / 2),
        }
    }
}