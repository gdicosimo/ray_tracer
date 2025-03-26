use super::*;

#[derive(Clone)]
pub struct HitRecord {
    pub(super) point: Point3,
    pub(super) normal: UnitVec3,
    pub(super) material: Arc<dyn Material>,
    t: f32,
    uv: Option<(f32, f32)>,
    pub(super) front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        point: Point3,
        outward_normal: UnitVec3,
        t: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        let (front_face, normal) = get_face_normal(ray, outward_normal);

        Self {
            point,
            normal,
            material,
            t,
            front_face,
            uv: None,
        }
    }

    pub fn point(&self) -> &Point3 {
        &self.point
    }

    pub fn into_point(self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> &UnitVec3 {
        &self.normal
    }

    pub fn into_normal(self) -> UnitVec3 {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn u(&self) -> f32 {
        match self.uv {
            Some((u, _)) => u,
            None => 0.0,
        }
    }

    pub fn v(&self) -> f32 {
        match self.uv {
            Some((_, v)) => v,
            None => 0.0,
        }
    }

    pub fn set_uv(mut self, uv: (f32, f32)) -> Self {
        self.uv = Some(uv);
        self
    }

    pub fn set_point(&mut self, point: Point3) {
        self.point = point;
    }

    pub fn set_normal(&mut self, n: UnitVec3) {
        self.normal = n;
    }

    pub fn negate_normal(&mut self) {
        self.normal = -&self.normal;
    }

    pub fn distance(&self) -> f32 {
        self.t
    }

    pub fn get_normal(&self) -> &UnitVec3 {
        &self.normal
    }

    pub fn get_point(&self) -> &Point3 {
        &self.point
    }

    pub fn get_uv(&self) -> (f32, f32) {
        self.uv.unwrap_or((0.0, 0.0))
    }

    pub(crate) fn new_arbitrary(point: Point3, t: f32, material: Arc<dyn Material>) -> Self {
        let normal = UnitVec3::I;
        let front_face = true;

        Self {
            point,
            normal,
            material,
            t,
            front_face,
            uv: None,
        }
    }
}

fn get_face_normal(ray: &Ray, outward_normal: UnitVec3) -> (bool, UnitVec3) {
    let front_face = ray.direction().dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
