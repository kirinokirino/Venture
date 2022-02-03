use std::default::Default;

use macroquad::camera::set_default_camera;
use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::{colors, Color};
use macroquad::color_u8;
use macroquad::input::{
    is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::math::{vec2, Mat3};
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_time};
use macroquad::window::clear_background;

use crate::camera::{top_down_camera_controls, Camera};
use crate::objects::noise::Noise;
use crate::objects::square::Square;

pub struct World {
    time: Time,
    noise_generators: Vec<Noise>,

    main_camera: Camera,
    player: Square,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: Time::default(),
            noise_generators: Vec::new(),

            main_camera: Camera::new(),
            player: Square::new(vec2(100.0, 0.0)),
        }
    }

    pub fn setup(&mut self) {
        let mut new_noise = Noise::new();
        new_noise.set_noise(0, 0.01);
        self.noise_generators.push(new_noise);
    }

    pub fn input(&mut self) {
        let lmb = is_mouse_button_pressed(MouseButton::Left);
        let W = is_key_down(KeyCode::W) || is_key_down(KeyCode::Comma);
        let S = is_key_down(KeyCode::S) || is_key_down(KeyCode::O);
        let A = is_key_down(KeyCode::A);
        let D = is_key_down(KeyCode::D) || is_key_down(KeyCode::E);

        let mut delta = vec2(0.0, 0.0);
        if W {
            delta.y += 1.0;
        } else if S {
            delta.y -= 1.0;
        }
        let mut rotation = 0.0;
        if A {
            rotation += 0.01;
        } else if D {
            rotation -= 0.01;
        }
        self.player.rotation += rotation;
        let r = Mat3::from_rotation_z(self.player.rotation);
        self.player.center += r.transform_vector2(delta);
    }

    pub fn update(&mut self) {
        self.update_time(get_time());

        self.main_camera.followed_pos = Some(self.player.center);
        self.main_camera.followed_rot = Some(self.player.rotation);
        //top_down_camera_controls(&mut self.main_camera);
        self.main_camera.update();
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
            rotation: -self.main_camera.rotation.to_degrees(),
            zoom: self.main_camera.zoom,
            ..Camera2D::default()
        });

        self.noise_generators
            .last()
            .expect("No noise found, consider adding one in setup")
            .draw_at(0.0, 0.0);

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
