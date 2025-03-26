use super::*;

pub struct MirrorYZ {
    object: Arc<dyn Primitive>,
    bbox: Aabb,
}

impl MirrorYZ {
    pub fn new(object: Arc<dyn Primitive>) -> Self {
        let bbox = object.bounding_box().mirror_yz();
        Self { object, bbox }
    }

    fn transform_point(p: &Point3) -> Point3 {
        Point3::new(-p.x(), p.y(), p.z())
    }

    fn transform_vector(v: &Vec3) -> Vec3 {
        Vec3::new(-v.x(), v.y(), v.z())
    }
}

impl Hittable for MirrorYZ {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        let mirrored_origin = Self::transform_point(ray.origin());
        let mirrored_direction = Self::transform_vector(ray.direction());
        let mirrored_ray = Ray::new(mirrored_origin, mirrored_direction).set_time(ray.time());

        let mut rec = self.object.hit(&mirrored_ray, t_interval)?;

        let point_t = Self::transform_point(rec.point());
        let normal_t = Self::transform_vector(&rec.normal().as_vec3());
        let unit_normal = UnitVec3::new(normal_t);

        rec.front_face = !rec.front_face;
        rec.normal = unit_normal;
        rec.point = point_t;

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
impl Primitive for MirrorYZ {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        let mirrored_origin = Self::transform_point(origin);
        let mirrored_direction = Self::transform_vector(direction);
        self.object.pdf_value(&mirrored_origin, &mirrored_direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let mirrored_origin = Self::transform_point(origin);
        Self::transform_vector(&self.object.random(&mirrored_origin))
    }
}
