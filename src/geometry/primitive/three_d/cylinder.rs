use super::*;

pub struct Cylinder {
    radius: f32,
    height: f32,
    bbox: Aabb,
    center: Point3,
    material: Arc<dyn Material>,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f32, height: f32, material: Arc<dyn Material>) -> Self {
        assert!(radius > 0.0, "Radius must be positive");
        assert!(height > 0.0, "Height must be positive");

        let half_height = height * 0.5;
        let min = &center - Vec3::new(radius, half_height, radius);
        let max = &center + Vec3::new(radius, half_height, radius);
        let bbox = Aabb::from_points(min, max);

        Self {
            radius,
            height,
            bbox,
            center,
            material,
        }
    }

    fn intersect_caps(&self, ray: &Ray, cap_y: f32, ray_t: &Interval) -> Option<HitRecord> {
        let dir_y = ray.direction().y();
        if dir_y.abs() < EPSILON {
            return None;
        }

        let t = (cap_y - ray.origin().y()) / dir_y;
        if !ray_t.contains(t) {
            return None;
        }

        let hit_point = ray.at(t);
        let delta = &hit_point - &self.center;
        if delta.x().powi(2) + delta.z().powi(2) > self.radius.powi(2) {
            return None;
        }

        let normal = if cap_y < self.center.y() {
            -UnitVec3::J
        } else {
            UnitVec3::J
        };

        Some(HitRecord::new(
            ray,
            hit_point,
            normal,
            t,
            self.material.clone(),
        ))
    }

    fn intersect_lateral(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let dir = ray.direction();

        let a = dir.x().powi(2) + dir.z().powi(2);
        if a < EPSILON {
            return None;
        }

        let b = 2.0 * (oc.x() * dir.x() + oc.z() * dir.z());
        let c = oc.x().powi(2) + oc.z().powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-b - sqrt_d) / (2.0 * a);
        if !ray_t.contains(t) {
            t = (-b + sqrt_d) / (2.0 * a);
            if !ray_t.contains(t) {
                return None;
            }
        }

        let hit_point = ray.at(t);
        let cap_bottom = self.center.y() - self.height * 0.5;
        let cap_top = self.center.y() + self.height * 0.5;
        if hit_point.y() < cap_bottom || hit_point.y() > cap_top {
            return None;
        }

        let normal = Vec3::new(
            hit_point.x() - self.center.x(),
            0.0,
            hit_point.z() - self.center.z(),
        )
        .unchecked_into_unit_vector();
        Some(HitRecord::new(
            ray,
            hit_point,
            normal,
            t,
            self.material.clone(),
        ))
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let cap_bottom = self.center.y() - self.height * 0.5;
        let cap_top = self.center.y() + self.height * 0.5;

        let mut closest_hit = self
            .intersect_caps(ray, cap_bottom, &ray_t)
            .or_else(|| self.intersect_caps(ray, cap_top, &ray_t));

        if let Some(hit) = self.intersect_lateral(ray, &ray_t) {
            closest_hit = Some(hit);
        }
        closest_hit
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for Cylinder {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        let ray = Ray::new(origin.clone(), direction.clone());
        if let Some(rec) = self.hit(&ray, Interval::new(0.001, f32::INFINITY)) {
            let distance_squared = rec.t().powi(2) * direction.len_squared();
            let cosine = direction.dot(rec.normal()).abs() / direction.norm();

            let lateral_area = 2.0 * PI * self.radius * self.height;
            let cap_area = PI * self.radius.powi(2);
            let total_area = 2.0 * cap_area + lateral_area;

            distance_squared / (cosine * total_area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let cap_area = PI * self.radius.powi(2);
        let lateral_area = 2.0 * PI * self.radius * self.height;
        let total_area = 2.0 * cap_area + lateral_area;
        let r_area = random_float() * total_area;

        let sampled_point = if r_area < cap_area {
            let theta = 2.0 * PI * random_float();
            let r = self.radius * random_float().sqrt();
            Point3::new(
                self.center.x() + r * theta.cos(),
                self.center.y() - self.height * 0.5,
                self.center.z() + r * theta.sin(),
            )
        } else if r_area < 2.0 * cap_area {
            let theta = 2.0 * PI * random_float();
            let r = self.radius * random_float().sqrt();
            Point3::new(
                self.center.x() + r * theta.cos(),
                self.center.y() + self.height * 0.5,
                self.center.z() + r * theta.sin(),
            )
        } else {
            let theta = 2.0 * PI * random_float();
            let y = self.center.y() - self.height * 0.5 + random_float() * self.height;
            Point3::new(
                self.center.x() + self.radius * theta.cos(),
                y,
                self.center.z() + self.radius * theta.sin(),
            )
        };

        (sampled_point - origin)
            .unchecked_into_unit_vector()
            .into_vec3()
    }
}
