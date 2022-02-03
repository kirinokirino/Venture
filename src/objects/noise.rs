use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::texture::{draw_texture, Image, Texture2D};

use once_cell::sync::OnceCell;
use simple_simplex::NoiseConfig;

pub struct Noise {
    noise: OnceCell<NoiseConfig>,
    image: OnceCell<Image>,
    texture: OnceCell<Texture2D>,
}

impl Noise {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            noise: OnceCell::new(),
            image: OnceCell::new(),
            texture: OnceCell::new(),
        }
    }

    pub fn set_noise(&mut self, seed: u64, frequency: f32) {
        let new_noise = Self::gen_noise(seed, frequency);
        if let Err(new_noise) = self.noise.set(new_noise) {
            *self
                .noise
                .get_mut()
                .expect("Noise config cell should be full.") = new_noise;
        }
    }

    pub fn get_point(&self, x: u32, y: u32) -> f32 {
        self.image
            .get_or_init(|| {
                Self::gen_image(250, self.noise.get_or_init(|| Self::gen_noise(0, 0.01)))
            })
            .get_pixel(x, y)
            .r
    }

    fn gen_noise(seed: u64, frequency: f32) -> NoiseConfig {
        NoiseConfig::new(
            4,            // Octaves
            frequency,    // X-Frequency
            frequency,    // Y-Frequency
            0.05,         // Amplitude
            3.0,          // Lacunarity
            0.25,         // Gain
            (0.0, 255.0), // range
            seed,         // seed
        )
    }

    #[must_use]
    pub fn gen_image(size: u16, noise: &NoiseConfig) -> Image {
        let mut image = Image::gen_image_color(size, size, color_u8!(255, 0, 255, 255));

        for y in 0..size {
            for x in 0..size {
                let color: u8 = noise.generate_range(x.into(), y.into()) as u8;
                let color = color_u8!(color, color, color, 255);
                image.set_pixel(u32::from(x), u32::from(y), color);
            }
        }
        image
    }
    pub fn draw_at(&self, x: f32, y: f32) {
        draw_texture(
            *self.texture.get_or_init(|| {
                Texture2D::from_image(self.image.get_or_init(|| {
                    Self::gen_image(250, self.noise.get_or_init(|| Self::gen_noise(0, 0.01)))
                }))
            }),
            x,
            y,
            color_u8!(255, 255, 255, 255),
        );
    }
}
