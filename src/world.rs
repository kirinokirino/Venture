use std::default::Default;

use macroquad::camera::set_default_camera;
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::{colors, Color};
use macroquad::color_u8;
use macroquad::input::{
    is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::math::vec2;
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_time};
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::camera::{top_down_camera_controls, Camera};
use crate::objects::noise::Noise;

pub struct World {
    time: Time,
    noise_generators: Vec<Noise>,

    main_camera: Camera,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: Time::default(),
            noise_generators: Vec::new(),

            main_camera: Camera::new(),
        }
    }

    pub fn setup(&mut self) {}

    pub fn update(&mut self) {
        self.update_time(get_time());

        top_down_camera_controls(&mut self.main_camera);
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
        set_camera(&Camera2D {
            target: self.main_camera.target,
            rotation: self.main_camera.rotation,
            zoom: self.main_camera.zoom,
            ..Camera2D::default()
        });

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
