use macroquad::math::Vec2;

use crate::special::chunk::Chunk;

pub trait Update {
    fn get_pos(&self) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    fn update(&mut self, _chunk: &mut Chunk) {}
    fn draw(&self) {}
}
