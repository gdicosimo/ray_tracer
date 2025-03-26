use super::{Color, Point3, Texture};

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn from_color(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_color(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.albedo.clone()
    }
}
