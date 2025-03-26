use std::sync::Arc;

use crate::{
    geometry::HitRecord,
    math::*,
    textures::{SolidColor, Texture},
};

pub use {
    dielectric::*, diffuse_light::*, isotropic::*, lambertian::*, metal::*, scatter_record::*,
};

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;
mod scatter_record;

pub struct Empty;

impl Material for Empty {}

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(&self, _hit_record: &HitRecord) -> Color {
        BLACK
    }
}
