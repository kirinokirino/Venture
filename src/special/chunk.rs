use std::default::Default;

use macroquad::color::colors;
use macroquad::math::{vec2, Vec2};
use macroquad::text::draw_text;

use crate::world::CHUNK_SIZE;

pub struct Chunk {
    stuff: Vec<(Vec2, String)>,
}

impl Chunk {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {}

    pub fn draw(&self) {
        for (pos, text) in &self.stuff {
            draw_text(text, pos.x, pos.y, 30.0, colors::BLACK);
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        let top_left = (vec2(0.0, 0.0), "0, 0".to_owned());
        let btm_right = (
            vec2(CHUNK_SIZE, CHUNK_SIZE),
            format!("{}, {}", CHUNK_SIZE, CHUNK_SIZE),
        );

        let center = (
            vec2(CHUNK_SIZE / 2.0, CHUNK_SIZE / 2.0),
            format!("{}, {}", CHUNK_SIZE / 2.0, CHUNK_SIZE / 2.0),
        );

        let top_right = (vec2(CHUNK_SIZE, 0.0), format!("{}, {}", CHUNK_SIZE, 0.0));

        let btm_left = (vec2(0.0, CHUNK_SIZE), format!("{}, {}", 0.0, CHUNK_SIZE));

        let stuff = vec![top_left, btm_right, center, top_right, btm_left];
        Self { stuff }
    }
}
