use super::*;

pub(crate) use crate::util::fmt;
pub(crate) use coord::Coord;

pub use {onb::*, ops::*, point3::*, unit_vec3::*, vec3::*};

mod coord;
mod onb;
mod ops;
mod point3;
mod unit_vec3;
mod vec3;

#[allow(dead_code)]
pub trait Dimensional: Sized {
    fn x(&self) -> f32;

    fn y(&self) -> f32;

    fn z(&self) -> f32;

    #[inline(always)]
    fn xy(&self) -> [f32; 2] {
        [self.x(), self.y()]
    }

    #[inline(always)]
    fn xz(&self) -> [f32; 2] {
        [self.x(), self.z()]
    }

    #[inline(always)]
    fn yz(&self) -> [f32; 2] {
        [self.y(), self.z()]
    }

    #[inline(always)]
    fn xyz(&self) -> [f32; 3] {
        [self.x(), self.y(), self.z()]
    }
}

pub trait Mutable: Dimensional {
    fn set_x(&mut self, x: f32) -> &mut Self;

    fn set_y(&mut self, y: f32) -> &mut Self;

    fn set_z(&mut self, z: f32) -> &mut Self;
}

#[allow(dead_code)]
pub trait Inmutable: Dimensional {
    type Output: Dimensional;

    fn with_x(&self, x: f32) -> Self::Output;

    fn with_y(&self, y: f32) -> Self::Output;

    fn with_z(&self, z: f32) -> Self::Output;
}

pub trait Measurable: Dimensional {
    fn norm(&self) -> f32;

    fn len_squared(&self) -> f32;
}
