use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::logging::warn;
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::rand;
use macroquad::texture::{draw_texture, Image, Texture2D};

use once_cell::sync::OnceCell;

use crate::common::map;
use crate::entity::dynamic::follower::Follower;
use crate::entity::dynamic::random_mover::RandomMover;
use crate::entity::dynamic::updatable::Update;
use crate::entity::statich::road::Segment;
use crate::entity::statich::stone::Stone;
use crate::entity::statich::terrain::Terrain;
use crate::entity::statich::Static;
use crate::special::noise::Noise;

use crate::world::WorldCoordinate;
use crate::world::CHUNK_SIZE;
use crate::world::NOISE_IMAGE_SIZE;

pub struct Chunk {
    pub dynamics: Vec<Option<Box<dyn Update>>>,
    pub statics: Vec<Static>,

    noise_image: OnceCell<Image>,
    noise_texture: OnceCell<Texture2D>,
}

impl Chunk {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dynamics: Vec::new(),
            statics: Vec::new(),
            noise_image: OnceCell::new(),
            noise_texture: OnceCell::new(),
        }
    }

    pub fn init(&mut self, noise: &Noise) {
        let image = Noise::gen_image(NOISE_IMAGE_SIZE, noise.get());
        match self.noise_image.set(image) {
            Ok(_) => (),
            Err(_) => warn!("Tried to reinit chunk"),
        }
    }

    pub fn populate(&mut self, world_position: WorldCoordinate, noise: &Noise) {
        self.init(noise);
        let cell_scale = 50.0;
        let cell_size = (CHUNK_SIZE / f32::from(NOISE_IMAGE_SIZE)) * cell_scale;
        let cells = (CHUNK_SIZE / cell_size) as usize;
        debug_assert!(cells < std::u32::MAX.try_into().expect("u32 fits into usize"));
        let (xoff, yoff) = world_position.offsets(CHUNK_SIZE);
        println!(
            "xoff: {}, yoff: {}, cell_size: {}, cells: {}",
            xoff, yoff, cell_size, cells
        );
        for y in 0..cells {
            let pos_y = (y as f32).mul_add(cell_size, yoff);
            for x in 0..cells {
                let pos_x = (x as f32).mul_add(cell_size, xoff);

                let noise_x = x * cell_scale as usize;
                let noise_y = y * cell_scale as usize;
                let noise_value =
                    self.get_point(noise_x.try_into().unwrap(), noise_y.try_into().unwrap());

                self.populate_cell(pos_x, pos_y, cell_size, noise_value);

                self.statics.push(Static::Terrain(Terrain::new(
                    vec2(pos_x as f32, pos_y as f32),
                    noise_value,
                    cell_size as f32,
                )));
            }
        }
        self.statics.sort_unstable();
    }

    fn populate_cell(&mut self, x: f32, y: f32, cell_size: f32, noise_value: f32) {
        let max_stone_size = 80.0;
        let noise_value = noise_value as u8;
        match noise_value {
            0..=49 => (),
            50..=99 => {
                let stones = rand::gen_range(0, 1 + 1);
                for _ in 0..stones {
                    let pos_x = rand::gen_range(x, x + cell_size);
                    let pos_y = rand::gen_range(y, y + cell_size);
                    self.statics.push(Static::Stone(Stone::new(
                        vec2(pos_x as f32, pos_y as f32),
                        f32::from(noise_value) * 3.0,
                        rand::gen_range(5.0, max_stone_size / 3.0) as f32,
                    )));
                }
            }
            100..=199 => {
                let stones = rand::gen_range(0, 2 + 1);
                for _ in 0..stones {
                    let pos_x = rand::gen_range(x, x + cell_size);
                    let pos_y = rand::gen_range(y, y + cell_size);
                    self.statics.push(Static::Stone(Stone::new(
                        vec2(pos_x as f32, pos_y as f32),
                        f32::from(noise_value) * 3.0,
                        rand::gen_range(5.0, max_stone_size / 2.0) as f32,
                    )));
                }
                let random = rand::gen_range(0, 10);
                if random < 3 {
                    let pos_x = rand::gen_range(x, x + cell_size);
                    let pos_y = rand::gen_range(y, y + cell_size);
                    self.add_random_mover(
                        Vec2::new(pos_x, pos_y),
                        0.0,
                        rand::gen_range(5.0, 25.0),
                        rand::gen_range(0.1, 1.5),
                    );
                }
            }
            200..=255 => {
                let stones = rand::gen_range(0, 3 + 1);
                for _ in 0..stones {
                    let pos_x = rand::gen_range(x, x + cell_size);
                    let pos_y = rand::gen_range(y, y + cell_size);
                    self.statics.push(Static::Stone(Stone::new(
                        vec2(pos_x as f32, pos_y as f32),
                        f32::from(noise_value) * 3.0,
                        rand::gen_range(5.0, max_stone_size) as f32,
                    )));
                }
                let random = rand::gen_range(0, 10);
                if random < 3 {
                    let pos_x = rand::gen_range(x, x + cell_size);
                    let pos_y = rand::gen_range(y, y + cell_size);
                    self.add_follower(Vec2::new(pos_x, pos_y));
                }
            }
        }
    }

    pub fn get_point(&self, x: u32, y: u32) -> f32 {
        map(
            self.noise_image
                .get()
                .expect("Chunk must be initialised to get the point from the noise image")
                .get_pixel(x, y)
                .r,
            0.0,
            1.0,
            0.0,
            255.0,
        )
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

    pub fn add_follower(&mut self, position: Vec2) {
        self.dynamics.push(Some(Box::new(Follower::new(position))));
    }

    pub fn update(&mut self) {
        for item in 0..self.dynamics.len() {
            let mut dynamic = std::mem::replace(&mut self.dynamics[item], None);
            dynamic
                .as_mut()
                .expect("should get dynamic entity after mem::replace")
                .update(self);
            self.dynamics[item] = dynamic;
        }
    }

    pub fn draw(&self, viewport: Rect) {
        for static_entity in &self.statics {
            match static_entity {
                Static::Stone(stone) => stone.draw(viewport),
                Static::Road(segment) => segment.draw(viewport),
                Static::Terrain(terrain) => terrain.draw(viewport),
            }
        }
        for dynamic_entity in &self.dynamics {
            dynamic_entity
                .as_ref()
                .expect("every dynamic entity should be present in draw call")
                .draw(viewport);
        }
    }

    pub fn draw_noise_texture(&self, x: f32, y: f32) {
        let texture = self.noise_texture.get_or_init(|| {
            Texture2D::from_image(
                self.noise_image
                    .get()
                    .expect("noise_image should be initialized before drawing the noise texture"),
            )
        });
        draw_texture(*texture, x, y, color_u8!(255, 255, 255, 255));
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
