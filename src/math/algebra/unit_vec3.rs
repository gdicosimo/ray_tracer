use super::*;

#[derive(Clone, PartialEq)]
pub struct UnitVec3(pub(crate) Vec3);

impl UnitVec3 {
    pub const I: UnitVec3 = Self(Vec3::I);
    pub const J: UnitVec3 = Self(Vec3::J);
    pub const K: UnitVec3 = Self(Vec3::K);

    /// Creates a new unit vector from x, y, z components.
    ///
    /// # Returns
    /// * `Ok(UnitVec3)` - A normalized vector if successful
    /// * `Err(UnitVecError::ZeroVector)` - If the components form a zero vector
    ///
    /// # Examples
    /// ```
    ///
    /// use ray_tracer::math::*;
    ///
    /// let normal = UnitVec3::try_new(1.0, 1.0, 1.0).unwrap();
    /// assert!((normal.norm() - 1.0).abs() < EPSILON);
    /// ```
    pub fn try_new(x: f32, y: f32, z: f32) -> Result<Self, fmt::UnitVecError> {
        Self::try_from_vec3(&Vec3::new(x, y, z))
    }

    /// Creates a unit vector from a 3-element array.
    ///
    /// # Returns
    /// * `Ok(UnitVec3)` - A normalized vector if successful
    /// * `Err(UnitVecError::ZeroVector)` - If the components form a zero vector
    pub fn try_from_array(values: [f32; 3]) -> Result<Self, fmt::UnitVecError> {
        Self::try_new(values[0], values[1], values[2])
    }

    /// Creates a unit vector by normalizing an existing Vec3.
    ///
    /// This method uses SIMD operations for efficient normalization when available.
    ///
    /// # Arguments
    /// * `vec` - The vector to normalize
    ///
    /// # Returns
    /// * `Ok(UnitVec3)` - A normalized vector if successful
    /// * `Err(UnitVecError::ZeroVector)` - If the input vector has zero magnitude
    ///
    /// # Examples
    /// ```
    ///
    /// use ray_tracer::math::*;
    ///
    /// let vec = Vec3::new(2.0, 0.0, 0.0);
    /// let unit = UnitVec3::try_from_vec3(&vec).unwrap();
    /// assert_eq!(unit.x(), 1.0);
    /// ```
    pub fn try_from_vec3(vec: &Vec3) -> Result<Self, fmt::UnitVecError> {
        let len_sq = vec.len_squared();

        if (len_sq - 1.0).abs() < EPSILON {
            return Ok(Self(vec.clone()));
        }

        let inv_sqrt = Coord::simd_rsqrt(vec.as_coord(), len_sq);

        if inv_sqrt.is_finite() && inv_sqrt > 0.0 {
            let vec_norm = inv_sqrt * vec;
            Ok(Self(vec_norm))
        } else {
            Err(fmt::UnitVecError::ZeroVector)
        }
    }

    /// Creates a new `UnitVec3` without normalization checks.
    ///
    /// **Warning:** This constructor does not perform any checks to ensure that the resulting vector
    /// is actually a unit vector.  It's intended for cases where normalization is guaranteed
    /// by other means or for performance-critical code where the overhead of checks is undesirable
    /// and the caller is responsible for correctness.
    ///
    /// # Safety
    ///
    /// Using this constructor can lead to undefined behavior in parts of your code that rely
    /// on `UnitVec3` instances being correctly normalized if the input components do not
    /// represent a vector that can be normalized or are already normalized.
    pub fn unchecked_new(x: f32, y: f32, z: f32) -> Self {
        Self::unchecked_from_vec3(&Vec3::new(x, y, z))
    }

    /// # Safety
    ///
    /// Same safety considerations as `unchecked_new`.
    pub fn unchecked_from_array(values: [f32; 3]) -> Self {
        Self::unchecked_new(values[0], values[1], values[2])
    }

    /// # Safety
    ///
    /// Same safety considerations as `unchecked_new`.
    pub fn unchecked_from_vec3(vec: &Vec3) -> Self {
        let len_sq = vec.len_squared();

        if (len_sq - 1.0).abs() < EPSILON {
            return Self(vec.clone());
        }

        //While it is faster using rsqrt (although the difference is not so noticeable) it brings more acne shadow
        // let rsqrt = Coord::simd_rsqrt(vec.as_coord(), len_sq);
        let inv_sqrt = Coord::simd_sqrt(vec.as_coord(), len_sq).recip();

        Self(inv_sqrt * vec)
    }

    /// Creates a new `UnitVec3` from a vector and a radius, assuming the vector represents a point on a sphere.
    ///
    /// This function normalizes the input vector by dividing it by the given radius `r`.  
    /// It is crucial that `vec` represents a point *on* a sphere of radius `r` centered at the origin.  
    /// If `vec` does not lie on this sphere, the resulting `UnitVec3` will not be a true unit vector.
    ///
    pub fn try_from_radius(vec: &Vec3, r: f32) -> Result<Self, fmt::UnitVecError> {
        let normalized = r / vec;

        if !normalized.x().is_nan() || normalized.len_squared() == 1.0 {
            Ok(Self(normalized))
        } else {
            Err(fmt::UnitVecError::ZeroVector)
        }
    }

    /// Creates a new `UnitVec3` from a vector and a radius without performing any checks.
    ///
    ///
    pub fn unchecked_from_radius(vec: &Vec3, r: f32) -> Self {
        let normalized = r / vec;

        Self(normalized)
    }

