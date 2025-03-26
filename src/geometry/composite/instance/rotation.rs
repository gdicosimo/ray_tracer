use super::*;

/// Core rotation type that encapsulates a rotated primitive and transformation functions.
struct Rotation {
    object: Arc<dyn Primitive>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Aabb,

    transform_point: fn(&Rotation, &Point3) -> Point3,
    transform_vector: fn(&Rotation, &Vec3) -> Vec3,

    inverse_transform_point: fn(&Rotation, &Point3) -> Point3,
    inverse_transform_vector: fn(&Rotation, &Vec3) -> Vec3,
}

pub struct RotationX(Rotation);
pub struct RotationY(Rotation);
pub struct RotationZ(Rotation);

impl Rotation {
    pub fn new(
        object: Arc<dyn Primitive>,
        angle: f32,
        bbox: Aabb,
        transform_point: fn(&Rotation, &Point3) -> Point3,
        transform_vector: fn(&Rotation, &Vec3) -> Vec3,
        inverse_transform_point: fn(&Rotation, &Point3) -> Point3,
        inverse_transform_vector: fn(&Rotation, &Vec3) -> Vec3,
    ) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        Rotation {
            object,
            sin_theta,
            cos_theta,
            bbox,
            transform_point,
            transform_vector,
            inverse_transform_point,
            inverse_transform_vector,
        }
    }
}

impl Hittable for Rotation {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        // Transform the ray into the rotated coordinate system.
        let rotated_origin = (self.transform_point)(self, ray.origin());
        let rotated_direction = (self.transform_vector)(self, ray.direction());
        let rotated_ray = Ray::new(rotated_origin, rotated_direction).set_time(ray.time());

        let mut rec = self.object.hit(&rotated_ray, t_interval)?;
        // Transform hit point back.
        rec.set_point((self.inverse_transform_point)(self, rec.point()));
        // Inverse-transform the normal.
        let inv_normal = (self.inverse_transform_vector)(self, &rec.normal().as_vec3());
        rec.set_normal(UnitVec3::unchecked_new(
            inv_normal.x(),
            inv_normal.y(),
            inv_normal.z(),
        ));

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for Rotation {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.object.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.object.random(origin)
    }
}

// ─────────────────────────────

impl RotationX {
    pub fn new(object: Arc<dyn Primitive>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box().rotate_x(cos_theta, sin_theta);
        let rotation = Rotation::new(
            object,
            angle,
            bbox,
            RotationX::transform_point,
            RotationX::transform_vector,
            RotationX::inverse_transform_point,
            RotationX::inverse_transform_vector,
        );
        Self(rotation)
    }

    fn transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Rotation around X: (x, y, z) -> (x, cosθ*y - sinθ*z, sinθ*y + cosθ*z)
        Point3::new(
            p.x(),
            r.cos_theta * p.y() - r.sin_theta * p.z(),
            r.sin_theta * p.y() + r.cos_theta * p.z(),
        )
    }

    fn transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            v.x(),
            r.cos_theta * v.y() - r.sin_theta * v.z(),
            r.sin_theta * v.y() + r.cos_theta * v.z(),
        )
    }

    fn inverse_transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Inverse rotation around X (rotate with -θ).
        Point3::new(
            p.x(),
            r.cos_theta * p.y() + r.sin_theta * p.z(),
            -r.sin_theta * p.y() + r.cos_theta * p.z(),
        )
    }

    fn inverse_transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            v.x(),
            r.cos_theta * v.y() + r.sin_theta * v.z(),
            -r.sin_theta * v.y() + r.cos_theta * v.z(),
        )
    }
}

impl Hittable for RotationX {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        self.0.hit(ray, t_interval)
    }

    fn bounding_box(&self) -> &Aabb {
        self.0.bounding_box()
    }
}

impl Primitive for RotationX {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.0.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.0.random(origin)
    }
}

// ─────────────────────────────

impl RotationY {
    pub fn new(object: Arc<dyn Primitive>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box().rotate_y(cos_theta, sin_theta);
        let rotation = Rotation::new(
            object,
            angle,
            bbox,
            RotationY::transform_point,
            RotationY::transform_vector,
            RotationY::inverse_transform_point,
            RotationY::inverse_transform_vector,
        );
        Self(rotation)
    }

    fn transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Rotation around Y: (x, y, z) -> ( cosθ*x - sinθ*z, y, sinθ*x + cosθ*z )
        Point3::new(
            r.cos_theta * p.x() - r.sin_theta * p.z(),
            p.y(),
            r.sin_theta * p.x() + r.cos_theta * p.z(),
        )
    }

    fn transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            r.cos_theta * v.x() - r.sin_theta * v.z(),
            v.y(),
            r.sin_theta * v.x() + r.cos_theta * v.z(),
        )
    }

    fn inverse_transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Inverse rotation around Y (rotate with -θ).
        Point3::new(
            r.cos_theta * p.x() + r.sin_theta * p.z(),
            p.y(),
            -r.sin_theta * p.x() + r.cos_theta * p.z(),
        )
    }

    fn inverse_transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            r.cos_theta * v.x() + r.sin_theta * v.z(),
            v.y(),
            -r.sin_theta * v.x() + r.cos_theta * v.z(),
        )
    }
}

impl Hittable for RotationY {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        self.0.hit(ray, t_interval)
    }
    fn bounding_box(&self) -> &Aabb {
        self.0.bounding_box()
    }
}

impl Primitive for RotationY {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.0.pdf_value(origin, direction)
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.0.random(origin)
    }
}

// ─────────────────────────────

impl RotationZ {
    pub fn new(object: Arc<dyn Primitive>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box().rotate_z(cos_theta, sin_theta);
        let rotation = Rotation::new(
            object,
            angle,
            bbox,
            RotationZ::transform_point,
            RotationZ::transform_vector,
            RotationZ::inverse_transform_point,
            RotationZ::inverse_transform_vector,
        );
        Self(rotation)
    }

    fn transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Rotation around Z: (x, y, z) -> (cosθ*x - sinθ*y, sinθ*x + cosθ*y, z)
        Point3::new(
            r.cos_theta * p.x() - r.sin_theta * p.y(),
            r.sin_theta * p.x() + r.cos_theta * p.y(),
            p.z(),
        )
    }

    fn transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            r.cos_theta * v.x() - r.sin_theta * v.y(),
            r.sin_theta * v.x() + r.cos_theta * v.y(),
            v.z(),
        )
    }

    fn inverse_transform_point(r: &Rotation, p: &Point3) -> Point3 {
        // Inverse rotation around Z (rotate with -θ).
        Point3::new(
            r.cos_theta * p.x() + r.sin_theta * p.y(),
            -r.sin_theta * p.x() + r.cos_theta * p.y(),
            p.z(),
        )
    }

    fn inverse_transform_vector(r: &Rotation, v: &Vec3) -> Vec3 {
        Vec3::new(
            r.cos_theta * v.x() + r.sin_theta * v.y(),
            -r.sin_theta * v.x() + r.cos_theta * v.y(),
            v.z(),
        )
    }
}

impl Hittable for RotationZ {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        self.0.hit(ray, t_interval)
    }
    fn bounding_box(&self) -> &Aabb {
        self.0.bounding_box()
    }
}

impl Primitive for RotationZ {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.0.pdf_value(origin, direction)
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        self.0.random(origin)
    }
}
