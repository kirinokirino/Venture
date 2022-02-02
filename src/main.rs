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

mod camera;
use camera::{top_down_camera_controls, Camera};
mod common;
mod objects;
use objects::noise::Noise;

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
        top_down_camera_controls(&mut main_camera);

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
