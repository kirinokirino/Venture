use macroquad::color::DARKGRAY;
use macroquad::math::{vec2, Mat3, Rect, Vec2};
use macroquad::rand;
use macroquad::shapes::draw_poly;

use crate::entity::dynamic::updatable::Update;
use crate::special::chunk::Chunk;

pub struct RandomMover {
    position: Vec2,
    rotation: f32,
    size: f32,
    speed: f32,
}

impl RandomMover {
    #[must_use]
    pub const fn new(position: Vec2, rotation: f32, size: f32, speed: f32) -> Self {
        Self {
            position,
            rotation,
            size,
            speed,
        }
    }
}

impl Update for RandomMover {
    fn get_pos(&self) -> Vec2 {
        self.position
    }

    fn update(&mut self, _chunk: &mut Chunk) {
        self.rotation += rand::gen_range(-10., 10.);
        let delta = vec2(0.0, self.speed);
        let rotation_matrix = Mat3::from_rotation_z(self.rotation.to_radians());
        self.position += rotation_matrix.transform_vector2(delta);
    }

    fn draw(&self, viewport: Rect) {
        if viewport.contains(self.position) {
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
}
