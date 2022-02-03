use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::math::{vec2, Mat3, Vec2};
use macroquad::shapes::draw_line;

pub struct Square {
    pub center: Vec2,
    pub rotation: f32,
    pub size: f32,
}

impl Square {
    #[must_use]
    pub const fn new(center: Vec2) -> Self {
        Self {
            center,
            rotation: 0.0,
            size: 25.0,
        }
    }

    #[must_use]
    fn corners(size: f32) -> [Vec2; 4] {
        let half_size = size / 2.0;
        let (x, y) = (0.0, 0.0);
        [
            Vec2::new(x - half_size, y - half_size),
            Vec2::new(x + half_size, y - half_size),
            Vec2::new(x + half_size, y + half_size),
            Vec2::new(x - half_size, y + half_size),
        ]
    }

    #[must_use]
    fn rotate(p: [Vec2; 4], rotation: f32) -> [Vec2; 4] {
        let r = Mat3::from_rotation_z(rotation);
        [
            r.transform_point2(p[0]),
            r.transform_point2(p[1]),
            r.transform_point2(p[2]),
            r.transform_point2(p[3]),
        ]
    }

    pub fn draw(&self) {
        let corners = Self::rotate(Self::corners(self.size), self.rotation);
        let thickness = 5.0;
        let color = color_u8!(155, 155, 155, 155);
        let rot_matrix = Mat3::from_rotation_z(self.rotation);

        let rot_point = rot_matrix.transform_vector2(vec2(0.0, self.size / 2.0));
        draw_line(
            self.center.x,
            self.center.y,
            self.center.x + rot_point.x,
            self.center.y + rot_point.y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[0].x,
            self.center.y + corners[0].y,
            self.center.x + corners[1].x,
            self.center.y + corners[1].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[1].x,
            self.center.y + corners[1].y,
            self.center.x + corners[2].x,
            self.center.y + corners[2].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[2].x,
            self.center.y + corners[2].y,
            self.center.x + corners[3].x,
            self.center.y + corners[3].y,
            thickness,
            color,
        );
        draw_line(
            self.center.x + corners[3].x,
            self.center.y + corners[3].y,
            self.center.x + corners[0].x,
            self.center.y + corners[0].y,
            thickness,
            color,
        );
    }
}
