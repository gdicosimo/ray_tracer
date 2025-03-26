use super::*;

#[derive(Default, Clone, PartialEq)]
pub struct Vec3(pub(crate) Coord);

impl Vec3 {
    pub const ZERO: Self = Self(Coord::ORIGIN);
    pub const I: Self = Vec3::new(1.0, 0.0, 0.0);
    pub const J: Self = Vec3::new(0.0, 1.0, 0.0);
    pub const K: Self = Vec3::new(0.0, 0.0, 1.0);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Coord::new3(x, y, z))
    }

    #[inline]
    pub const fn from_array(values: [f32; 3]) -> Self {
        Self(Coord::from_array3(values))
    }

    /// Creates a vector with all components set to the same value.
    ///
    /// # Examples
    /// ```
    /// let v = Vec3::splat(2.0);
    /// assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
    /// ```
    #[inline]
    pub fn splat(scalar: f32) -> Self {
        Self(Coord::simd_splat(scalar))
    }

    /// Generates a random vector with components uniformly distributed in [0, 1).
    ///
    /// The uniform distribution means each component is independently sampled
    /// with equal probability across the interval [0,1). This creates points
    /// that uniformly fill a unit cube in 3D space.
    ///
    #[inline]
    pub fn random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    /// Generates a random vector with components uniformly distributed in [min, max].
    ///
    /// This method maps the output of `random()` from [0,1) to [min,max] for each
    /// component. The distribution remains uniform over the new interval.
    ///
    #[inline]
    pub fn random_between(min: f32, max: f32) -> Self {
        Self::new(
            random_float_between(min, max),
            random_float_between(min, max),
            random_float_between(min, max),
        )
    }

    /// Generates a random unit vector with cosine-weighted distribution in the z-up hemisphere.
    ///
    /// This method implements cosine-weighted hemisphere sampling, which is crucial for
    /// efficient Monte Carlo integration in ray tracing. The probability density of the
    /// generated directions follows a cosine distribution:
    ///
    /// ```text
    /// p(θ,φ) = cos(θ)/π
    /// ```
    ///
    /// where:
    /// - θ is the polar angle from the z-axis (0 ≤ θ ≤ π/2)
    /// - φ is the azimuthal angle in the xy-plane (0 ≤ φ < 2π)
    ///
    /// This distribution is important because:
    /// 1. It matches the natural falloff of diffuse reflection (Lambert's law)
    /// 2. It provides optimal sampling for diffuse surfaces
    /// 3. It reduces noise in rendered images
    ///
    ///
    /// # Examples
    ///
    /// Sampling diffuse reflection:
    /// ```
    /// use crate::math::{Vec3, Onb};
    ///
    /// // Create local coordinate system at hit point
    /// let normal = Vec3::new(0.0, 1.0, 0.0);
    /// let onb = Onb::from_vec3(&normal);
    ///
    /// // Generate cosine-weighted direction in local space
    /// let local_dir = Vec3::random_cosine_direction();
    ///
    /// // Transform to world space
    /// let world_dir = onb.transform(&local_dir);
    /// ```
    #[inline]
    pub fn random_cosine_direction() -> Self {
        let r1 = random_float();
        let r2 = random_float();
        let sqrt_r2 = f32::sqrt(r2);

        let phi = 2.0 * PI * r1;
        let x = f32::cos(phi) * sqrt_r2;
        let y = f32::sin(phi) * sqrt_r2;
        let z = f32::sqrt(1.0 - r2);

        Self::new(x, y, z)
    }

    /// Scales this vector in place by a scalar value.
    ///
    /// # Examples
    /// ```
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let scaled = v.scaled_inplace(2.0);
    /// assert_eq!(scaled, Vec3::new(2.0, 4.0, 6.0));
    /// ```
    #[inline]
    pub fn scaled_inplace(mut self, scalar: f32) -> Self {
        self *= scalar;
        self
    }

    /// Projects this vector onto a unit vector, computing the parallel component.
    ///
    /// Vector projection decomposes a vector into components parallel and perpendicular
    /// to a given direction. This method computes the parallel component:
    ///
    /// ```text
    /// proj_n(v) = (v·n)n
    /// ```
    ///
    /// # Examples
    ///
    /// Basic projection:
    /// ```
    ///
    /// let v = Vec3::new(1.0, 1.0, 0.0);
    /// let n = UnitVec3::try_new(1.0, 0.0, 0.0).unwrap();
    /// let proj = v.proyect(&n);
    /// assert_eq!(proj, Vec3::new(1.0, 0.0, 0.0));
    /// ```
    pub fn proyect(&self, n: &UnitVec3) -> Vec3 {
        self.dot(n) * n
    }

    /// Computes the rejection of this vector from a unit vector, giving the perpendicular component.
    ///
    /// Vector rejection gives the component of a vector perpendicular to a given direction.
    /// It is computed by subtracting the projection from the original vector:
    ///
    /// ```text
    /// rej_n(v) = v - proj_n(v) = v - (v·n)n
    /// ```
    /// # Examples
    ///
    /// Computing surface tangent:
    /// ```
    /// // Surface normal
    /// let normal = UnitVec3::try_new(0.0, 1.0, 0.0).unwrap();
    ///
    /// // Arbitrary vector
    /// let v = Vec3::new(1.0, 1.0, 0.0);
    ///
    /// // Get component tangent to surface
    /// let tangent = v.reject(&normal);
    /// assert_eq!(tangent, Vec3::new(1.0, 0.0, 0.0));
    /// ```
    #[inline(always)]
    pub fn reject(&self, n: &UnitVec3) -> Vec3 {
        self - self.proyect(n)
    }

    /// Reflects this vector off a surface with the given normal.
    ///
    /// Implements the law of reflection where the angle of incidence equals
    /// the angle of reflection. The reflection formula is:
    ///
    /// ```text
    /// r = v - 2(v·n)n
    /// ```
    /// # Examples
    /// ```
    /// // Create incident vector and surface normal
    /// let v = Vec3::new(1.0, -1.0, 0.0).try_normalize().unwrap();
    /// let n = UnitVec3::try_new(0.0, 1.0, 0.0).unwrap();
    ///
    /// // Compute reflection
    /// let r = v.reflect(&n);
    ///
    /// // Verify angle of incidence equals angle of reflection
    /// let incident_angle = v.dot(&n).acos();
    /// let reflection_angle = r.dot(&n).acos();
    /// assert!((incident_angle - reflection_angle).abs() < 1e-6);
    /// ```
    ///
    /// # Examples
    /// ```
    /// let v = Vec3::new(1.0, -1.0, 0.0);
    /// let n = UnitVec3::try_new(0.0, 1.0, 0.0).unwrap();
    /// let reflected = v.reflect(&n);
    /// assert_eq!(reflected, Vec3::new(1.0, 1.0, 0.0));
    /// ```
    pub fn reflect(&self, n: &UnitVec3) -> Vec3 {
        let proj = 2.0 * self.dot(n);
        self - (proj * n)
    }

    /// Refracts this vector through a surface with the given normal and refractive indices ratio.
    ///
    /// Implements Snell's law for vector refraction, which relates the angles of incidence
    /// and refraction to the refractive indices of the materials:
    ///
    /// ```text
    /// n₁sin(θ₁) = n₂sin(θ₂)
    /// ```
    ///
    /// The vector form of Snell's law used here is:
    ///
    /// ```text
    /// t = η(v - (v·n)n) - n√(1 - η²(1 - (v·n)²))
    /// ```
    ///
    /// # Examples
    /// ```
    /// use crate::math::*;
    ///
    /// // Ray entering glass from air
    /// let v = Vec3::new(0.0, -1.0, 0.0).try_normalize().unwrap();
    /// let n = UnitVec3::try_new(0.0, 1.0, 0.0).unwrap();
    /// let n1_over_n2 = 1.0 / 1.5; // Air (1.0) to glass (1.5)
    /// let refracted = v.refract(&n, n1_over_n2);
    ///
    /// // Ray leaving glass back into air
    /// let n2_over_n1 = 1.5 / 1.0; // Glass (1.5) to air (1.0)
    /// let total_internal = v.refract(&n, n2_over_n1);
    /// ```
    ///
    /// Note: Total internal reflection occurs when the angle of incidence is greater
    /// than the critical angle. In this case, there is no refracted ray and all
    /// light is reflected.
    pub fn refract(&self, n: &UnitVec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self.dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (self + (cos_theta * n));
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).sqrt() * n;

        r_out_perp + r_out_parallel
    }

    /// Attempts to normalize this vector into a unit vector, returning an error if the vector is zero.
    ///
    /// Normalization scales a vector to have unit length (magnitude = 1) while preserving
    /// its direction. The formula is:
    ///
    /// ```text
    /// û = v/|v| where |v| = √(v·v)
    /// ```
    ///
    /// # Returns
    /// * `Ok(UnitVec3)` - A new unit vector in the same direction
    /// * `Err(UnitVecError)` - If the vector has zero length (cannot be normalized)
    ///
    /// # Examples
    ///
    /// Basic normalization:
    /// ```
    /// use crate::math::Vec3;
    ///
    /// let v = Vec3::new(3.0, 0.0, 0.0);
    /// let unit = v.try_normalize().unwrap();
    /// assert_eq!(unit.norm(), 1.0);
    /// assert_eq!(unit.x(), 1.0);
    /// ```
    ///
    /// Handling zero vectors:
    /// ```
    /// use crate::math::Vec3;
    ///
    /// let zero = Vec3::new(0.0, 0.0, 0.0);
    /// assert!(zero.try_normalize().is_err());
    /// ```
    pub fn try_normalize(&self) -> Result<UnitVec3, fmt::UnitVecError> {
        UnitVec3::try_from_vec3(self)
    }

    /// Normalizes this vector without checking if it has zero length, for improved performance.
    ///
    /// This method performs the same normalization as `try_normalize()` but without the
    /// safety checks.
    ///
    /// # Safety
    /// This method assumes the vector is not zero length. Using it on a zero vector
    /// will result in NaN or infinity values. Use only when:
    /// - You have explicitly checked the vector length
    /// - The vector comes from a calculation that guarantees non-zero length
    /// - You are willing to accept undefined behavior for zero vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::math::Vec3;
    ///
    /// // We know this vector isn't zero
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// // Safe to use unchecked normalization
    /// let unit = v.unchecked_normalize();
    /// assert!((unit.norm() - 1.0).abs() < 1e-6);
    /// ```
    ///
    pub fn unchecked_normalize(&self) -> UnitVec3 {
        UnitVec3::unchecked_from_vec3(self)
    }

    /// Consumes this vector and converts it to a unit vector without length check, for improved performance.
    pub fn unchecked_into_unit_vector(self) -> UnitVec3 {
        UnitVec3::unchecked_from_vec3(&self)
    }

    #[inline]
    pub fn is_near_zero(&self) -> bool {
        self.x().abs() < EPSILON && self.y().abs() < EPSILON && self.z().abs() < EPSILON
    }
}

