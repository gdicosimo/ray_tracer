use std::ops::Neg;

use super::*;

impl Coord {
    pub fn simd_negate(&self) -> Coord {
        self.simd_mul(&Self::NEG)
    }
}

// ─────────────────────────────

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(self.as_coord().simd_negate())
    }
}

// ─────────────────────────────

impl Neg for UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> Self::Output {
        UnitVec3(-self.into_vec3())
    }
}

impl Neg for &UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> Self::Output {
        UnitVec3(-self.as_vec3_ref())
    }
}
