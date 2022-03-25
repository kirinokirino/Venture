use std::convert::From;
use std::default::Default;
use std::fmt::Display;

use macroquad::camera::set_default_camera;
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::{colors, Color};
use macroquad::color_u8;
use macroquad::input::{
    is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::logging::debug;
use macroquad::math::{vec2, Mat3, Vec2};
use macroquad::rand;
use macroquad::shapes::draw_rectangle_lines;
use macroquad::telemetry::log_string;
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_time};
use macroquad::window::{clear_background, screen_height, screen_width};

use indexmap::IndexMap;

use crate::entity::dynamic::updatable::Update;
use crate::special::camera::{top_down_camera_controls, Camera};
use crate::special::chunk::Chunk;
use crate::special::noise::Noise;
use crate::special::square::Square;

pub const CHUNK_SIZE: u16 = 16;
pub const CHUNK_TILE_SIZE: f32 = 400.0;
pub const NOISE_IMAGE_SIZE: u16 = 256;
pub const RENDER_DISTANCE: i32 = 2;
pub const UPDATE_DISTANCE: i32 = 5;

pub struct World {
    time: Time,

    seed: u64,
    noise_generators: Vec<Noise>,

    main_camera: Camera,
    player: Square,

    chunks: IndexMap<ChunkPosition, Chunk>,
    out_of_chunk: Vec<Option<Box<dyn Update>>>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: Time::default(),
            seed: 0,
            noise_generators: Vec::new(),

            main_camera: Camera::new(),
            player: Square::new(vec2(0.0, 0.0)),

