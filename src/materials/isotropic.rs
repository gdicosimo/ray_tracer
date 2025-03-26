use super::*;

pub struct Isotropic {
    text: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            text: Arc::new(SolidColor::from_color(albedo)),
        }
    }

    pub fn from_texture(text: Arc<dyn Texture>) -> Self {
        Self { text }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.text.value(rec.u(), rec.v(), rec.point());
        let pdf = pdf::Sphere::new();

        Some(ScatterRecord::new(attenuation, Box::new(pdf)))
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        1.0 / (4.0 * PI)
    }
}
