#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::multiple_inherent_impl,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads
)]
#![allow(clippy::cast_precision_loss)]

use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    color::{colors, Color},
    color_u8,
    input::{is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton},
    logging::{debug, error, info, warn},
    math::{clamp, vec2, Vec2},
    rand,
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    texture::{draw_texture, Image, Texture2D},
    time::{get_fps, get_time},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use simple_simplex::NoiseConfig;

use once_cell::sync::OnceCell;

const NOISE_SIZE: u16 = 1000;
static mut NOISE: Noise = Noise::new();

pub fn setup() {}

pub fn draw(_delta: f64) {
    let lmb = is_mouse_button_pressed(MouseButton::Left);
    /* unsafe {
        for (i, noise) in NOISES.iter().enumerate() {
            noise.draw_at(
                0.0 + f32::from(NOISE_SIZE * i as u16),
                0.0, // + f32::from(NOISE_SIZE * i as u16),
            );
        }
    } */
    unsafe {}

    if lmb {}

    draw_ui();
}

fn draw_ui() {
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

pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
    from.mul_add(1.0 - p, to * p)
}

pub fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn norm(value: f32, start: f32, stop: f32) -> f32 {
    map(value, start, stop, 0.0, 1.0)
}

struct Square {
    center: Vec2,
    size: f32,
    rotation: f32,
}

impl Square {
    pub const fn new(center: Vec2) -> Self {
        Self {
            center,
            size: 25.0,
            rotation: 0.0,
        }
    }
    pub const fn rotated(self, rotation: f32) -> Self {
        Self {
            center: self.center,
            size: self.size,
            rotation,
        }
    }
    pub const fn sized(self, size: f32) -> Self {
        Self {
            center: self.center,
            size,
            rotation: self.rotation,
        }
    }

    pub fn corners(&self) -> [Vec2; 4] {
        let half_size = self.size / 2.0;
        let (x, y) = (self.center.x, self.center.y);

        [
            Vec2::new(x - half_size, y - half_size),
            Vec2::new(x + half_size, y - half_size),
            Vec2::new(x + half_size, y + half_size),
            Vec2::new(x - half_size, y + half_size),
        ]
    }

    pub fn draw(&self) {
        let corners = self.corners();
        let thickness = 5.0;
        let color = color_u8!(155, 155, 155, 155);
        draw_line(
            corners[0].x,
            corners[0].y,
            corners[1].x,
            corners[1].y,
            thickness,
            color,
        );
        draw_line(
            corners[1].x,
            corners[1].y,
            corners[2].x,
            corners[2].y,
            thickness,
            color,
        );
        draw_line(
            corners[2].x,
            corners[2].y,
            corners[3].x,
            corners[3].y,
            thickness,
            color,
        );
        draw_line(
            corners[3].x,
            corners[3].y,
            corners[0].x,
            corners[0].y,
            thickness,
            color,
        );
    }
}

struct Noise {
    image: OnceCell<Image>,
    texture: OnceCell<Texture2D>,
}

impl Noise {
    pub const fn new() -> Self {
        Self {
            image: OnceCell::new(),
            texture: OnceCell::new(),
        }
    }

    pub fn get_point(&self, x: u32, y: u32) -> f32 {
        self.image.get_or_init(Self::gen_image).get_pixel(x, y).r
    }

    pub fn gen_image() -> Image {
        let simplex: NoiseConfig = NoiseConfig::new(
            4,                   // Octaves
            0.01,                // X-Frequency
            0.01,                // Y-Frequency
            0.05,                // Amplitude
            3.0,                 // Lacunarity
            0.25,                // Gain
            (0.0, 255.0),        // range
            rand::rand().into(), // seed
        );

        let mut image = Image::gen_image_color(NOISE_SIZE, NOISE_SIZE, color_u8!(255, 0, 255, 255));

        for y in 0..NOISE_SIZE {
            for x in 0..NOISE_SIZE {
                let color: u8 = simplex.generate_range(x.into(), y.into()) as u8;
                let color = color_u8!(color, color, color, 255);
                image.set_pixel(u32::from(x), u32::from(y), color);
            }
        }
        image
    }
    pub fn draw_at(&self, x: f32, y: f32) {
        draw_texture(
            *self
                .texture
                .get_or_init(|| Texture2D::from_image(self.image.get_or_init(Self::gen_image))),
            x,
            y,
            color_u8!(255, 255, 255, 255),
        );
    }
}

#[allow(clippy::future_not_send, clippy::too_many_lines)]
#[macroquad::main("Name")]
async fn main() {
    let starting_zoom = 1.0 / screen_width();
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    loop {
        move_camera(&mut main_camera);

        // Camera space, render game objects
        set_camera(&Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Camera2D::default()
        });

        next_frame().await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Time {
    elapsed_seconds: f64,
    overall_time: f64,
}

/*
let delta = get_time() - self.time.overall_time;
self.time = Time {
    elapsed_seconds: delta,
    overall_time: get_time(),
};
*/

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    target: Vec2,
    zoom: Vec2,
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

fn move_camera(camera: &mut Camera) {
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
