use super::*;

mod three_d;
mod two_d;

pub use self::{three_d::*, two_d::*};

pub trait Primitive: Hittable {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32;

    fn random(&self, origin: &Point3) -> Vec3;
}

pub trait PlaneShape: Hittable {
    fn is_interior(&self, alpha: f32, beta: f32) -> bool;
}
