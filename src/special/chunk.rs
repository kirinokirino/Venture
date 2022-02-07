use std::default::Default;

use macroquad::color::colors;
use macroquad::math::{vec2, Vec2};
use macroquad::text::draw_text;

use crate::entity::dynamic::random_mover::RandomMover;
use crate::entity::dynamic::updatable::Update;
use crate::entity::statich::road::Segment;
use crate::entity::statich::stone::Stone;
use crate::entity::statich::Static;
use crate::world::CHUNK_SIZE;

pub struct Chunk {
    dynamics: Vec<Option<Box<dyn Update>>>,
    statics: Vec<Static>,
}

impl Chunk {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dynamics: Vec::new(),
            statics: Vec::new(),
        }
    }

    pub fn add_stone(&mut self, position: Vec2, rotation: f32, size: f32) {
        self.statics
            .push(Static::Stone(Stone::new(position, rotation, size)));
    }

    pub fn add_road_segment(&mut self, position: Vec2, rotation: f32, size: f32) {
        self.statics
            .push(Static::Road(Segment::new(position, rotation, size)));
    }

    pub fn add_random_mover(&mut self, position: Vec2, rotation: f32, size: f32, speed: f32) {
        self.dynamics.push(Some(Box::new(RandomMover::new(
            position, rotation, size, speed,
        ))));
    }

    pub fn update(&mut self) {
        for item in 0..self.dynamics.len() {
            let mut dynamic = std::mem::replace(self.dynamics.get_mut(item).unwrap(), None);
            dynamic.as_mut().unwrap().update(self);
            self.dynamics[item] = dynamic;
        }
    }

    pub fn draw(&self) {
        for static_entity in &self.statics {
            match static_entity {
                Static::Stone(stone) => stone.draw(),
                Static::Road(seg) => seg.draw(),
            }
        }
        for dynamic_entity in &self.dynamics {
            dynamic_entity.as_ref().unwrap().draw();
        }
    }
}

/*
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
*/
