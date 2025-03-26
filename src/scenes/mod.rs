use std::sync::Arc;

use crate::{
    camera::{Camera, CameraBuilder},
    geometry::*,
    materials::*,
    math::{color, Color, Point3, Vec3},
    textures::*,
};

pub use {cornell_box::CornellBox, earth::Earth, office::Svrnc, rtiow::Rtiow};

mod cornell_box;
mod earth;
mod office;
mod rtiow;

#[allow(dead_code)]
struct SceneData {
    glass: Arc<dyn Material>,
    aluminum: Arc<dyn Material>,
    empty: Arc<dyn Material>,
    red: Arc<dyn Material>,
    green: Arc<dyn Material>,
}

#[allow(dead_code)]
impl SceneData {
    const ALUMINUM: Metal = Metal::new(Color::new(0.722, 0.721, 0.709), 0.20);
    const GLASS: Dielectric = Dielectric::new(1.5);

    pub fn new() -> Self {
        Self {
            glass: Arc::new(Self::GLASS),
            aluminum: Arc::new(Self::ALUMINUM),
            empty: Arc::new(Empty),
            red: Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05), 1.0)),
            green: Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15), 1.0)),
        }
    }

    pub fn glass(&self) -> Arc<dyn Material> {
        self.glass.clone()
    }

    pub fn aluminum(&self) -> Arc<dyn Material> {
        self.aluminum.clone()
    }

    pub fn empty(&self) -> Arc<dyn Material> {
        self.empty.clone()
    }

    pub fn green(&self) -> Arc<dyn Material> {
        self.green.clone()
    }
    pub fn red(&self) -> Arc<dyn Material> {
        self.red.clone()
    }
}

pub trait Scene {
    fn build(&self) -> (HittableList, HittableList);

    fn default_camera(&self) -> CameraBuilder;

    fn render(&self, cam: &Camera) -> Vec<f32> {
        let (world, lights) = self.build();

        let world = Bvh::build(world);

        cam.render(&world, &lights)
    }
}

impl Default for SceneData {
    fn default() -> Self {
        Self::new()
    }
}
