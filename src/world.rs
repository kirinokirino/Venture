use std::default::Default;

use macroquad::camera::set_default_camera;
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::{colors, Color};
use macroquad::color_u8;
use macroquad::input::{
    is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::math::{vec2, Mat3};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_time};
use macroquad::window::{clear_background, screen_height, screen_width};

use indexmap::IndexMap;

use crate::camera::{top_down_camera_controls, Camera};
use crate::special::chunk::Chunk;
use crate::special::noise::Noise;
use crate::special::square::Square;

pub const CHUNK_SIZE: f32 = 20_000.0;

pub struct World {
    time: Time,
    noise_generators: Vec<Noise>,

    main_camera: Camera,
    player: Square,

    chunks: IndexMap<WorldCoordinate, Chunk>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: Time::default(),
            noise_generators: Vec::new(),

            main_camera: Camera::new(),
            player: Square::new(vec2(100.0, 0.0)),

            chunks: IndexMap::new(),
        }
    }

    pub fn setup(&mut self) {
        let mut new_noise = Noise::new(2000);
        new_noise.set_noise(0, 0.001);
        self.noise_generators.push(new_noise);

        let mut chunk = Chunk::new();
        chunk.add_stone(vec2(0.0, 0.0), 0.0, 20.0);
        chunk.add_stone(vec2(100.0, 60.0), 40.0, 60.0);
        chunk.add_stone(vec2(300.0, 100.0), 120.0, 34.0);
        chunk.add_stone(vec2(180.0, 250.0), 160.0, 54.0);
        chunk.add_road_segment(vec2(500.0, 60.0), 0.0, 40.0);
        chunk.add_road_segment(vec2(500.0, 100.0), -90.0, 40.0);
        chunk.add_road_segment(vec2(540.0, 100.0), 180.0, 100.0);

        chunk.add_random_mover(vec2(540.0, 100.0), 180.0, 100.0, 5.0);
        chunk.add_random_mover(vec2(300.0, 300.0), 0.0, 15.0, 1.0);
        chunk.add_random_mover(vec2(300.0, 300.0), 0.0, 15.0, -1.0);
        let world_center = WorldCoordinate { x: 0, y: 0 };
        self.chunks.insert(world_center, chunk);
    }

    pub fn input(&mut self) {
        let _lmb = is_mouse_button_pressed(MouseButton::Left);
        let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let A = is_key_down(KeyCode::A);
        let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

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
            self.player.rotation += rotation;
            let r = Mat3::from_rotation_z(self.player.rotation);
            self.player.center += r.transform_vector2(delta);

            self.main_camera
                .set_follow(Some(self.player.center), Some(self.player.rotation));
        }
    }

    pub fn update(&mut self) {
        self.update_time(get_time());
        self.main_camera.update();
        for (pos, chunk) in &mut self.chunks {
            chunk.update();
        }
    }

    fn update_time(&mut self, time: f64) {
        self.time = Time {
            delta: time - self.time.overall,
            overall: get_time(),
        };
    }

    pub fn draw(&self) {
        clear_background(color_u8!(255, 255, 255, 255));
        // Camera space, render game objects
        let zoom = vec2(self.main_camera.zoom.x, -self.main_camera.zoom.y);
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom,
            ..Camera2D::default()
        });

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

        let grid_height = 10;
        let grid_width = 10;
        let grid_spacing = height * 2.0 / grid_height as f32;

        let noise = self
            .noise_generators
            .get(0)
            .expect("Should have a noise initialised in setup");
        for grid_y in 0..grid_height {
            let y = (grid_y as f32).mul_add(grid_spacing, top_left_y);
            for grid_x in 0..grid_width {
                let x = (grid_x as f32).mul_add(grid_spacing, top_left_x);
                let noise_value =
                    noise.get_point((x.abs() / 100.0) as u32, (y.abs() / 100.0) as u32);
                draw_rectangle(
                    x,
                    y,
                    grid_spacing,
                    grid_spacing,
                    color_u8!(
                        120,
                        noise_value * 255.0,
                        noise_value * 255.0,
                        noise_value * 255.0
                    ),
                );
            }
        }

        for (pos, chunk) in &self.chunks {
            chunk.draw();
        }

        self.player.draw();
        self.draw_ui();
    }

    fn draw_ui(&self) {
        // Screen space, render fixed ui
        set_default_camera();
        draw_text(
            &format!("mouse: {:?}, fps: {}", mouse_position(), get_fps()),
            10.0,
            20.0,
            30.0,
            colors::BLACK,
        );
        let noise = self
            .noise_generators
            .get(0)
            .expect("Should have a noise after setup");
        let noise_value = noise.get_point(
            (self.player.center.x / 100.0).abs() as u32,
            (self.player.center.y / 100.0).abs() as u32,
        );
        draw_text(
            &format!(
                "x:{:3.0} y:{:3.0}, biome: {}",
                self.player.center.x, self.player.center.y, noise_value
            ),
            10.0,
            40.0,
            30.0,
            colors::BLACK,
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
struct WorldCoordinate {
    x: i32,
    y: i32,
}