impl Dimensional for Vec3 {
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

impl Mutable for Vec3 {
    fn set_x(&mut self, x: f32) -> &mut Self {
        self.0.set_x(x);
        self
    }

    fn set_y(&mut self, y: f32) -> &mut Self {
        self.0.set_y(y);
        self
    }

    fn set_z(&mut self, z: f32) -> &mut Self {
        self.0.set_z(z);
        self
    }
}

impl Measurable for Vec3 {
    fn norm(&self) -> f32 {
        self.as_coord().len()
    }

    fn len_squared(&self) -> f32 {
        self.as_coord().len_squared()
    }
}

#[cfg(test)]
mod tests {
    use super::super::ops::Product;
    use super::*;

    #[test]
    fn test_vec3_creation_and_accessors() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);

        let v_zero = Vec3::default();
        assert_eq!(v_zero.x(), 0.0);
        assert_eq!(v_zero.y(), 0.0);
        assert_eq!(v_zero.z(), 0.0);

        let v_neg = Vec3::new(-1.5, 2.5, -3.5);
        assert_eq!(v_neg.x(), -1.5);
        assert_eq!(v_neg.y(), 2.5);
        assert_eq!(v_neg.z(), -3.5);
    }

    #[test]
    fn test_addition_subtraction() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(&v1 + &v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(&v1 + v2.clone(), Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1.clone() + &v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1.clone() + v2.clone(), Vec3::new(5.0, 7.0, 9.0));

        let mut v3 = v1.clone();
        v3 += &v2;
        assert_eq!(v3, Vec3::new(5.0, 7.0, 9.0));

        let mut v4 = v1.clone();
        v4 += v2.clone();
        assert_eq!(v4, Vec3::new(5.0, 7.0, 9.0));

        assert_eq!(&v2 - &v1, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(v2.clone() - v1.clone(), Vec3::new(3.0, 3.0, 3.0));

        assert_eq!(&v2 - &v1, &v2 + (-&v1));
    }

    #[test]
    fn test_scalar_operations() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let scalar = 2.0;

        assert_eq!(scalar * &v, Vec3::new(4.0, 8.0, 12.0));
        assert_eq!(scalar * v.clone(), Vec3::new(4.0, 8.0, 12.0));

        let mut v1 = v.clone();
        v1 *= scalar;
        assert_eq!(v1, Vec3::new(4.0, 8.0, 12.0));

        assert_eq!(scalar / &v1, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(scalar / v.clone(), Vec3::new(1.0, 2.0, 3.0));

        let mut v2 = Vec3::new(4.0, 8.0, 12.0);
        v2 /= scalar;
        assert_eq!(v2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.dot(&v2), 32.0);

        let cross_result = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(v1.cross(&v2), cross_result);
    }

    #[test]
    fn test_edge_cases() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 0.0;
        assert!(v.x().is_infinite() && v.x().is_sign_positive());
        assert!(v.y().is_infinite() && v.y().is_sign_positive());
        assert!(v.z().is_infinite() && v.z().is_sign_positive());

        let nan_vec = Vec3::new(f32::NAN, f32::NAN, f32::NAN);
        assert!(nan_vec.x().is_nan());
        assert!((2.0 * &nan_vec).x().is_nan());
    }

    #[test]
    fn test_borrowing_and_ownership() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);

        let _sum = &v1 + &v2;
        let _sum = &v1 + v2.clone();
        let _sum = v1.clone() + &v2;

        assert_eq!(v1.x(), 1.0);
        assert_eq!(v2.x(), 2.0);
    }
}
