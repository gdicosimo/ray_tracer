use super::*;

pub struct Onb {
    u: UnitVec3,
    v: UnitVec3,
    w: UnitVec3,
}

impl Onb {
    /// Creates a new ONB with the given normal vector as the w basis.
    ///
    /// The algorithm:
    ///
    /// 1. Starts with the input normal as w
    /// 2. Chooses an auxiliary vector that's unlikely to be parallel to w
    /// 3. Computes v = w × aux (cross product)
    /// 4. Computes u = w × v to complete the right-handed system
    ///
    /// This method is numerically stable and efficient, avoiding
    /// degenerate cases where the input normal is nearly parallel
    /// to the auxiliary vector.
    ///
    /// # Arguments
    /// * `w` - Unit vector to use as the normal (w basis vector)
    ///
    /// # Implementation Details
    ///
    /// The auxiliary vector is chosen based on the x component of w:
    /// - If w.x > 0.9: use y-axis to avoid near-parallel vectors
    /// - Otherwise: use x-axis as it's generally well-behaved
    fn new(w: UnitVec3) -> Self {
        let aux = if w.x().abs() > 0.9 {
            //This was the problem
            UnitVec3::J
        } else {
            UnitVec3::I
        };

        let v = w.cross(&aux).unchecked_normalize();
        let u = UnitVec3::new(w.cross(&v)); //Safety

        Self { u, v, w }
    }

    #[allow(dead_code)]
    pub fn from_vec3(vec: Vec3) -> Self {
        let n = vec.unchecked_into_unit_vector();
        Self::new(n)
    }

    pub fn from_unit_vec3(n: UnitVec3) -> Self {
        Self::new(n.clone())
    }

    /// Transforms a vector from local to world coordinates using the ONB.
    ///
    ///
    /// ```text
    /// v_world = x*u + y*v + z*w
    /// ```
    ///
    /// where (x,y,z) are the components of the input vector in the local coordinate
    /// system defined by the ONB's basis vectors (u,v,w).
    ///
    /// # Arguments
    /// * `vec` - Vector in local coordinates where:
    ///   - x: component along the u basis (tangent)
    ///   - y: component along the v basis (bitangent)
    ///   - z: component along the w basis (normal)
    ///
    ///
    /// # Example
    /// ```
    /// use crate::math::{Onb, Vec3};
    ///
    /// // Create ONB with normal along y-axis
    /// let onb = Onb::from_vec3(&Vec3::new(0.0, 1.0, 0.0));
    ///
    /// // Transform a vector pointing 45° from normal toward tangent
    /// let local_vec = Vec3::new(
    ///     0.707, // x = sin(45°)
    ///     0.0,   // y = 0
    ///     0.707  // z = cos(45°)
    /// );
    /// let world_vec = onb.transform(&local_vec);
    /// ```
    pub fn transform(&self, vec: &Vec3) -> Vec3 {
        vec.x() * &self.u + vec.y() * &self.v + vec.z() * &self.w
    }

    pub fn u(&self) -> &UnitVec3 {
        &self.u
    }

    pub fn v(&self) -> &UnitVec3 {
        &self.v
    }

    pub fn w(&self) -> &UnitVec3 {
        &self.w
    }
}
