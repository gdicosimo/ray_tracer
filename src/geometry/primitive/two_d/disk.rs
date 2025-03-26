use super::*;

pub struct Disk(Quad);

impl Disk {
    /// Creates a new `Disk` given a base point `q`, edge vectors `u` and `v`, and a material.
    ///
    /// # Parameters
    /// - `q`: The lower-left corner of the quad in world space.
    /// - `u`: The edge vector corresponding to the horizontal direction.
    /// - `v`: The edge vector corresponding to the vertical direction.
    ///
    /// The underlying quad spans the square from `q` to `q + u + v`, and the disk is defined
    /// as the subset of that square where the parameter-space distance from (0.5,0.5) is less than 0.5.
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        Self(Quad::new(q, u, v, mat))
    }
}

impl Hittable for Disk {
    /// The method delegates the intersection test to the underlying `Quad` and then applies a filter
    /// that accepts only intersections with (α,β) coordinates that satisfy the disk condition:
    /// the distance from (0.5, 0.5) is less than 0.5.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Use hit_with on the underlying quad with a custom predicate.
        self.0.hit_with(ray, ray_t, |alpha, beta| {
            let x = alpha - 0.5;
            let y = beta - 0.5;

            (x * x + y * y).sqrt() < 0.5
        })
    }

    fn bounding_box(&self) -> &Aabb {
        self.0.bounding_box()
    }
}

impl Primitive for Disk {
    /// The underlying quad is parameterized over an area of 1 (in parameter space), but the disk's area
    /// is (π/4) (since its normalized radius is 0.5). Thus, the PDF value for the disk is scaled by 4/π
    /// relative to the quad's PDF.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        if self
            .hit(
                &Ray::new(origin.clone(), direction.clone()),
                Interval::CAMERA_VIEW,
            )
            .is_some()
        {
            self.0.pdf_value(origin, direction) * (4.0 / PI)
        } else {
            0.0
        }
    }

    /// The method samples polar coordinates in the unit disk (in parameter space) and maps these to
    /// the underlying quad. The disk is inscribed in the square, so the center is at (0.5,0.5) and the
    /// radius is 0.5 in parameter space.
    fn random(&self, origin: &Point3) -> Vec3 {
        let r1: f32 = random_float();
        let r2: f32 = random_float();

        let r = r1.sqrt();
        let theta = 2.0 * PI * r2;

        let offset_alpha = 0.5 * r * theta.cos();
        let offset_beta = 0.5 * r * theta.sin();

        let alpha = 0.5 + offset_alpha;
        let beta = 0.5 + offset_beta;

        let p = self.0.q();
        let u = self.0.u();
        let v = self.0.v();
        let point_on_disk = p + alpha * u + beta * v;

        (point_on_disk - origin).unchecked_normalize().into_vec3()
    }
}
