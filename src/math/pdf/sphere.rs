use super::*;

#[derive(Default)]
pub struct Sphere();

impl Sphere {
    pub fn new() -> Self {
        Self()
    }
}

impl Pdf for Sphere {
    fn value(&self, _direction: Vec3) -> f32 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        UnitVec3::unchecked_random().into_vec3()
    }
}
