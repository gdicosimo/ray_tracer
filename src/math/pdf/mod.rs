use crate::{geometry, math::*};

pub use self::{cosine::Cosine, mixture::Mixture, primitive::Primitive, sphere::Sphere};

mod cosine;
mod mixture;
mod primitive;
mod sphere;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f32;
    fn generate(&self) -> Vec3;
}
