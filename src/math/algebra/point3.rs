use super::*;

#[derive(Default, Clone)]
pub struct Point3(pub(crate) Coord);

impl Point3 {
    pub const ORIGIN: Self = Self(Coord::ORIGIN);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Coord::new3(x, y, z))
    }

    pub const fn from_array(values: [f32; 3]) -> Self {
        Self(Coord::from_array3(values))
    }

    pub const fn from_origin() -> Self {
        Self(Coord::ORIGIN)
    }

    /// Creates a new point with all coordinates set to the same value.
    pub const fn splat(value: f32) -> Self {
        Self(Coord::new3(value, value, value))
    }

    /// Translates this point by a vector, returning a new point.
    ///
    /// # Arguments
    /// * `v` - Vector to translate by
    ///
    /// # Examples
    /// ```
    /// let p = Point3::new(1.0, 1.0, 1.0);
    /// let v = Vec3::new(0.0, 1.0, 0.0);
    /// let translated = p.translate(&v);
    /// assert_eq!(translated.y(), 2.0);
    /// ```
    #[inline]
    pub fn translate(&self, v: &Vec3) -> Self {
        self + v
    }

    #[inline]
    pub fn translate_inplace(mut self, v: &Vec3) -> Self {
        self += v;
        self
    }

    /// Scales this point's coordinates relative to the origin.
    ///
    /// # Examples
    /// ```
    /// let p = Point3::new(1.0, 2.0, 3.0);
    /// let scaled = p.scaled_from_origin(2.0);
    /// assert_eq!(scaled, Point3::new(2.0, 4.0, 6.0));
    /// ```
    #[inline]
    pub fn scaled_from_origin(mut self, scalar: f32) -> Self {
        self *= scalar;
        self
    }

    /// Generates a random point inside a unit disk on the xy-plane (z=0).
    ///
    /// Uses rejection sampling to generate points uniformly within
    /// a disk of radius 1 centered at the origin.
    ///
    /// # Returns
    /// A point (x,y,0) where x² + y² < 1
    ///
    /// # Performance
    /// Uses rejection sampling which may require multiple iterations.
    /// For performance-critical code, consider caching the results.
    pub fn random_in_unit_disk() -> Self {
        loop {
            let point = Self::new(random_float(), random_float(), 0.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_between(min: f32, max: f32) -> Self {
        Self::new(
            random_float_between(min, max),
            random_float_between(min, max),
            random_float_between(min, max),
        )
    }

    fn length_squared(&self) -> f32 {
        self.as_coord().len_squared()
    }

    /// Returns a new point with each coordinate being the minimum of
    /// the corresponding coordinates from this point and another.
    ///
    /// # Examples
    /// ```
    /// let p1 = Point3::new(1.0, 4.0, 2.0);
    /// let p2 = Point3::new(2.0, 3.0, 1.0);
    /// let min_point = p1.min(&p2);
    /// assert_eq!(min_point, Point3::new(1.0, 3.0, 1.0));
    /// ```
    pub fn min(&self, other: &Self) -> Self {
        Self(self.as_coord().simd_min(other.as_coord()))
    }

    /// Returns a new point with each coordinate being the maximum of
    /// the corresponding coordinates from this point and another.
    pub fn max(&self, other: &Self) -> Self {
        Self(self.as_coord().simd_max(other.as_coord()))
    }
}

impl Dimensional for Point3 {
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

impl Mutable for Point3 {
    #[inline(always)]
    fn set_x(&mut self, x: f32) -> &mut Self {
        self.0.set_x(x);
        self
    }

    #[inline(always)]
    fn set_y(&mut self, y: f32) -> &mut Self {
        self.0.set_y(y);
        self
    }

    #[inline(always)]
    fn set_z(&mut self, z: f32) -> &mut Self {
        self.0.set_z(z);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_assign() {
        let p = Point3::from_origin();
        let mut v = Vec3::new(1.0, 2.0, 3.0);

        v += Vec3::default();
        p.translate_inplace(&v);
    }
}
