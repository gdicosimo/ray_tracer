use super::*;

pub struct Translation {
    object: Arc<dyn Primitive>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translation {
    pub fn new(object: Arc<dyn Primitive>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + &offset;

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray =
            Ray::new(ray.origin() - &self.offset, ray.direction().clone()).set_time(ray.time());

        let mut rec = self.object.hit(&offset_ray, ray_t)?;

        rec.set_point(rec.point().translate(&self.offset));
        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for Translation {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.object.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.object.random(origin)
    }
}
