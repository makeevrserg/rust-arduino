use crate::renderer::{Canvas, Flushable, Point, Renderer};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    pixelcolor::BinaryColor,
    prelude::{Drawable, Primitive},
    primitives::{Circle, Line, PrimitiveStyle},
};

pub struct EmbeddedCanvas {
    width: i32,
    height: i32,
}

impl EmbeddedCanvas {
    pub fn new(width: i32, height: i32) -> Self {
        EmbeddedCanvas { width, height }
    }
}

impl Canvas for EmbeddedCanvas {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

pub struct EmbeddedGraphicsAdapter<'a, D> {
    display: &'a mut D,
    canvas: EmbeddedCanvas,
}

impl<'a, D> EmbeddedGraphicsAdapter<'a, D>
where
    D: DrawTarget<Color = BinaryColor>,
    D: OriginDimensions,
{
    pub fn new(display: &'a mut D) -> Self {
        let size = display.size();
        let canvas = EmbeddedCanvas::new(size.width as i32, size.height as i32);
        EmbeddedGraphicsAdapter { display, canvas }
    }

    fn to_eg_point(&self, point: Point) -> embedded_graphics::geometry::Point {
        embedded_graphics::geometry::Point::new(point.x, point.y)
    }

    fn to_eg_color(&self, color: bool) -> BinaryColor {
        if color {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

impl<'a, D> Renderer for EmbeddedGraphicsAdapter<'a, D>
where
    D: DrawTarget<Color = BinaryColor>,
    D: OriginDimensions,
    D: Flushable,
{
    fn draw_pixel(&mut self, point: Point, color: bool) {
        let _ = embedded_graphics::Pixel(
            self.to_eg_point(point),
            self.to_eg_color(color),
        )
        .draw(self.display);
    }

    fn draw_line(&mut self, p1: Point, p2: Point, color: bool) {
        let _ = Line::new(self.to_eg_point(p1), self.to_eg_point(p2))
            .into_styled(PrimitiveStyle::with_stroke(self.to_eg_color(color), 1))
            .draw(self.display);
    }

    fn draw_circle(&mut self, center: Point, radius: i32, color: bool, filled: bool) {
        let circle =
            Circle::new(self.to_eg_point(center), (radius * 2) as u32);

        if filled {
            let _ = circle
                .into_styled(PrimitiveStyle::with_fill(self.to_eg_color(color)))
                .draw(self.display);
        } else {
            let _ = circle
                .into_styled(PrimitiveStyle::with_stroke(self.to_eg_color(color), 1))
                .draw(self.display);
        }
    }

    fn clear(&mut self, color: bool) {
        let _ = self.display.clear(self.to_eg_color(color));
    }

    fn flush(&mut self) {
        let _ = self.display.flush();
    }

    fn canvas(&self) -> &dyn Canvas {
        &self.canvas
    }
}
