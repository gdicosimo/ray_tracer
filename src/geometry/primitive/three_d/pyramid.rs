use super::*;

pub struct Pyramid {
    faces: HittableList,
}

impl Pyramid {
    /// A pyramid with a rectangular base (5 faces total).
    ///
    /// The pyramid is constructed by:
    /// 1. Computing the four base vertices from two opposite corners (in the XZ plane).
    /// 2. Creating a single quad for the rectangular base.
    /// 3. Creating four lateral triangular faces that connect each edge of the base to the apex.
    ///
    /// # Parameters
    /// - `p0` and `p1`: Two opposite corners of the base rectangle.
    ///   Their x and z components define the base dimensions, while p0.y() sets the base height.
    /// - `height`: The height (in the Y direction) added to the base level to define the apex.
    /// - `mat`: The material to be applied to all faces of the pyramid.
    ///
    /// # Note
    /// The base is assumed to lie in the plane at y = p0.y(), and the apex is located at the center
    /// of the base with y-coordinate `p0.y() + height`.
    ///
    /// # Example
    /// ```rust
    /// let p0 = Point3::new(0.0, 0.0, 0.0);
    /// let p1 = Point3::new(100.0, 0.0, 100.0);
    /// let height = 50.0;
    /// let material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
    /// let pyramid = Pyramid::new(p0, p1, height, material);
    /// // Now `pyramid` is a Hittable object you can add to your scene.
    /// ```
    pub fn new(p0: Point3, p1: Point3, height: f32, mat: Arc<dyn Material>) -> Self {
        let min_x = p0.x().min(p1.x());
        let max_x = p0.x().max(p1.x());
        let min_z = p0.z().min(p1.z());
        let max_z = p0.z().max(p1.z());
        // The base lies at the y value of p0.
        let base_y = p0.y();

        let p0_base = Point3::new(min_x, base_y, min_z);
        let p1_base = Point3::new(max_x, base_y, min_z);
        let p2_base = Point3::new(max_x, base_y, max_z);
        let p3_base = Point3::new(min_x, base_y, max_z);

        let apex = Point3::new(
            (min_x + max_x) * 0.5,
            base_y + height,
            (min_z + max_z) * 0.5,
        );

        let mut faces = HittableList::new();

        // Base face: use a single Quad.
        // The quad is constructed with p0_base as origin,
        // with one edge along the X axis and the other along the Z axis.
        let u = &p1_base - &p0_base; // Along the X direction.
        let v = &p3_base - &p0_base; // Along the Z direction.
        faces.push(Arc::new(Quad::new(p0_base.clone(), u, v, mat.clone())));

        // Lateral faces: each is a Triangle formed by one edge of the base and the apex.
        faces.push(Arc::new(Triangle::new(
            p0_base.clone(),
            &p1_base - &p0_base,
            &apex - &p0_base,
            mat.clone(),
        )));
        faces.push(Arc::new(Triangle::new(
            p1_base.clone(),
            &p2_base - &p1_base,
            &apex - p1_base,
            mat.clone(),
        )));
        faces.push(Arc::new(Triangle::new(
            p2_base.clone(),
            &p3_base - &p2_base,
            &apex - p2_base,
            mat.clone(),
        )));
        faces.push(Arc::new(Triangle::new(
            p3_base.clone(),
            p0_base - &p3_base,
            apex - p3_base,
            mat,
        )));

        Self { faces }
    }
}

impl Hittable for Pyramid {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.faces.hit(ray, ray_t)
    }

    fn bounding_box(&self) -> &Aabb {
        self.faces.bounding_box()
    }
}

impl Primitive for Pyramid {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.faces.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.faces.random(origin)
    }
}
