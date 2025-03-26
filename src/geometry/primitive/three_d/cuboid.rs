use super::*;

pub struct Cuboid {
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Point3, p1: Point3, mat: Arc<dyn Material>) -> Self {
        let min = Point3::new(p0.x().min(p1.x()), p0.y().min(p1.y()), p0.z().min(p1.z()));
        let max = Point3::new(p0.x().max(p1.x()), p0.y().max(p1.y()), p0.z().max(p1.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let mut sides = HittableList::with_capacity(8);

        let front = Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx.clone(),
            dy.clone(),
            mat.clone(),
        ));

        let right = Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz.clone(),
            dy.clone(),
            mat.clone(),
        ));

        let back = Arc::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx.clone(),
            dy.clone(),
            mat.clone(),
        ));

        let left = Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz.clone(),
            dy,
            mat.clone(),
        ));

        let top = Arc::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx.clone(),
            -dz.clone(),
            mat.clone(),
        ));

        let bottom = Arc::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat,
        ));

        sides.push(front);
        sides.push(right);
        sides.push(back);
        sides.push(left);
        sides.push(top);
        sides.push(bottom);

        Self { sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.sides.hit(ray, ray_t)
    }

    fn bounding_box(&self) -> &Aabb {
        self.sides.bounding_box()
    }
}

impl Primitive for Cuboid {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.sides.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.sides.random(origin)
    }
}
