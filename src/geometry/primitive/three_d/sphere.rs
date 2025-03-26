use super::*;

pub struct Sphere {
    center: Ray,
    material: Arc<dyn Material>,
    bbox: Aabb,
    radius: f32,
}

pub struct Hemisphere {
    sphere: Sphere,
    up: UnitVec3, // Define la orientación "hacia arriba" de la hemisfera.
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        let radius_vec = Vec3::splat(radius);
        let bbox = Aabb::from_points(&center - &radius_vec, &center + &radius_vec);

        Self {
            center: Ray::new(center, Vec3::ZERO),
            radius: radius.max(0.0),
            material,
            bbox,
        }
    }

    pub fn new_moving(
        start_center: Point3,
        end_center: Point3,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        let radius_vec = Vec3::splat(radius);
        let direction = &end_center - &start_center;

        let min_point = start_center.min(&end_center) - &radius_vec;
        let max_point = start_center.max(&end_center) + radius_vec;

        let bbox = Aabb::from_points(min_point, max_point);

        Self {
            center: Ray::new(start_center, direction),
            radius,
            material,
            bbox,
        }
    }

    pub fn center(&self) -> &Ray {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = &current_center - ray.origin();
        let a = ray.direction().len_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.len_squared() - (self.radius() * self.radius());
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let inv_a = 1.0 / a;
        let mut root = (h - sqrtd) * inv_a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) * inv_a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (&point - current_center).unchecked_into_unit_radius(self.radius());
        let uv = get_sphere_uv(&outward_normal);

        let hit_record =
            HitRecord::new(ray, point, outward_normal, root, self.material()).set_uv(uv);

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for Sphere {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        // This method only works for stationary spheres
        let ray = Ray::new(origin.clone(), direction.clone());

        if self.hit(&ray, Interval::CAMERA_VIEW).is_none() {
            return 0.0;
        }

        let cos_theta_max = (1.0
            - self.radius * self.radius / (self.center.origin() - origin).len_squared())
        .sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let center = self.center.origin();
        let direction = center - origin;
        let distance_squared = direction.len_squared();

        let w = direction.unchecked_normalize();
        let onb = Onb::from_unit_vec3(w.clone());
        let u = onb.u();
        let v = onb.v();

        let r1 = random_float();
        let r2 = random_float();

        let cos_theta_max = (1.0 - self.radius * self.radius / distance_squared).sqrt();
        let z = 1.0 + r2 * (cos_theta_max - 1.0);
        let phi = 2.0 * PI * r1;
        let sin_theta = (1.0 - z * z).sqrt();

        let x = phi.cos() * sin_theta;
        let y = phi.sin() * sin_theta;

        x * u + y * v + z * &w
    }
}

impl Hemisphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>, up: UnitVec3) -> Self {
        let sphere = Sphere::new(center, radius, material);
        Self { sphere, up }
    }
}

impl Hittable for Hemisphere {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        let rec = self.sphere.hit(ray, t_interval)?;

        if rec.normal().dot(&self.up) < 0.0 {
            return None;
        }
        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        self.sphere.bounding_box()
    }
}

impl Primitive for Hemisphere {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.sphere.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.sphere.random(origin)
    }
}

pub(super) fn get_sphere_uv(n: &UnitVec3) -> (f32, f32) {
    let theta = f32::acos(-n.y());
    let phi = f32::atan2(-n.z(), n.x()) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    (u, v)
}
