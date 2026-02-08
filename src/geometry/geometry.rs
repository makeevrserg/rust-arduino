#![no_std]

use embedded_graphics::{
    geometry::Point,
};
use crate::math::point::rotate_point;

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
            bottom_right: Point::new(center.x + size / 2, center.y + size / 2),
            bottom_left: Point::new(center.x - size / 2, center.y + size / 2),
        }
    }
    pub fn corners(&self) -> [Point; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        ]
    }
}

impl Square {
    pub fn rotate(&self, angle: i32, center: Point) -> Self {
        Square {
            top_left: rotate_point(self.top_left, center.x, center.y, angle),
            top_right: rotate_point(self.top_right, center.x, center.y, angle),
            bottom_left: rotate_point(self.bottom_left, center.x, center.y, angle),
            bottom_right: rotate_point(self.bottom_right, center.x, center.y, angle),
        }
    }
}