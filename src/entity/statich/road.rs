use macroquad::color::DARKGRAY;
use macroquad::math::{Mat3, Vec2};
use macroquad::shapes::draw_line;

#[derive(Debug)]
pub struct Segment {
    position: Vec2,
    rotation: f32,
    size: f32,
}

impl Segment {
    #[must_use]
    pub const fn new(position: Vec2, rotation: f32, size: f32) -> Self {
        Self {
            position,
            rotation,
            size,
        }
    }

    fn end(&self) -> Vec2 {
        let rotation_matrix = Mat3::from_rotation_z(self.rotation.to_radians());
        rotation_matrix.transform_point2(Vec2::new(0.0, self.size))
    }

    pub fn draw(&self) {
        let end = self.position + self.end();
        draw_line(
            self.position.x,
            self.position.y,
            end.x,
            end.y,
            self.size / 5.0,
            DARKGRAY,
        );
    }
}
