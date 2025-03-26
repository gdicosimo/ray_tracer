use super::*;

pub struct Triangle(Quad);

impl Triangle {
    /// Creates a new `Triangle` given a base point `q`, edge vectors `u` and `v`, and a material.
    ///
    /// It is constructed from a base point `q` and two edge vectors `u` and `v`,
    /// forming the vertices:
    /// - `q`
    /// - `q + u`
    /// - `q + v`
    ///
    /// The underlying `Quad` represents the entire parallelogram, and the triangle
    /// is obtained by restricting to the region where the barycentric coordinates satisfy:
    /// `alpha > 0`, `beta > 0` and `alpha + beta < 1`.
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        Self(Quad::new(q, u, v, mat))
    }
}

impl Hittable for Triangle {
    /// This method delegates the intersection test to the underlying `Quad`'s `hit_with` method,
    /// filtering the results so that only intersections with barycentric coordinates that satisfy:
    /// `alpha > 0`, `beta > 0`, and `alpha + beta < 1` are accepted.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.0.hit_with(ray, ray_t, |alpha, beta| {
            alpha > 0.0 && beta > 0.0 && (alpha + beta) < 1.0
        })
    }

    /// This is directly derived from the underlying `Quad`'s bounding box.
    fn bounding_box(&self) -> &Aabb {
        self.0.bounding_box()
    }
}

impl Primitive for Triangle {
    /// Since the area of the triangle is exactly half that of the underlying quad, the PDF value
    /// computed by the quad is adjusted by a factor of 2.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.0.pdf_value(origin, direction) * 2.0
    }

    /// This implementation uses the standard technique for uniformly sampling a triangle:
    /// 1. Generate two random numbers `r1` and `r2` in [0, 1].
    /// 2. If `r1 + r2 > 1`, remap them as `(1 - r1, 1 - r2)` to ensure uniform distribution.
    /// 3. Compute the point on the triangle as: `p + u * alpha + v * beta`.
    /// 4. Return the normalized direction from the `origin` to this point.
    fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.0.q();
        let u = self.0.u();
        let v = self.0.v();

        let r1: f32 = random_float();
        let r2: f32 = random_float();

        let (alpha, beta) = if r1 + r2 > 1.0 {
            (1.0 - r1, 1.0 - r2)
        } else {
            (r1, r2)
        };

        let point_on_triangle = p + alpha * u + beta * v;
        (point_on_triangle - origin)
            .unchecked_normalize()
            .into_vec3()
    }
}
