use std::default::Default;

use macroquad::input::{is_key_down, mouse_position, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::window::{screen_height, screen_width};

use crate::common::map;

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
        //self.followed_rot = rotation;
    }

    #[must_use]
    pub fn viewport_size(&self) -> (f32, f32) {
        (2.0 * (1.0 / self.zoom.x), 2.0 * (1.0 / self.zoom.y))
    }

    #[must_use]
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        let (half_width, half_height) = (screen_width() / 2.0, screen_height() / 2.0);
        // x, y offset from the center of the window.
        let (x, y) = ((point.x - half_width), (point.y - half_height));

        let (viewport_width, viewport_height) = self.viewport_size();
        let (half_viewport_width, half_viewport_height) =
            ((viewport_width / 2.0), (viewport_height / 2.0));

        let result_x = map(
            x,
            -half_width,
            half_width,
            -half_viewport_width,
            half_viewport_width,
        );
        let result_y = map(
            y,
            -half_height,
            half_height,
            -half_viewport_height,
            half_viewport_height,
        );
        vec2(result_x + self.target.x, result_y + self.target.y)
    }

    #[must_use]
    pub fn mouse_world_position(&self) -> Vec2 {
        let mouse = mouse_position();
        let mouse = vec2(mouse.0, mouse.1);
        self.screen_to_world(mouse)
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
        camera.target.y -= 0.01 / camera.zoom.x;
        camera.unfollow();
    }
    if is_key_down(KeyCode::O) {
        // && is_key_pressed(KeyCode::LeftControl) {
        camera.target.y += 0.01 / camera.zoom.x;
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
