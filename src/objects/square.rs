use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;

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
