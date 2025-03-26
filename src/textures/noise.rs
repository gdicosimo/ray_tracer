use super::*;

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f32,
}

pub struct MarbleTexture(NoiseTexture, Color);
pub struct WoodTexture(NoiseTexture);
pub struct Melamine(NoiseTexture, Color);

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }

    fn smooth(&self, _u: f32, _v: f32, p: Point3) -> Color {
        let smooth_noise = self.perlin.smooth_noise(&(self.scale * p));

        (smooth_noise.clamp(-1.0, 1.0) * 0.5 + 0.5) * WHITE
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.smooth(u, v, p.clone())
    }
}

impl WoodTexture {
    pub fn new(scale: f32) -> Self {
        Self(NoiseTexture::new(scale))
    }
}

impl Melamine {
    pub fn new(albedo: Color, scale: f32) -> Self {
        Self(NoiseTexture::new(scale), albedo)
    }
}

impl Texture for Melamine {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        let noise_value = self.0.perlin.smooth_noise(&(self.0.scale * p));
        let t = 0.5 * (1.0 + noise_value);

        Color::new(0.1, 0.1, 0.1).lerp(&self.1, t)
    }
}
impl Texture for WoodTexture {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        let radial = (p.x().powi(2) + p.z().powi(2)).sqrt() * self.0.scale;
        let noise_value = f32::sin(radial * 10.0 + 5.0 * self.0.perlin.turbulence(p.clone(), 3));

        let light_brown = Color::new(0.6, 0.4, 0.2);
        let dark_brown = Color::new(0.0, 0.0, 0.0);
        light_brown.lerp(&dark_brown, noise_value)
    }
}

impl MarbleTexture {
    pub fn new(scale: f32, color: Color) -> Self {
        Self(NoiseTexture::new(scale), color)
    }
}

impl Texture for MarbleTexture {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        (1.0 + f32::sin(self.0.scale * p.z() + 10.0 * self.0.perlin.turbulence(p.clone(), 7)))
            * &self.1
    }
}
