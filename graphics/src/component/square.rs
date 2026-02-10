use crate::math::rotator::Rotator;
use crate::renderer::{Point, Renderer};
use crate::component::line::Line;
use crate::component::Component;

#[derive(Clone, Copy, Debug)]
pub struct Square {
    center: Point,
    size: i32,
    angle: i32,
    color: bool,
}

impl Square {
    pub const fn new(center: Point, size: i32) -> Self {
        Square {
            center,
            size,
            angle: 0,
            color: true,
        }
    }

    pub const fn with_rotation(self, angle: i32) -> Self {
        Square { angle, ..self }
    }

    pub const fn with_color(self, color: bool) -> Self {
        Square { color, ..self }
    }

    pub fn top_left(&self) -> Point {
        let half = self.size / 2;
        Point::new(self.center.x - half, self.center.y - half).rotate(self.center, self.angle)
    }
    pub fn top_right(&self) -> Point {
        let half = self.size / 2;
        Point::new(self.center.x + half, self.center.y - half).rotate(self.center, self.angle)
    }
    pub fn bottom_right(&self) -> Point {
        let half = self.size / 2;
        Point::new(self.center.x + half, self.center.y + half).rotate(self.center, self.angle)
    }
    pub fn bottom_left(&self) -> Point {
        let half = self.size / 2;
        Point::new(self.center.x - half, self.center.y + half).rotate(self.center, self.angle)
    }
    fn corners(&self) -> [Point; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bottom_right(),
            self.bottom_left(),
        ]
    }

    pub fn edges(&self) -> [Line; 4] {
        let corners = self.corners();
        [
            Line::with_color(corners[0], corners[1], self.color),
            Line::with_color(corners[1], corners[2], self.color),
            Line::with_color(corners[2], corners[3], self.color),
            Line::with_color(corners[3], corners[0], self.color),
        ]
    }
}

impl Component for Square {
    fn draw(&self, renderer: &mut dyn Renderer) {
        for edge in self.edges() {
            edge.draw(renderer);
        }
    }
}
