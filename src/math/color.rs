use super::{Dimensional, Interval, Vec3};

pub type Color = Vec3;

pub const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const WHITE: Vec3 = Vec3::new(0.99, 0.99, 0.99);
pub const GRAY: Vec3 = Vec3::new(0.67, 0.7, 0.73);
pub const SKY_BLUE: Vec3 = Vec3::new(0.5, 0.7, 1.0);

const RGB_RANGE: f32 = 256.0;
static GAMMA_INTERVAL: Interval = Interval::new(0.0, 0.999);

pub fn color_to_bytes(pixel_color: Color) -> [u8; 3] {
    let to_byte = |component: f32| {
        let gamma_corrected = if component > 0.0 {
            component.powf(1.0 / 2.2)
            // component.sqrt()
        } else {
            0.0
        };
        let clamped = GAMMA_INTERVAL.clamp(gamma_corrected);

        (clamped * RGB_RANGE) as u8
    };
    [
        to_byte(pixel_color.x()),
        to_byte(pixel_color.y()),
        to_byte(pixel_color.z()),
    ]
}

impl Color {
    pub fn mul(&self, other: &Color) -> Color {
        Color::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color::new(
            self.x() + t * (other.x() - self.x()),
            self.y() + t * (other.y() - self.y()),
            self.z() + t * (other.z() - self.z()),
        )
    }
}
