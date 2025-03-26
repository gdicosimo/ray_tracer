use super::*;
use crate::textures::SolidColor;

pub struct ConstantMedium {
    boundary: Arc<dyn Primitive>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn from_texture(
        boundary: Arc<dyn Primitive>,
        density: f32,
        texture: Arc<dyn Texture>,
    ) -> Self {
        let phase_function = Arc::new(Isotropic::from_texture(texture));
        let neg_inv_density = -1.0 / density;
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density,
        }
    }

    pub fn from_color(boundary: Arc<dyn Primitive>, density: f32, color: Color) -> Self {
        let texture = Arc::new(SolidColor::from_color(color));
        Self::from_texture(boundary, density, texture)
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let record1 = self.boundary.hit(ray, Interval::UNIVERSE)?;

        let record2 = self
            .boundary
            .hit(ray, Interval::new(record1.t() + 0.0001, f32::INFINITY))?;

        let t1 = ray_t.min().max(record1.t());
        let t2 = ray_t.max().min(record2.t());

        if t1 >= t2 {
            return None;
        }

        let t1 = t1.max(0.0);

        let ray_length = ray.direction().norm();
        let distance_inside_boundary = (t2 - t1) * ray_length;
        let hit_distance = self.neg_inv_density * random_float().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t_hit = t1 + hit_distance / ray_length;
        let p = ray.at(t_hit);

        let rec = HitRecord::new_arbitrary(p, t_hit, self.phase_function.clone());

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}

impl Primitive for ConstantMedium {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        self.boundary.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        self.boundary.random(origin)
    }
}
