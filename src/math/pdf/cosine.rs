use super::*;

pub struct Cosine {
    uvw: Onb,
}

impl Cosine {
    pub fn new(n: UnitVec3) -> Self {
        Cosine {
            uvw: Onb::from_unit_vec3(n),
        }
    }
}

impl Pdf for Cosine {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine_theta = direction.unchecked_into_unit_vector().dot(self.uvw.w());
        f32::max(0.0, cosine_theta / PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}
