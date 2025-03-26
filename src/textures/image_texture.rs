use super::*;

pub struct ImageTexture {
    image: Option<RtwImage>,
}

impl ImageTexture {
    pub fn from_image(filename: &str) -> Self {
        let image = RtwImage::try_new(filename);
        Self {
            image: if let Ok(image) = image {
                Some(image)
            } else {
                None
            },
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: &Point3) -> Color {
        let image = if let Some(image) = &self.image {
            image
        } else {
            return WHITE;
        };

        let u = f32::clamp(u, 0.0, 1.0);
        let v = 1.0 - f32::clamp(v, 0.0, 1.0);

        let i = (u * image.width() as f32) as u32;
        let j = (v * image.height() as f32) as u32;
        let pixel = image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            pixel[0] as f32 * color_scale,
            pixel[1] as f32 * color_scale,
            pixel[1] as f32 * color_scale,
        )
    }
}
