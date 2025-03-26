use super::*;

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
    inv_scale: f32,
}

impl CheckerTexture {
    pub fn from_textures(even: Arc<dyn Texture>, odd: Arc<dyn Texture>, scale: f32) -> Self {
        Self {
            even,
            odd,
            inv_scale: 1.0 / scale,
        }
    }

    pub fn from_colors(even: Color, odd: Color, scale: f32) -> Self {
        Self::from_textures(
            Arc::new(SolidColor::from_color(even)),
            Arc::new(SolidColor::from_color(odd)),
            scale,
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let x = f32::floor(self.inv_scale * p.x()) as i32;
        let y = f32::floor(self.inv_scale * p.y()) as i32;
        let z = f32::floor(self.inv_scale * p.z()) as i32;

        if ((x + y + z) % 2) == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
