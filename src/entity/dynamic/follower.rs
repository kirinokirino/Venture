use macroquad::color::DARKGRAY;
use macroquad::math::{vec2, Mat3, Rect, Vec2};
use macroquad::rand;
use macroquad::shapes::draw_poly;

use crate::entity::dynamic::updatable::Update;
use crate::special::chunk::Chunk;

pub struct Follower {
    position: Vec2,
    target: Option<Vec2>,
}

impl Follower {
    #[must_use]
    pub const fn new(position: Vec2) -> Self {
        Self {
            position,
            target: None,
        }
    }
}

impl Update for Follower {
    fn get_pos(&self) -> Vec2 {
        self.position
    }

    fn update(&mut self, chunk: &mut Chunk) {
        let mut min_distance = f32::INFINITY;
        for entity in &chunk.dynamics {
            if let Some(entity) = entity.as_ref() {
                let pos = entity.get_pos();
                let distance = self.position.distance(pos);
                if distance < 400.0 && distance < min_distance && distance > 50.0 {
                    min_distance = distance;
                    self.target = Some(pos);
                }
            }
        }
        if min_distance > 400.0 {
            self.target = None;
        }

        if let Some(target) = self.target {
            self.position += (target - self.position).normalize() * 0.3;
        }
    }

    fn draw(&self, viewport: Rect) {
        if viewport.contains(self.position) {
            draw_poly(self.position.x, self.position.y, 10, 20.0, 0.0, DARKGRAY);
        }
    }
}
