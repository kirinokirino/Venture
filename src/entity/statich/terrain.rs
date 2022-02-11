use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::draw_rectangle;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Terrain {
    position: Vec2,
    color: f32,
    size: f32,
}

impl Terrain {
    #[must_use]
    pub const fn new(position: Vec2, color: f32, size: f32) -> Self {
        Self {
            position,
            color,
            size,
        }
    }

    pub fn draw(&self, viewport: Rect) {
        if viewport.contains(self.position) {
            draw_rectangle(
                self.position.x,
                self.position.y,
                self.size,
                self.size,
                color_u8!(0.0, self.color, 50.0 + self.color / 2.0, 70),
            );
        }
    }
}
