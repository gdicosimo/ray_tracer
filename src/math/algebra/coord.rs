use super::{Dimensional, Mutable};

/// Internal coordinate type optimized for SIMD operations.
///
/// This type is not meant to be used directly by client code. Instead, use
/// the higher-level types `Vec3` and `Point3` which provide a more ergonomic
/// interface for 3D geometry operations.
///
/// The struct is aligned to 16 bytes to match SIMD register requirements,
/// enabling efficient vectorized operations.
///
/// # Implementation Notes
/// - Uses a 4D vector internally for SIMD alignment
/// - The fourth component (w) is typically set to 0.0
/// - All operations are designed to preserve SIMD optimization opportunities
#[repr(C, align(16))]
#[derive(Clone, PartialEq)]
pub(crate) struct Coord(pub(crate) [f32; 4]);

impl Coord {
    pub const ORIGIN: Coord = Self([0.0; 4]);
    pub const NEG: Coord = Self([-1.0; 4]);

    pub const fn new3(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 0.0])
    }

    pub const fn from_array3([x, y, z]: [f32; 3]) -> Self {
        Self([x, y, z, 0.0])
    }

    pub fn len(&self) -> f32 {
        self.simd_sqrt(self.len_squared())
    }

    pub fn len_squared(&self) -> f32 {
        // self.0.iter().map(|x| x * x).sum()
        self.simd_dot(self)
    }
}

impl Dimensional for Coord {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn z(&self) -> f32 {
        self[2]
    }
}

impl Mutable for Coord {
    fn set_x(&mut self, x: f32) -> &mut Self {
        self.0[0] = x;
        self
    }

    fn set_y(&mut self, y: f32) -> &mut Self {
        self.0[1] = y;
        self
    }

    fn set_z(&mut self, z: f32) -> &mut Self {
        self.0[2] = z;
        self
    }
}

impl Default for Coord {
    fn default() -> Self {
        Coord::ORIGIN
    }
}

#[cfg(test)]
mod test {
    use super::Coord;

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<Coord>(), 16)
    }

    #[test]
    fn align() {
        assert_eq!(std::mem::align_of::<Coord>(), 16)
    }
}
