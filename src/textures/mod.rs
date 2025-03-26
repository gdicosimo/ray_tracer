mod checker;
mod image_texture;
mod noise;
mod perlin;
mod solid;

use std::sync::Arc;

pub(crate) use crate::{
    math::{self, Color, Dimensional, Point3, WHITE},
    util::RtwImage,
};

pub(crate) use perlin::Perlin;

pub use checker::CheckerTexture;
pub use image_texture::ImageTexture;
pub use noise::{MarbleTexture, Melamine, NoiseTexture, WoodTexture};
pub use solid::SolidColor;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}
