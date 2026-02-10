use crate::component::circle::Circle;
use crate::component::Component;
use crate::renderer::{Point, Renderer, Updatable};

#[derive(Clone, Copy, Debug)]
pub struct PulsatingCircle {
    center: Point,
    min_radius: i32,
    max_radius: i32,
    current_radius: i32,
    direction: i32,
    color: bool,
    filled: bool,
}

impl PulsatingCircle {
    pub const fn new(center: Point, min_radius: i32, max_radius: i32) -> Self {
        PulsatingCircle {
            center,
            min_radius,
            max_radius,
            current_radius: min_radius,
            direction: 1,
            color: true,
            filled: true,
        }
    }

    pub const fn with_color(self, color: bool) -> Self {
        PulsatingCircle { color, ..self }
    }

    pub const fn with_filled(self, filled: bool) -> Self {
        PulsatingCircle { filled, ..self }
    }

    pub fn current_radius(&self) -> i32 {
        self.current_radius
    }

    fn create_circle(&self) -> Circle {
        Circle::new(self.center, self.current_radius, self.color, self.filled)
    }
}

impl Updatable for PulsatingCircle {
    fn update(&mut self) {
        if self.current_radius <= self.min_radius {
            self.direction = 1;
        } else if self.current_radius >= self.max_radius {
            self.direction = -1;
        }

        self.current_radius = self.current_radius + self.direction;
    }
}

impl Component for PulsatingCircle {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let circle = self.create_circle();
        circle.draw(renderer);
    }
}
