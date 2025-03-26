use super::*;

pub struct Lambertian {
    texture: Arc<dyn Texture>,
    scatter_prob: f32,
}

impl Lambertian {
    pub fn from_color(albedo: Color, scatter_prob: f32) -> Self {
        Self {
            texture: Arc::new(SolidColor::from_color(albedo)),
            scatter_prob: scatter_prob.clamp(0.001, 1.0),
        }
    }
    pub fn from_texture(texture: Arc<dyn Texture>, scatter_prob: f32) -> Self {
        Self {
            texture,
            scatter_prob: scatter_prob.clamp(0.001, 1.0),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_roll = random_float();

        if scatter_roll > self.scatter_prob {
            return None;
        }

        let attenuation = self.texture.value(rec.u(), rec.v(), rec.point());
        let pdf = pdf::Cosine::new(rec.normal().clone());

        let srec = ScatterRecord::new(attenuation, Box::new(pdf));

        Some(srec)
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cos_theta = rec
            .normal()
            .dot(&scattered.direction().unchecked_normalize());

        if cos_theta < 0.0 || cos_theta.is_nan() {
            return 0.0;
        }

        cos_theta / PI
    }
}
