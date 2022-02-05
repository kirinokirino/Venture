use std::default::Default;

use macroquad::input::{is_key_down, is_key_pressed, mouse_position, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::window::{screen_height, screen_width};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub target: Vec2,
    pub rotation: f32,
    pub zoom: Vec2,
    pub followed_pos: Option<Vec2>,
    pub followed_rot: Option<f32>,
}

impl Camera {
    #[must_use]
    pub fn new() -> Self {
        let starting_zoom = 1.0 / screen_width();
        Self {
            target: vec2(0.0, 0.0),
            rotation: 0.0,
            zoom: vec2(
                starting_zoom,
                starting_zoom * screen_width() / screen_height(),
            ),
            followed_pos: None,
            followed_rot: None,
        }
    }

    pub fn update(&mut self) {
        if let Some(target) = self.followed_pos {
            self.target = target;
        }
        if let Some(rot) = self.followed_rot {
            self.rotation = rot;
        }
    }

    pub fn unfollow(&mut self) {
        self.followed_pos = None;
        self.followed_rot = None;
    }

    pub fn set_follow(&mut self, position: Option<Vec2>, rotation: Option<f32>) {
        self.followed_pos = position;
        self.followed_rot = rotation;
    }

    #[must_use]
    pub fn mouse_position(&self) -> Vec2 {
        let mouse = mouse_position();
        Vec2::new(
            ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / self.zoom.x)
                + self.target.x,
            ((-mouse.1 + screen_height() / 2.0)
                / (screen_height() / 2.0)
                / self.zoom.x
                / (screen_width() / screen_height()))
                + self.target.y,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

pub fn top_down_camera_controls(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        // && is_key_pressed(KeyCode::LeftControl) {
        camera.target.y += 0.01 / camera.zoom.x;
        camera.unfollow();
    }
    if is_key_down(KeyCode::O) {
        // && is_key_pressed(KeyCode::LeftControl) {
        camera.target.y -= 0.01 / camera.zoom.x;
        camera.unfollow();
    }
    if is_key_down(KeyCode::A) {
        // && is_key_pressed(KeyCode::LeftControl) {
        camera.target.x -= 0.01 / camera.zoom.x;
        camera.unfollow();
    }
    if is_key_down(KeyCode::E) {
        // && is_key_pressed(KeyCode::LeftControl) {
        camera.target.x += 0.01 / camera.zoom.x;
        camera.unfollow();
    }
    // zoom
    if is_key_down(KeyCode::PageUp) || is_key_down(KeyCode::Apostrophe) {
        camera.zoom.x *= 0.98;
        camera.zoom.y *= 0.98;
        camera.unfollow();
    }
    if is_key_down(KeyCode::PageDown) || is_key_down(KeyCode::Period) {
        camera.zoom.x /= 0.98;
        camera.zoom.y /= 0.98;
        camera.unfollow();
    }
}
