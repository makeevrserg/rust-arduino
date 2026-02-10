use crate::component::square::Square;
use crate::component::Component;
use crate::renderer::{Point, Renderer, Updatable};

#[derive(Clone, Copy, Debug)]
pub struct RotatingSquare {
    center: Point,
    size: i32,
    current_angle: i32,
    rotation_step: i32,
    color: bool,
}

impl RotatingSquare {
    pub const fn new(center: Point, size: i32) -> Self {
        RotatingSquare {
            center,
            size,
            current_angle: 0,
            rotation_step: 45,
            color: true,
        }
    }

    pub const fn with_rotation_step(self, step: i32) -> Self {
        RotatingSquare {
            rotation_step: step,
            ..self
        }
    }

    pub const fn with_color(self, color: bool) -> Self {
        RotatingSquare { color, ..self }
    }

    pub fn current_angle(&self) -> i32 {
        self.current_angle
    }

    fn create_square(&self) -> Square {
        Square::new(self.center, self.size)
            .with_rotation(self.current_angle)
            .with_color(self.color)
    }
}

impl Updatable for RotatingSquare {
    fn update(&mut self) {
        // 6283 milli-radians ≈ 2π (one full rotation)
        self.current_angle = (self.current_angle + self.rotation_step) % 6283;
    }
}

impl Component for RotatingSquare {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let square = self.create_square();
        square.draw(renderer);
    }
}
