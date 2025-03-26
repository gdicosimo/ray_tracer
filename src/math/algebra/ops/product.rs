use super::*;

pub trait Product<Rhs = Self> {
    fn dot(&self, other: &Rhs) -> f32;
    fn cross(&self, other: &Rhs) -> Vec3;
}

impl Product<UnitVec3> for Vec3 {
    fn dot(&self, other: &UnitVec3) -> f32 {
        self.as_coord().simd_dot(other.as_coord())
    }

    fn cross(&self, other: &UnitVec3) -> Vec3 {
        Self(self.as_coord().simd_cross(other.as_coord()))
    }
}

impl Product<Vec3> for Vec3 {
    fn dot(&self, other: &Vec3) -> f32 {
        self.as_coord().simd_dot(other.as_coord())
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Self(self.as_coord().simd_cross(other.as_coord()))
    }
}

// ─────────────────────────────

impl Product<Vec3> for UnitVec3 {
    fn dot(&self, other: &Vec3) -> f32 {
        self.as_coord().simd_dot(other.as_coord())
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(self.as_coord().simd_cross(other.as_coord()))
    }
}

impl Product<UnitVec3> for UnitVec3 {
    fn dot(&self, other: &UnitVec3) -> f32 {
        self.as_coord().simd_dot(other.as_coord())
    }

    fn cross(&self, other: &UnitVec3) -> Vec3 {
        Vec3(self.as_coord().simd_cross(other.as_coord()))
    }
}
