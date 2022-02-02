use macroquad::input::{is_key_down, mouse_position, KeyCode};
use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub target: Vec2,
    pub zoom: Vec2,
}

impl Camera {
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

pub fn top_down_camera_controls(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        camera.target.y += 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::O) {
        camera.target.y -= 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::A) {
        camera.target.x -= 0.01 / camera.zoom.x;
    }
    if is_key_down(KeyCode::E) {
        camera.target.x += 0.01 / camera.zoom.x;
    }
    // zoom
    if is_key_down(KeyCode::PageUp) || is_key_down(KeyCode::Apostrophe) {
        camera.zoom.x *= 0.98;
        camera.zoom.y *= 0.98;
    }
    if is_key_down(KeyCode::PageDown) || is_key_down(KeyCode::Period) {
        camera.zoom.x /= 0.98;
        camera.zoom.y /= 0.98;
    }
}
