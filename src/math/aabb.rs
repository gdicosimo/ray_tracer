use std::ops::Add;

use super::*;

/// A 3D axis-aligned bounding box.
///
/// An AABB is represented by three intervals, one for each axis (x, y, z).
/// Each interval defines the minimum and maximum extent of the box along that axis.
///
/// # Examples
///
/// ```
/// use crate::math::{Aabb, Point3};
///
/// // Create an AABB from two diagonal points
/// let min = Point3::new(0.0, 0.0, 0.0);
/// let max = Point3::new(1.0, 2.0, 3.0);
/// let box = Aabb::from_points(min, max);
///
/// // Get the center of the box
/// let center = box.center();
/// assert_eq!(center, [0.5, 1.0, 1.5]);
/// ```
#[derive(Default, Debug, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    const DELTA: f32 = 0.0001;

    /// Creates an AABB from three intervals representing its extent along each axis.
    ///
    /// The box is automatically padded if any dimension is too thin.
    #[inline(always)]
    pub fn from_intervals(a: Interval, b: Interval, c: Interval) -> Self {
        let mut aabb = Self { x: a, y: b, z: c };
        aabb.pad_to_minimus();
        aabb
    }

    /// Creates an AABB from two points representing opposite corners.
    ///
    /// The points can be any two opposite corners of the box; they don't need
    /// to be specifically the minimum and maximum corners.
    ///
    /// # Arguments
    /// * `a` - First corner point
    /// * `b` - Opposite corner point
    ///
    /// # Examples
    /// ```
    /// let min = Point3::new(0.0, 0.0, 0.0);
    /// let max = Point3::new(1.0, 2.0, 3.0);
    /// let box = Aabb::from_points(min, max);
    /// ```
    #[inline(always)]
    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self::from_intervals(
            Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        )
    }

    /// The resulting box is the smallest AABB that completely encloses
    /// both input boxes.
    pub fn from_boxes(a: &Aabb, b: &Aabb) -> Self {
        Self {
            x: a.x.merge(&b.x),
            y: a.y.merge(&b.y),
            z: a.z.merge(&b.z),
        }
    }
    /// Alias `from_boxes`
    pub fn surrounding(a: &Aabb, b: &Aabb) -> Self {
        Self::from_boxes(a, b)
    }

    /// Returns the intersection (common region) of two AABBs.
    ///
    /// The intersection is computed by intersecting the intervals of each axis.
    /// If any axis does not overlap, returns `Aabb::EMPTY`.
    pub fn intersection(a: &Aabb, b: &Aabb) -> Self {
        let x_int = a.x.intersection(&b.x);
        let y_int = a.y.intersection(&b.y);
        let z_int = a.z.intersection(&b.z);

        if let (Some(x), Some(y), Some(z)) = (x_int, y_int, z_int) {
            Self::from_intervals(x, y, z)
        } else {
            Self::EMPTY
        }
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis index"),
        }
    }

    pub fn x(&self) -> &Interval {
        &self.x
    }

    pub fn y(&self) -> &Interval {
        &self.y
    }

    pub fn z(&self) -> &Interval {
        &self.z
    }

    pub fn longest_axis(&self) -> usize {
        let extents = [
            self.x.max - self.x.min,
            self.y.max - self.y.min,
            self.z.max - self.z.min,
        ];
        let (axis, _) = extents
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        axis
    }

    pub fn center(&self) -> [f32; 3] {
        [
            (self.x.min + self.x.max) * 0.5,
            (self.y.min + self.y.max) * 0.5,
            (self.z.min + self.z.max) * 0.5,
        ]
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            x: self.x.merge(&other.x),
            y: self.y.merge(&other.y),
            z: self.z.merge(&other.z),
        }
    }

    pub fn merge_inplace(&mut self, other: &Self) {
        self.x.merge_inplace(&other.x);
        self.y.merge_inplace(&other.y);
        self.z.merge_inplace(&other.z);
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Interval> {
        let mut current_interval = ray_t;

        for axis in 0..3 {
            let inv_dir = 1.0 / ray.direction()[axis];
            let orig = ray.origin()[axis];
            let ax = self.axis_interval(axis);

            let mut t0 = (ax.min - orig) * inv_dir;
            let mut t1 = (ax.max - orig) * inv_dir;

            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            current_interval.min = t0.max(current_interval.min);
            current_interval.max = t1.min(current_interval.max);

            if current_interval.max <= current_interval.min {
                return None;
            }
        }

        Some(current_interval)
    }

    pub fn mirror_yz(&self) -> Self {
        let mut new_min = -self.x.max;
        let mut new_max = -self.x.min;

        if new_min > new_max {
            std::mem::swap(&mut new_min, &mut new_max);
        }

        Aabb {
            x: Interval::new(new_min, new_max),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }

    /// The rotation is performed on the x and z axes, leaving y unchanged.
    pub fn rotate_y(&self, cos_theta: f32, sin_theta: f32) -> Self {
        let (new_x, new_z) = Aabb::rotate_axes(&self.x, &self.z, |x, z| {
            // Fórmulas para rotación en Y:
            //   x' = cosθ*x + sinθ*z
            //   z' = -sinθ*x + cosθ*z
            (
                cos_theta * x + sin_theta * z,
                -sin_theta * x + cos_theta * z,
            )
        });
        Self {
            x: new_x,
            y: self.y.clone(),
            z: new_z,
        }
    }

    /// The rotation is performed on the x and y axes, leaving z unchanged.
    pub fn rotate_z(&self, cos_theta: f32, sin_theta: f32) -> Self {
        let (new_x, new_y) = Aabb::rotate_axes(&self.x, &self.y, |x, y| {
            //   x' = cosθ*x - sinθ*y
            //   y' = sinθ*x + cosθ*y
            (cos_theta * x - sin_theta * y, sin_theta * x + cos_theta * y)
        });
        Self {
            x: new_x,
            y: new_y,
            z: self.z.clone(), // La componente z permanece inalterada.
        }
    }

    /// The rotation is performed on the y and z axes, leaving the x axis unchanged.
    pub fn rotate_x(&self, cos_theta: f32, sin_theta: f32) -> Self {
        let (new_y, new_z) = Aabb::rotate_axes(&self.y, &self.z, |y, z| {
            //   y' = cosθ*y - sinθ*z
            //   z' = sinθ*y + cosθ*z
            (cos_theta * y - sin_theta * z, sin_theta * y + cos_theta * z)
        });
        Self {
            x: self.x.clone(),
            y: new_y,
            z: new_z,
        }
    }

    fn pad_to_minimus(&mut self) {
        if self.x.size() < Self::DELTA {
            self.x.expand_inplace(Self::DELTA);
        }
        if self.y.size() < Self::DELTA {
            self.y.expand_inplace(Self::DELTA);
        }
        if self.z.size() < Self::DELTA {
            self.z.expand_inplace(Self::DELTA);
        }
    }

    fn rotate_axes<F>(first: &Interval, second: &Interval, f: F) -> (Interval, Interval)
    where
        F: Fn(f32, f32) -> (f32, f32),
    {
        let mut new_first_min = f32::INFINITY;
        let mut new_first_max = f32::NEG_INFINITY;
        let mut new_second_min = f32::INFINITY;
        let mut new_second_max = f32::NEG_INFINITY;

        for &a in &[first.min, first.max] {
            for &b in &[second.min, second.max] {
                let (a_new, b_new) = f(a, b);
                new_first_min = new_first_min.min(a_new);
                new_first_max = new_first_max.max(a_new);
                new_second_min = new_second_min.min(b_new);
                new_second_max = new_second_max.max(b_new);
            }
        }

        (
            Interval::new(new_first_min, new_first_max),
            Interval::new(new_second_min, new_second_max),
        )
    }
}

impl Add<&Vec3> for &Aabb {
    type Output = Aabb;

    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Aabb::from_intervals(&self.x + rhs.x(), &self.y + rhs.y(), &self.z + rhs.z())
    }
}

impl Add<&Aabb> for &Vec3 {
    type Output = Aabb;

    #[inline]
    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}