    /// Generates a random unit vector using rejection sampling.
    ///
    /// This method generates random vectors in a unit cube and rejects those
    /// that fall outside the unit sphere, ensuring uniform distribution on
    /// the sphere's surface.
    ///
    /// # Performance
    /// This method uses rejection sampling which may require multiple iterations.
    /// For performance-critical code, consider caching the results.
    ///
    /// # Returns
    /// A random unit vector uniformly distributed on the unit sphere.
    pub fn unchecked_random() -> Self {
        loop {
            let v = Vec3::random_between(-1.0, 1.0);
            let normsq = v.len_squared();
            if (NORMALIZATION_TOLERANCE..=1.0).contains(&normsq) {
                return Self::unchecked_from_vec3(&v);
            }
        }
    }

    /// Generates a random unit vector in the hemisphere defined by a normal.
    ///
    /// # Arguments
    /// * `u` - The normal vector defining the hemisphere's direction
    ///
    /// # Returns
    /// A random unit vector in the hemisphere oriented along `u`.
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::math::*;
    ///
    /// let normal = UnitVec3::I; // Up vector
    /// let random = UnitVec3::unchecked_random_on_hemisphere(&normal);
    /// assert!(random.dot(&normal) >= 0.0); // Vector is in upper hemisphere
    /// ```
    pub fn unchecked_random_on_hemisphere(u: &UnitVec3) -> UnitVec3 {
        use super::ops::Product;

        let on_unit_sphere = Self::unchecked_random();
        if on_unit_sphere.dot(u) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    ///This method should not be used without checking that the new resulting vector is still UnitVec3.
    pub(crate) fn new(v: Vec3) -> Self {
        Self(v)
    }
}

impl Dimensional for UnitVec3 {
    #[inline(always)]
    fn x(&self) -> f32 {
        self.as_coord().x()
    }

    #[inline(always)]
    fn y(&self) -> f32 {
        self.as_coord().y()
    }

    #[inline(always)]
    fn z(&self) -> f32 {
        self.as_coord().z()
    }
}

impl Inmutable for UnitVec3 {
    type Output = Vec3;

    #[inline]
    fn with_x(&self, x: f32) -> Self::Output {
        Vec3::new(x, self.y(), self.z())
    }

    #[inline]
    fn with_y(&self, y: f32) -> Self::Output {
        Vec3::new(self.x(), y, self.z())
    }

    #[inline]
    fn with_z(&self, z: f32) -> Self::Output {
        Vec3::new(self.x(), self.y(), z)
    }
}

impl Measurable for UnitVec3 {
    /// Return the norm_squared since it is more effective and ought to yield 1.
    #[inline(always)]
    fn norm(&self) -> f32 {
        self.len_squared()
    }

    #[inline(always)]
    fn len_squared(&self) -> f32 {
        let squeared = self.as_coord().len_squared();

        assert!(
            (1.0 - squeared).abs() < NORMALIZATION_TOLERANCE,
            "UnitVec3 is not normalized! Norm: {}",
            squeared,
        );

        squeared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tolerance used for comparing floating-point values.
    const TOLERANCE: f32 = 1e-3;

    #[test]
    fn test_normalization() {
        // Create a vector that is not zero.
        let v = Vec3::new(35.0985, -106.5932, 0.0);
        let unit = v.try_normalize().expect("Vector should be non-zero");
        // The normalized vector should have a norm of approximately 1.
        assert!(
            (unit.norm() - 1.0).abs() < TOLERANCE,
            "Normalized vector norm is not 1: {}",
            unit.norm()
        );
    }

    #[test]
    fn test_normalize_zero_vector() {
        // Normalizing a zero vector should return None.
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert!(
            v.try_normalize().is_err(),
            "Normalizing a zero vector should return None"
        );
    }

    #[test]
    fn test_try_from_radius() {
        // Test converting a vector to a unit vector given a specific radius.
        let v = Vec3::new(2.45, 3.0, 1.0);
        // Assume we desire a radius of 4.0 for the original vector.
        let unit = UnitVec3::try_from_radius(&v, 4.0).expect("Conversion from radius failed");
        // The resulting unit vector should have a norm of 1.
        assert!(
            (unit.norm() - 1.0).abs() < TOLERANCE,
            "Unit vector norm is not 1: {}",
            unit.norm()
        );
    }

    #[test]
    fn test_unit_dot_product() {
        // Verify that the dot product of a unit vector with itself is 1.
        let v = Vec3::new(1.0, 2.0, 3.0);
        let unit = v.unchecked_normalize();
        let dot = unit.dot(&unit);
        assert!(
            (dot - 1.0).abs() < TOLERANCE,
            "Dot product of unit vector with itself is not 1: {}",
            dot
        );
    }

    #[test]
    fn test_reciprocal() {
        // Compare the standard reciprocal of the norm with the fast reciprocal square root.
        let v = Vec3::new(3.0, 4.0, 0.0);
        let norm = v.norm();
        let recip = norm.recip();
        let fast_recip = Coord::simd_rsqrt(v.as_coord(), v.len_squared());
        assert!(
            (recip - fast_recip).abs() < TOLERANCE,
            "Reciprocal mismatch: {} vs {}",
            recip,
            fast_recip
        );
    }

    #[test]
    fn test_angle_between_unit_vectors() {
        // Create two unit vectors with a known angle.
        let v1 = Vec3::new(1.0, 0.0, 0.0).unchecked_normalize();
        // Create another vector at a 45° angle relative to v1 in the XY plane.
        let v2 = Vec3::new(1.0, 1.0, 0.0).unchecked_normalize();
        let dot = v1.dot(&v2);
        // The expected angle between v1 and v2 is acos(dot) ≈ 45° (PI/4 radians).
        let angle = dot.acos();
        assert!(
            (angle - (PI / 4.0)).abs() < TOLERANCE,
            "Angle between unit vectors is not 45°: {} radians",
            angle
        );
    }
}
