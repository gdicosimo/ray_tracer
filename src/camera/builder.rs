use super::*;

// ─────────────────────────────

const DEFAULT_DEFOCUS_ANGLE: f32 = 0.0;
const DEFAULT_FOCUS_DIST: f32 = 10.0;
const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
const DEFAULT_VFOV: f32 = 90.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 2.0;
const DEFAULT_IMAGE_WIDTH: u32 = 100;
const DEFAULT_SAMPLES_PER_PIXEL: u16 = 10;
const DEFAULT_MAX_DEPTH: u16 = 10;
const DEFAULT_LOOK_FROM: Point3 = Point3::from_origin();
const DEFAULT_LOOK_AT: Point3 = Point3::new(0.0, 0.0, -1.0);
const DEFAULT_VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const DEFAULT_BACKGROUND: Color = BLACK;

// ─────────────────────────────

#[derive(Default)]
struct ViewportBuilderParams {
    look_at: Option<Point3>,
    look_from: Option<Point3>,
    vup: Option<Vec3>,
    vfov: Option<f32>,
    viewport_height: Option<f32>,
}

impl ViewportBuilderParams {
    fn build(self) -> (Point3, Point3, Vec3, f32, f32) {
        (
            self.look_from.unwrap_or(DEFAULT_LOOK_FROM),
            self.look_at.unwrap_or(DEFAULT_LOOK_AT),
            self.vup.unwrap_or(DEFAULT_VUP),
            self.vfov.unwrap_or(DEFAULT_VFOV),
            self.viewport_height.unwrap_or(DEFAULT_VIEWPORT_HEIGHT),
        )
    }
}

// ─────────────────────────────

#[derive(Default)]

struct LensBuilderParams {
    defocus_angle: Option<f32>,
    focus_dist: Option<f32>,
}

impl LensBuilderParams {
    fn build(self) -> (f32, f32) {
        (
            self.focus_dist.unwrap_or(DEFAULT_FOCUS_DIST),
            self.defocus_angle.unwrap_or(DEFAULT_DEFOCUS_ANGLE),
        )
    }
}

// ─────────────────────────────

#[derive(Default)]
struct RenderBuilderParams {
    background: Option<Color>,
    aspect_ratio: Option<f32>,
    image_width: Option<u32>,
    samples_per_pixel: Option<u16>,
    max_depth: Option<u16>,
}

impl RenderBuilderParams {
    fn build(self) -> (Color, f32, u32, u16, u16) {
        (
            self.background.unwrap_or(DEFAULT_BACKGROUND),
            self.aspect_ratio.unwrap_or(DEFAULT_ASPECT_RATIO),
            self.image_width.unwrap_or(DEFAULT_IMAGE_WIDTH),
            self.samples_per_pixel.unwrap_or(DEFAULT_SAMPLES_PER_PIXEL),
            self.max_depth.unwrap_or(DEFAULT_MAX_DEPTH),
        )
    }
}

// ─────────────────────────────

#[derive(Default)]
pub struct CameraBuilder {
    viewport: ViewportBuilderParams,
    lens: LensBuilderParams,
    render: RenderBuilderParams,
}

impl CameraBuilder {
    ///Alias for default()
    pub fn uninitialized() -> Self {
        Self::default()
    }

    pub fn look_at(mut self, look: Point3) -> Self {
        self.viewport.look_at = Some(look);
        self
    }

    pub fn look_from(mut self, look: Point3) -> Self {
        self.viewport.look_from = Some(look);
        self
    }

    pub fn vup(mut self, direction: Vec3) -> Self {
        self.viewport.vup = Some(direction);
        self
    }

    pub fn vfov(mut self, vfov: f32) -> Self {
        self.viewport.vfov = Some(vfov);
        self
    }

    pub fn viewport_height(mut self, height: f32) -> Self {
        self.viewport.viewport_height = Some(height);
        self
    }

    pub fn defocus_angle(mut self, angle: f32) -> Self {
        self.lens.defocus_angle = Some(angle);
        self
    }
    pub fn focus_dist(mut self, dist: f32) -> Self {
        self.lens.focus_dist = Some(dist);
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.render.background = Some(background);
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.render.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.render.image_width = Some(image_width);
        self
    }

    pub fn samples_per_pixel(mut self, samples: u16) -> Self {
        self.render.samples_per_pixel = Some(samples);
        self
    }

    pub fn max_depth(mut self, depth: u16) -> Self {
        self.render.max_depth = Some(depth);
        self
    }

    #[inline(always)]
    pub fn build(self) -> Camera {
        let (look_from, look_at, vup, vfov, viewport_height_factor) = self.viewport.build();
        let (focus_dist, defocus_angle) = self.lens.build();
        let (background, aspect_ratio, image_width, samples_per_pixel, max_depth) =
            self.render.build();

        let image_width_f = image_width as f32;
        let image_height_f = (image_width_f / aspect_ratio).max(1.0);
        let image_height = image_height_f.round() as u32;

        let theta = math::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let effective_viewport_height = viewport_height_factor * h * focus_dist;
        let viewport_width = effective_viewport_height * (image_width_f / image_height_f);

        let w = (&look_from - look_at).unchecked_into_unit_vector();
        let u = vup.cross(&w).unchecked_into_unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * &u;
        let viewport_v = -effective_viewport_height * &v;

        let pixel_delta_u = image_width_f / &viewport_u;
        let pixel_delta_v = image_height_f / &viewport_v;

        let viewport_center = &look_from - focus_dist * w;
        let viewport_upper_left = viewport_center - 2.0 / viewport_u - 2.0 / viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        let defocus_radius = focus_dist * math::degrees_to_radians(defocus_angle / 2.0);
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera {
            viewport: ViewportParams::new(look_from, pixel00_loc, pixel_delta_u, pixel_delta_v),
            lens: LensParams::new(defocus_angle, defocus_disk_u, defocus_disk_v),
            render: RenderParams::new(
                background,
                max_depth,
                image_width,
                image_height,
                samples_per_pixel,
            ),
        }
    }
}
