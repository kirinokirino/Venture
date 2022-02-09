use macroquad::color::DARKGRAY;
use macroquad::math::Vec2;
use macroquad::shapes::draw_poly;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Stone {
    position: Vec2,
    rotation: f32,
    size: f32,
}

impl Stone {
    #[must_use]
    pub const fn new(position: Vec2, rotation: f32, size: f32) -> Self {
        Self {
            position,
            rotation,
            size,
        }
    }

    pub fn draw(&self) {
        draw_poly(
            self.position.x,
            self.position.y,
            5,
            self.size,
            self.rotation,
            DARKGRAY,
        );
    }
}
