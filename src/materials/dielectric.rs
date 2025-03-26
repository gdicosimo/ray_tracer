use super::*;

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub const fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = UnitVec3::unchecked_from_vec3(r_in.direction());
        let cos_theta = (-unit_direction.dot(rec.normal())).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_float()
        {
            unit_direction.as_vec3().reflect(rec.normal())
        } else {
            unit_direction.as_vec3().refract(rec.normal(), ri)
        };

        let scattered = Ray::new(rec.point().clone(), direction).set_time(r_in.time());
        Some(ScatterRecord::new_specular(WHITE, scattered))
    }
}
