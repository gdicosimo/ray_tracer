use super::*;

use rayon::prelude::*;

// ─────────────────────────────
#[derive(Clone)]
pub struct LensParams {
    pub defocus_angle: f32,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl LensParams {
    pub fn new(defocus_angle: f32, defocus_disk_u: Vec3, defocus_disk_v: Vec3) -> Self {
        Self {
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
// ─────────────────────────────
#[derive(Clone)]
pub struct RenderParams {
    pub background: Color,
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_samples_scale: f32,
    pub recip_sqrt_spp: f32,
    pub max_depth: u16,
    pub sqrt_spp: u16,
}

impl RenderParams {
    pub fn new(
        background: Color,
        max_depth: u16,
        image_width: u32,
        image_height: u32,
        samples_per_pixel: u16,
    ) -> Self {
        let sqrt_spp = samples_per_pixel.isqrt();
        Self {
            background,
            max_depth,
            image_width,
            image_height,
            sqrt_spp,
            pixel_samples_scale: 1.0 / (sqrt_spp * sqrt_spp) as f32,
            recip_sqrt_spp: 1.0 / sqrt_spp as f32,
        }
    }
}
// ─────────────────────────────
#[derive(Clone)]
pub struct ViewportParams {
    pub center: Point3,
    pub pixel00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
}

impl ViewportParams {
    pub fn new(
        center: Point3,
        pixel00_loc: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    ) -> Self {
        Self {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
// ─────────────────────────────
pub struct Camera {
    pub(crate) viewport: ViewportParams,
    pub(crate) lens: LensParams,
    pub(crate) render: RenderParams,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable, lights: &dyn Primitive) -> Vec<f32> {
        let capacity = (self.render.image_width * self.render.image_height * 3) as usize;
        let mut pixels = Vec::with_capacity(capacity);

        let progress = Progress::new(self.render.image_height as usize);

        let rows: Vec<Vec<f32>> = (0..self.render.image_height)
            .into_par_iter()
            .map(|j| {
                progress.inc();

                let mut row_pixels = Vec::with_capacity(self.render.image_width as usize * 3);
                for i in 0..self.render.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for s_j in 0..self.render.sqrt_spp {
                        for s_i in 0..self.render.sqrt_spp {
                            let ray = self.get_ray(i, j, s_i, s_j);
                            pixel_color +=
                                self.ray_color(&ray, world, lights, self.render.max_depth);
                        }
                    }

                    let scaled_color = self.render.pixel_samples_scale * pixel_color;
                    row_pixels.extend_from_slice(&[
                        scaled_color.x(),
                        scaled_color.y(),
                        scaled_color.z(),
                    ]);
                }
                row_pixels
            })
            .collect();

        for row in rows {
            pixels.extend(row);
        }

        progress.finish();

        pixels
    }

    //Antialiasing
    fn get_ray(&self, i: u32, j: u32, s_i: u16, s_j: u16) -> Ray {
        let offset = sample_square_stratified(s_i, s_j, self.render.recip_sqrt_spp);
        let pixel_sample = &self.viewport.pixel00_loc
            + ((i as f32 + offset.x()) * &self.viewport.pixel_delta_u)
            + ((j as f32 + offset.y()) * &self.viewport.pixel_delta_v);

        let ray_origin = if self.lens.defocus_angle <= 0.0 {
            self.viewport.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction).set_time(math::random_float())
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let point = Point3::random_in_unit_disk();
        &self.viewport.center
            + (point.x() * &self.lens.defocus_disk_u)
            + (point.y() * &self.lens.defocus_disk_v)
    }

    fn ray_color(
        &self,
        ray: &Ray,
        world: &dyn Hittable,
        lights: &dyn Primitive,
        depth: u16,
    ) -> Color {
        if depth == 0 {
            return BLACK;
        }

        let rec = match world.hit(ray, Interval::CAMERA_VIEW) {
            Some(rec) => rec,
            None => return self.render.background.clone(),
        };

        let emitted = rec.material().emitted(&rec);

        let srec = match rec.material().scatter(ray, &rec) {
            Some(srec) => srec,
            None => return emitted,
        };

        let attenuation = srec.attenuation();

        if let Some(spec) = srec.specular() {
            return attenuation.mul(&self.ray_color(spec, world, lights, depth - 1));
        }

        let pdf = srec.pdf().unwrap();

        let light_pdf = pdf::Primitive::new(lights, rec.point());
        let mixture_pdf = pdf::Mixture::new(&light_pdf, pdf);

        let scattered_direction = mixture_pdf.generate();
        let scattered = Ray::new(rec.point().clone(), scattered_direction).set_time(ray.time());
        let pdf_val = mixture_pdf.value(scattered.direction().clone());

        let scattering_pdf = rec.material().scattering_pdf(ray, &rec, &scattered);

        let sample_color = self.ray_color(&scattered, world, lights, depth - 1);

        let color_from_scatter = (scattering_pdf / pdf_val) * sample_color.mul(srec.attenuation());

        emitted + color_from_scatter
    }

    pub fn width(&self) -> u32 {
        self.render.image_width
    }

    pub fn height(&self) -> u32 {
        self.render.image_height
    }
}

fn sample_square_stratified(s_i: u16, s_j: u16, recip_sqrt_spp: f32) -> Vec3 {
    let r1 = math::random_float();
    let r2 = math::random_float();

    let px = ((s_i as f32 + r1) * recip_sqrt_spp) - 0.5;
    let py = ((s_j as f32 + r2) * recip_sqrt_spp) - 0.5;

    Vec3::new(px, py, 0.0)
}

#[allow(dead_code)]
fn sample_square() -> Vec3 {
    Vec3::new(math::random_float() - 0.5, math::random_float() - 0.5, 0.0)
}
