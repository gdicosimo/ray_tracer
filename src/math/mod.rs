use rand::distr::{Distribution, Uniform};

pub(crate) use self::{aabb::Aabb, algebra::*, color::*, interval::Interval, ray::Ray};

pub mod color;
pub mod pdf;

mod aabb;
mod algebra;
mod interval;
mod ray;

pub const EPSILON: f32 = 1e-6;
pub const NORMALIZATION_TOLERANCE: f32 = 1e-3;
pub const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    // degrees * PI / 180.0
    degrees.to_radians()
}

pub fn random_float() -> f32 {
    let between = Uniform::new(0.0, 1.0).unwrap();
    let mut rng = rand::rng();
    between.sample(&mut rng)
}

#[inline]
pub fn random_int_beetwen(min: f32, max: f32) -> usize {
    random_float_between(min, max + 1.0) as usize
}

#[inline]
pub fn random_float_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}
