use super::*;

const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);

pub struct Quad {
    bbox: Aabb,
    mat: Arc<dyn Material>,
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: UnitVec3,
    d: f32,
    area: f32,
}

//Ax+By+Cz=D, n = (A, B, C)
//n = (u x v)
impl Quad {
    pub fn q(&self) -> &Point3 {
        &self.q
    }

    pub fn u(&self) -> &Vec3 {
        &self.u
    }

    pub fn v(&self) -> &Vec3 {
        &self.v
    }

    pub fn area(&self) -> f32 {
        self.area
    }

    #[inline]
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unchecked_normalize();
        let d = normal.dot(&q.to_vec3());
        let norm_squared = n.dot(&n);
        let w = norm_squared / n;

        let area = norm_squared.sqrt();

        let quad = Self {
            q,
            u,
            v,
            w,
            mat,
            normal,
            d,
            area,
            bbox: Aabb::EMPTY,
        };

        quad.set_bounding_box()
    }

    pub(crate) fn hit_with<F>(
        &self,
        ray: &Ray,
        ray_t: Interval,
        is_interior: F,
    ) -> Option<HitRecord>
    where
        F: Fn(f32, f32) -> bool,
    {
        let denom = Vec3::dot(ray.direction(), &self.normal);

        if denom.abs() < f32::EPSILON {
            return None;
        }

        let t = (self.d - Vec3::dot(&ray.origin().to_vec3(), &self.normal)) / denom;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt: Vec3 = &intersection - &self.q;

        let alpha = planar_hitpt.cross(&self.v).dot(&self.w);
        let beta = self.u.cross(&planar_hitpt).dot(&self.w);

        if !is_interior(alpha, beta) {
            return None;
        }

        Some(
            HitRecord::new(ray, intersection, self.normal.clone(), t, self.mat.clone())
                .set_uv((alpha, beta)),
        )
    }

    fn set_bounding_box(mut self) -> Self {
        let u = &self.u;
        let v = &self.v;
        let q = &self.q;

        let bbox_diagonal1 = Aabb::from_points(q.clone(), q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
        self.bbox = Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2);
        self
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.hit_with(ray, ray_t, |alpha, beta| {
            UNIT_INTERVAL.contains(alpha) && UNIT_INTERVAL.contains(beta)
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for Quad {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        let rec = match self.hit(
            &Ray::new(origin.clone(), direction.clone()),
            Interval::CAMERA_VIEW,
        ) {
            Some(rec) => rec,
            None => return 0.0,
        };

        let norm_squared = direction.len_squared();
        let distance_squared = (rec.get_point() - origin).len_squared();
        let cosine = f32::abs(direction.dot(rec.get_normal()) / norm_squared.sqrt());

        if cosine.is_nan() || cosine.is_infinite() {
            println!("aca hay otro");
        }

        distance_squared / (self.area * cosine)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let r1 = random_float();
        let r2 = random_float();
        let p = &self.q + (r1 * &self.u) + (r2 * &self.v);

        p - origin
    }
}