            chunks: IndexMap::new(),
            out_of_chunk: Vec::new(),
        }
    }

    pub fn setup(&mut self) {
        let mut new_noise = Noise::new();
        new_noise.set_noise(self.seed, 0.005);
        self.noise_generators.push(new_noise);

        self.generate_chunks_around(ChunkPosition::from(self.player.center));
    }

    fn generate_chunks_around(&mut self, pos: ChunkPosition) {
        self.generate_chunk(pos.add(-1, -1));
        self.generate_chunk(pos.add(-1, 0));
        self.generate_chunk(pos.add(-1, 1));
        self.generate_chunk(pos.add(0, -1));
        self.generate_chunk(pos);
        self.generate_chunk(pos.add(0, 1));
        self.generate_chunk(pos.add(1, -1));
        self.generate_chunk(pos.add(1, 0));
        self.generate_chunk(pos.add(1, 1));
    }

    fn reset(&mut self) {
        self.chunks.clear();
        self.setup();
    }

    fn generate_chunk(&mut self, pos: ChunkPosition) {
        if !self.chunks.contains_key(&pos) {
            log_string(format!("Chunk spawn at {}", pos).as_str());
            let mut chunk = Chunk::new(pos);
            chunk.populate(
                self.noise_generators
                    .last()
                    .expect("World needs to have a noise generator to populate a chunk"),
            );
            self.chunks.insert(pos, chunk);
        }
    }

    pub fn input(&mut self) {
        let lmb = is_mouse_button_pressed(MouseButton::Left);
        let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let A = is_key_down(KeyCode::A);
        let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

        if is_key_down(KeyCode::Space) {
            self.seed = u64::from(rand::rand());
            self.reset();
        }

        if lmb {
            let camera = self.main_camera;
            debug!(
                "{}",
                format!(
                    "target: {}, zoom: {:?}, view_port: {:?}",
                    camera.target,
                    camera.zoom,
                    camera.viewport_size(),
                )
            );
            let mouse = camera.mouse_world_position();
            debug!(
                "mouse: {:?}, mouse_world: {}, mouse_chunk: {}",
                mouse_position(),
                mouse,
                ChunkPosition::from(mouse)
            );
        }

        let player_speed = -1.0;
        self.player.rotation += rand::gen_range(-1., 1.);
        let delta = vec2(0.0, player_speed);
        let rotation_matrix = Mat3::from_rotation_z(self.player.rotation.to_radians());
        self.player.center += rotation_matrix.transform_vector2(delta);

        self.generate_chunks_around(ChunkPosition::from(self.player.center));

        if is_key_down(KeyCode::LeftControl) {
            top_down_camera_controls(&mut self.main_camera);
        } else {
            let reversed = -1.0;
            let mut delta = vec2(0.0, 0.0);
            if W {
                delta.y += 1.0 * reversed;
            } else if S {
                delta.y -= 1.0 * reversed;
            }
            let mut rotation = 0.0;
            if A {
                rotation += 0.01 * reversed;
            } else if D {
                rotation -= 0.01 * reversed;
            }
            let last_chunk = ChunkPosition::from(self.player.center);
            self.player.rotation += rotation;
            let r = Mat3::from_rotation_z(self.player.rotation);
            self.player.center += r.transform_vector2(delta * player_speed);
            let chunk = ChunkPosition::from(self.player.center);
            if last_chunk != chunk {
                self.generate_chunks_around(ChunkPosition::from(self.player.center));
            }

            self.main_camera
                .set_follow(Some(self.player.center), Some(self.player.rotation));
        }
    }

    pub fn update(&mut self) {
        self.update_time(get_time());
        self.main_camera.update();

        let player_chunk = ChunkPosition::from(self.player.center);
        for (pos, chunk) in &mut self.chunks {
            if pos.is_within(player_chunk, UPDATE_DISTANCE) {
                chunk.update();
                self.out_of_chunk.extend(chunk.extract_outside_entities());
            }
        }

        for entity in self.out_of_chunk.drain(..) {
            let entity = unsafe {
                // SAFETY Extracted entities can't be None
                entity.unwrap_unchecked()
            };
            let new_chunk = ChunkPosition::from(entity.get_pos());
            if let Some(chunk) = self.chunks.get_mut(&new_chunk) {
                chunk.dynamics.push(Some(entity));
            }
        }
    }

    fn update_time(&mut self, time: f64) {
        self.time = Time {
            delta: time - self.time.overall,
            overall: get_time(),
        };
    }

    pub fn draw(&self) {
        clear_background(color_u8!(0, 0, 0, 255));
        // Camera space, render game objects
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });

        let mut viewport = self.main_camera.viewport_rect();
        viewport.w += CHUNK_TILE_SIZE * 2.0;
        viewport.h += CHUNK_TILE_SIZE * 2.0;
        viewport.move_to(vec2(
            viewport.x - CHUNK_TILE_SIZE,
            viewport.y - CHUNK_TILE_SIZE,
        ));
        let (width, height) = (screen_width(), screen_height());
        let (center_x, center_y) = (self.main_camera.target.x, self.main_camera.target.y);
        let top_left_x = center_x - width;
        let top_left_y = center_y - height;
        draw_rectangle_lines(
            top_left_x,
            top_left_y,
            width * 2.0,
            height * 2.0,
            50.0,
            color_u8!(50, 120, 100, 100),
        );

        let player_chunk = ChunkPosition::from(self.player.center);
        for (pos, chunk) in &self.chunks {
            if pos.is_within(player_chunk, RENDER_DISTANCE) {
                chunk.draw(viewport);
            }
        }

        self.player.draw();
        self.draw_ui();
    }

    fn draw_ui(&self) {
        // Screen space, render fixed ui
        set_default_camera();
        draw_text(
            &format!(
                "fps: {}, mouse: {:?}, chunk: {}",
                get_fps(),
                (
                    self.main_camera.mouse_world_position().x as i32,
                    self.main_camera.mouse_world_position().y as i32
                ),
                ChunkPosition::from(self.main_camera.mouse_world_position())
            ),
            10.0,
            20.0,
            30.0,
            colors::GRAY,
        );

        draw_text(
            &format!(
                "x:{:3.0} y:{:3.0}",
                self.player.center.x, self.player.center.y
            ),
            10.0,
            40.0,
            30.0,
            colors::GRAY,
        );

        let statics: usize = self
            .chunks
            .iter()
            .map(|(_, chunk)| chunk.statics.len())
            .sum();
        let dynamics: usize = self
            .chunks
            .iter()
            .map(|(_, chunk)| chunk.dynamics.len())
            .sum();
        draw_text(
            &format!(
                "static entities: {}, dynamic entities: {}",
                statics, dynamics
            ),
            10.0,
            60.0,
            30.0,
            colors::GRAY,
        );
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    delta: f64,
    overall: f64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChunkPosition {
    x: i32,
    y: i32,
}

impl ChunkPosition {
    #[must_use]
    pub fn offsets(&self, chunk_size: f32) -> (f32, f32) {
        (self.x as f32 * chunk_size, self.y as f32 * chunk_size)
    }

    #[must_use]
    pub const fn add(&self, delta_x: i32, delta_y: i32) -> Self {
        Self {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }

    #[must_use]
    pub const fn is_within(&self, other: Self, distance: i32) -> bool {
        if (self.y - other.y).abs() > distance {
            return false;
        }
        if (self.x - other.x).abs() > distance {
            return false;
        }
        true
    }
}

impl From<Vec2> for ChunkPosition {
    fn from(position: Vec2) -> Self {
        let chunk_size = f32::from(CHUNK_SIZE) * CHUNK_TILE_SIZE;
        let x = (position.x / chunk_size).floor();
        let y = (position.y / chunk_size).floor();
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}
