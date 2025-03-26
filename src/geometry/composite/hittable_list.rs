use super::*;
use crate::collections::ArrayList;

static DEFAULT_CAPACITY: usize = 4;

pub struct HittableList {
    pub(super) objects: ArrayList<Arc<dyn Primitive>>,
    pub(super) bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: ArrayList::new(),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: ArrayList::with_capacity(capacity),
            bbox: Aabb::EMPTY,
        }
    }

    #[inline]
    pub fn from_object(object: Arc<dyn Primitive>) -> Self {
        let mut hittable_list = Self::with_capacity(DEFAULT_CAPACITY);
        hittable_list.push(object.clone());
        hittable_list
    }

    pub fn from_objects(objects: ArrayList<Arc<dyn Primitive>>) -> Self {
        let mut hlist = Self::with_capacity(objects.len());

        for object in objects.iter() {
            hlist.push(object.clone());
        }

        hlist
    }

    pub fn push(&mut self, object: Arc<dyn Primitive>) {
        self.objects.push(object.clone());
        self.bbox.merge_inplace(object.bounding_box());
    }

    pub fn objects(&self) -> &ArrayList<Arc<dyn Primitive>> {
        &self.objects
    }

    pub fn into_objects(self) -> ArrayList<Arc<dyn Primitive>> {
        self.objects
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;

        let mut temp_rec = None;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = record.t();
                temp_rec = Some(record);
            }
        }

        temp_rec
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Primitive for HittableList {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        if self.objects.is_empty() {
            return 0.0;
        }

        let weight = 1.0 / (self.objects.len() as f32);
        self.objects
            .iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        if self.objects.is_empty() {
            return UnitVec3::I.as_vec3();
        }

        let size = self.objects.len() as f32;
        self.objects
            .get(random_int_beetwen(0.0, size - 1.0))
            .unwrap()
            .random(origin)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
