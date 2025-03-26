use std::cmp::Ordering;

use super::*;

pub enum Bvh {
    Empty,
    Node {
        bbox: Aabb,
        left: Box<Bvh>,
        right: Box<Bvh>,
    },
    Leaf(Arc<dyn Primitive>),
}

impl Bvh {
    pub fn build(list: HittableList) -> Self {
        if list.is_empty() {
            return Bvh::Empty;
        }

        let HittableList { mut objects, bbox } = list;

        Self::build_recursive(objects.as_mut_slice(), bbox)
    }

    // Recursively builds the BVH from a mutable slice of primitives and their bounding box.
    // The primitives are sorted along the longest axis of the provided bounding box.
    fn build_recursive(objects: &mut [Arc<dyn Primitive>], bbox: Aabb) -> Self {
        let object_span = objects.len();

        let axis = bbox.longest_axis();

        let comparator = |a: &Arc<dyn Primitive>, b: &Arc<dyn Primitive>| {
            let a_center = a.bounding_box().center()[axis];
            let b_center = b.bounding_box().center()[axis];
            a_center.partial_cmp(&b_center).unwrap_or(Ordering::Equal)
        };

        match object_span {
            0 => Bvh::Empty,
            1 => Bvh::Leaf(Arc::clone(&objects[0])),
            2 => {
                let left = Bvh::Leaf(Arc::clone(&objects[0]));
                let right = Bvh::Leaf(Arc::clone(&objects[1]));
                Bvh::Node {
                    bbox,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            _ => {
                objects.sort_unstable_by(comparator);
                let mid = object_span / 2;
                let (left_slice, right_slice) = objects.split_at_mut(mid);

                let left_bbox = calculate_bbox_slice(left_slice);
                let right_bbox = calculate_bbox_slice(right_slice);

                let left = Self::build_recursive(left_slice, left_bbox);
                let right = Self::build_recursive(right_slice, right_bbox);
                Bvh::Node {
                    bbox,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }
}

fn calculate_bbox_slice(objects: &[Arc<dyn Primitive>]) -> Aabb {
    objects
        .iter()
        .fold(None, |acc, obj| {
            let obj_box = obj.bounding_box();
            match acc {
                None => Some(obj_box.clone()),
                Some(current_bbox) => Some(current_bbox.merge(obj_box)),
            }
        })
        .unwrap_or(Aabb::EMPTY)
}

impl Hittable for Bvh {
    /// For internal nodes, the method first checks if the ray intersects the node's bounding box.
    /// Then it recursively tests the left and right children, updating the closest hit.
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Self::Node { left, right, .. } => {
                if let Some(bbox_interval) = self.bounding_box().hit(ray, ray_t.clone()) {
                    let mut closest_hit: Option<HitRecord> = None;
                    let mut closest_so_far = bbox_interval.max;

                    if let Some(left_rec) = left.hit(ray, bbox_interval) {
                        closest_so_far = left_rec.t();
                        closest_hit = Some(left_rec);
                    }

                    let right_ray_t = Interval::new(ray_t.min(), closest_so_far);
                    if let Some(right_rec) = right.hit(ray, right_ray_t) {
                        if closest_hit.is_none()
                            || right_rec.t() < closest_hit.as_ref().unwrap().t()
                        {
                            closest_hit = Some(right_rec);
                        }
                    }
                    closest_hit
                } else {
                    None
                }
            }
            Self::Leaf(primitive) => primitive.hit(ray, ray_t),
            Self::Empty => None,
        }
    }

    fn bounding_box(&self) -> &Aabb {
        match self {
            Self::Node { bbox, .. } => bbox,
            Self::Leaf(primitive) => primitive.bounding_box(),
            Self::Empty => &Aabb::EMPTY,
        }
    }
}
