use super::*;

pub struct Trapezoid {
    quad: Quad,
    ratio: f32,
}

impl Trapezoid {
    /// Creates a new trapezoid in the canonical coordinate system.
    ///
    /// The trapezoid is built on an underlying Quad defined by:
    ///
    ///     p(α, β) = q + α * u + β * v,   with α, β ∈ [0, 1]
    ///
    /// In this canonical system:
    /// - `q` is the base origin (typically the lower-left corner),
    /// - `u` is the vector along the bottom edge (e.g., in the x-direction),
    /// - `v` is the vector along the height (e.g., in the y-direction).
    ///
    /// The trapezoidal region is obtained by restricting the allowed α values:
    ///
    ///     α ∈ [α_min(β), α_max(β)]
    ///
    /// where:
    ///
    ///     α_min(β) = β * 0.5 * (1.0 - ratio)
    ///     α_max(β) = 1.0 - β * 0.5 * (1.0 - ratio)
    ///
    /// Thus:
    /// - When `ratio == 1.0`, we have α_min(β)=0 and α_max(β)=1 for all β (the full quad).
    /// - When `ratio == 0.0`, for β=1 we get α_min(1)=0.5 and α_max(1)=0.5, so the top edge collapses to the center (resulting in a triangle).
    /// - Intermediate values yield a trapezoid with the top base scaled accordingly.
    ///
    pub fn new(q: Point3, u: Vec3, v: Vec3, ratio: f32, mat: Arc<dyn Material>) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        Self {
            quad: Quad::new(q, u, v, mat),
            ratio,
        }
    }
}

impl Hittable for Trapezoid {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitRecord> {
        self.quad.hit_with(ray, t_interval, |alpha, beta| {
            if !(0.0..=1.0).contains(&beta) {
                return false;
            }
            let alpha_min = beta * 0.5 * (1.0 - self.ratio);
            let alpha_max = 1.0 - beta * 0.5 * (1.0 - self.ratio);
            alpha >= alpha_min && alpha <= alpha_max
        })
    }

    fn bounding_box(&self) -> &Aabb {
        self.quad.bounding_box()
    }
}

impl Primitive for Trapezoid {
    /// The trapezoid area is estimated as the area of the underlying quad scaled by
    /// ((1 + ratio) / 2). Thus, the PDF is adjusted by a factor of:
    ///
    ///     quad_area / trap_area = 2 / (1 + ratio)
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f32 {
        let quad_area = self.quad.area();
        let trap_area = quad_area * ((1.0 + self.ratio) / 2.0);
        self.quad.pdf_value(origin, direction) * (quad_area / trap_area)
    }

    /// Generates a random direction from `origin` toward a uniformly sampled point on the trapezoid.
    ///
    /// It samples β uniformly in [0, 1] and, for each β, samples α uniformly in
    /// [β * 0.5 * (1.0 - ratio), 1.0 - β * 0.5 * (1.0 - ratio)].
    fn random(&self, origin: &Point3) -> Vec3 {
        let q = self.quad.q();
        let u = self.quad.u();
        let v = self.quad.v();

        let beta = random_float(); // Must return f32 in [0, 1]
        let alpha_min = beta * 0.5 * (1.0 - self.ratio);
        let alpha_max = 1.0 - beta * 0.5 * (1.0 - self.ratio);
        let alpha = alpha_min + (alpha_max - alpha_min) * random_float();

        let point = q + alpha * u + beta * v;
        (point - origin).unchecked_into_unit().into_vec3()
    }
}
