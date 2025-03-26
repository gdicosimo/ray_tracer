mod composite;
mod hit_record;
mod primitive;

pub(crate) use {
    crate::{
        materials::{Isotropic, Material},
        math::*,
        textures::Texture,
    },
    std::sync::Arc,
};

pub use {composite::*, hit_record::*, primitive::*};

pub trait Hittable: Send + Sync + std::any::Any {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
}
