use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::texture::Image;

use once_cell::sync::OnceCell;
use simple_simplex::NoiseConfig;

pub struct Noise {
    noise: OnceCell<NoiseConfig>,
}

impl Noise {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            noise: OnceCell::new(),
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

    pub fn get(&self) -> &NoiseConfig {
        self.noise.get().expect("tried to get uninitialized noise")
    }

    pub fn get_point(&self, x: f32, y: f32) -> f32 {
        let noise = self.get();
        noise.generate_range(x, y)
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
    pub fn gen_image(size: u16, xoff: f32, yoff: f32, noise: &NoiseConfig) -> Image {
        let mut image = Image::gen_image_color(size, size, color_u8!(255, 0, 255, 255));

        for y in 0..size {
            for x in 0..size {
                let color: u8 =
                    noise.generate_range(xoff + f32::from(x), yoff + f32::from(y)) as u8;
                let color = color_u8!(color, color, color, 255);
                image.set_pixel(u32::from(x), u32::from(y), color);
            }
        }
        image
    }
}
