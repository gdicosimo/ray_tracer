use super::{Point3, Vec3};

/// A ray in 3D space, defined by an origin point and a direction vector.
///
/// # Examples
///
/// ```
/// use crate::math::{Point3, Ray, Vec3};
///
/// // Create a ray starting at origin (0,0,0) pointing in x direction
/// let origin = Point3::new(0.0, 0.0, 0.0);
/// let direction = Vec3::new(1.0, 0.0, 0.0);
/// let ray = Ray::new(origin, direction);
///
/// // Get a point 2 units along the ray
/// let point = ray.at(2.0);
/// assert_eq!(point.x(), 2.0);
/// ```
#[derive(Default, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f32,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir, tm: 0.0 }
    }

    pub fn from_array(orig: [f32; 3], dir: [f32; 3]) -> Self {
        Self {
            orig: Point3::from_array(orig),
            dir: Vec3::from_array(dir),
            tm: 0.0,
        }
    }

    pub fn from_tuple(orig_dir: (Point3, Vec3)) -> Self {
        Self::new(orig_dir.0, orig_dir.1)
    }

    pub fn as_tuple(&self) -> (&Point3, &Vec3) {
        (&self.orig, &self.dir)
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn into_parts(self) -> (Point3, Vec3) {
        (self.orig, self.dir)
    }

    pub fn into_origin(self) -> Point3 {
        self.orig
    }

    pub fn into_direction(self) -> Vec3 {
        self.dir
    }

    /// Computes a point along the ray at parameter t.
    ///
    /// The point is calculated using the parametric equation P(t) = O + tD,
    /// where O is the origin and D is the direction.
    ///
    /// # Examples
    /// ```
    /// let ray = Ray::new(Point3::ORIGIN, Vec3::new(1.0, 0.0, 0.0));
    /// let point = ray.at(2.0); // Point 2 units along x-axis
    /// assert_eq!(point, Point3::new(2.0, 0.0, 0.0));
    /// ```
    pub fn at(&self, t: f32) -> Point3 {
        &self.orig + (t * &self.dir)
    }

    /// Returns the ray's time parameter used for motion blur.
    pub fn time(&self) -> f32 {
        self.tm
    }

    pub fn set_time(mut self, tm: f32) -> Self {
        self.tm = tm;
        self
    }
}
