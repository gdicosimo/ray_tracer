use super::*;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = r_in.direction().reflect(rec.normal());
        reflected =
            UnitVec3::unchecked_from_vec3(&reflected) + (self.fuzz * UnitVec3::unchecked_random());

        let scattered = Ray::new(rec.point().clone(), reflected).set_time(r_in.time());
        let attenuation = self.albedo.clone();

        if scattered.direction().dot(rec.normal()) <= 0.0 {
            return None;
        }

        Some(ScatterRecord::new_specular(attenuation, scattered))
    }
}
